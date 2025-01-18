#!/bin/bash

# Colors
BLUE='\033[0;34m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Output functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_process() { echo -e "${CYAN}[PROCESS]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }

# URL encode function
urlencode() {
    local string="$1"
    local strlen=${#string}
    local encoded=""
    for (( pos=0 ; pos<strlen ; pos++ )); do
        c=${string:$pos:1}
        case "$c" in
            [-_.~a-zA-Z0-9] ) encoded+="${c}" ;;
            * ) encoded+=$(printf '%%%02x' "'$c") ;;
        esac
    done
    echo "${encoded}"
}

# Clean text function
clean_text() {
    echo "$1" | \
    sed 's/<[^>]*>//g' | \
    sed 's/\[[0-9]*\]//g' | \
    sed 's/<!--.*-->//g' | \
    sed 's/&quot;/'"'"'/g' | \
    sed 's/&nbsp;/ /g' | \
    sed 's/&ndash;/-/g' | \
    sed 's/&mdash;/-/g' | \
    sed 's/&amp;/\&/g' | \
    sed 's/"/'"'"'/g' | \
    tr '\n' ' ' | \
    sed 's/  */ /g' | \
    sed 's/^ *//g' | \
    sed 's/ *$//g'
}

# Extract meaningful tags from text
extract_tags() {
    local TITLE="$1"
    local CONTENT="$2"
    local COMBINED_TEXT="${TITLE,,} ${CONTENT,,}"  # Combine and lowercase

    # Common categories to check
    local CATEGORIES=(
        "blog" "news" "article" "post" "update" "guide" "tutorial"
        "review" "opinion" "analysis" "interview" "feature" "announcement"
        "product" "service" "event" "release" "update" "case-study"
    )

    # Initialize tags array
    local TAGS=()

    # Add words from title that are at least 3 characters
    for word in $TITLE; do
        word="${word,,}"  # lowercase
        word="${word//[^a-z]/}"  # remove non-letters
        if [ ${#word} -ge 3 ]; then
            TAGS+=("$word")
        fi
    done

    # Add matching categories
    for category in "${CATEGORIES[@]}"; do
        if echo "$COMBINED_TEXT" | grep -iq "\b${category}\b"; then
            TAGS+=("$category")
        fi
    done

    # Remove duplicates and format
    printf '%s\n' "${TAGS[@]}" | sort -u | tr '\n' ',' | sed 's/,$//' | sed 's/,/, /g'
}

# Filter and limit tags
filter_tags() {
    local TAGS="$1"
    local FILTERED=""

    # Common words to exclude
    local EXCLUDE=(
        "the" "and" "but" "or" "nor" "for" "yet" "so"
        "a" "an" "to" "in" "on" "at" "by" "up" "off" "it" "its" "with"
        "from" "into" "onto" "upon" "out" "about" "over" "this" "that"
        "these" "those" "what" "which" "who" "whom" "whose" "when" "where"
        "why" "how" "all" "any" "both" "each" "few" "more" "most" "other"
        "some" "such" "than" "too" "very" "can" "will" "just" "should"
        "now" "one" "also" "however" "therefore" "hence" "thus"
    )

    # Convert string of tags to array and process each
    local IFS=', '
    read -ra TAG_ARRAY <<< "$TAGS"

    # Filter out excluded words and short tags
    local VALID_TAGS=()
    for tag in "${TAG_ARRAY[@]}"; do
        tag=$(echo "$tag" | tr -d '"' | tr '[:upper:]' '[:lower:]' | tr -d ' ')

        # Skip if tag is in exclude list or too short
        local EXCLUDED=false
        for exclude in "${EXCLUDE[@]}"; do
            if [ "$tag" = "$exclude" ]; then
                EXCLUDED=true
                break
            fi
        done

        if [ ${#tag} -ge 3 ] && ! $EXCLUDED; then
            VALID_TAGS+=("$tag")
        fi
    done

    # Take up to 8 unique tags
    printf '%s\n' "${VALID_TAGS[@]}" | sort -u | head -n 8 | tr '\n' ',' | sed 's/,$//'
}

# Get external domains from content
get_external_domains() {
    local CONTENT="$1"
    echo "$CONTENT" | \
    grep -oE 'https?://[^/"]+' | \
    sed 's/^https\?:\/\///' | \
    grep -E '\.[a-zA-Z]{2,}$' | \
    sort -u | \
    tr '\n' ',' | \
    sed 's/,$//'
}

# Format domains for TOML
format_domains() {
    local DOMAINS="$1"
    if [ -z "$DOMAINS" ]; then
        echo "\"none\""
    else
        echo "[\"${DOMAINS//,/\",\"}\"]"
    fi
}

# Check for input
if [ -z "$1" ]; then
    log_error "No WordPress site URL provided"
    echo "Usage: $0 <wordpress_site_url>"
    exit 1
fi

# Check for required commands
for cmd in curl jq; do
    if ! command -v $cmd &> /dev/null; then
        log_error "$cmd is required but not installed."
        exit 1
    fi
done

# Set variables
SITE_URL="${1%/}"  # Remove trailing slash if present
OUTPUT_FILE="wordpress_posts.toml"
API_ENDPOINT="/wp-json/wp/v2/posts?per_page=100"

# Main process
log_info "Preparing to fetch posts from: '$SITE_URL'"

# Create API URL
API_URL="${SITE_URL}${API_ENDPOINT}"
log_process "Fetching posts list..."
log_info "URL: $API_URL"

# Get response with proper headers
RESPONSE=$(curl -s \
    -H "Accept: application/json" \
    -A "WordPressTOMLScraper/1.0" \
    "$API_URL")

# Check for valid JSON
if ! echo "$RESPONSE" | jq . >/dev/null 2>&1; then
    log_error "Invalid JSON response from API"
    log_error "Response: $RESPONSE"
    exit 1
fi

# Get result count
RESULTS_COUNT=$(echo "$RESPONSE" | jq '. | length')

log_success "Found $RESULTS_COUNT posts"
log_process "Generating TOML file: $OUTPUT_FILE"

# Clear output file
> "$OUTPUT_FILE"

# Process each post
CURRENT=0
echo "$RESPONSE" | jq -r '.[] | @base64' | while read -r post; do
    CURRENT=$((CURRENT + 1))

    # Decode post data
    POST_DATA=$(echo "$post" | base64 --decode)

    TITLE=$(echo "$POST_DATA" | jq -r '.title.rendered')
    CONTENT=$(echo "$POST_DATA" | jq -r '.content.rendered')
    EXCERPT=$(echo "$POST_DATA" | jq -r '.excerpt.rendered')
    URL=$(echo "$POST_DATA" | jq -r '.link')
    DATE=$(echo "$POST_DATA" | jq -r '.date')

    # Show progress with title
    printf "\r\033[K${CYAN}[PROCESS]${NC} [$CURRENT/$RESULTS_COUNT] Processing: $TITLE"

    # Clean the data
    CLEAN_TITLE=$(clean_text "$TITLE")
    CLEAN_CONTENT=$(clean_text "$CONTENT")
    CLEAN_EXCERPT=$(clean_text "$EXCERPT")

    # Extract and format tags
    CONTENT_TAGS=$(extract_tags "$CLEAN_TITLE" "$CLEAN_CONTENT")
    FILTERED_TAGS=$(filter_tags "$CONTENT_TAGS")

    if [ -n "$FILTERED_TAGS" ]; then
        ALL_TAGS="[\"${FILTERED_TAGS//,/\",\"}\"]"
    else
        ALL_TAGS="[]"
    fi

    # Get and format external domains
    EXTERNAL_DOMAINS=$(get_external_domains "$CLEAN_CONTENT")
    FORMATTED_DOMAINS=$(format_domains "$EXTERNAL_DOMAINS")

    # Write output
    {
        echo "[\"$URL\"]"
        echo "title = \"$CLEAN_TITLE\""
        echo "description = \"$CLEAN_EXCERPT\""
        echo "full_description = \"$CLEAN_CONTENT\""
        echo "clearnet = \"$URL\""
        echo "status = \"Operational\""
        echo "category = \"wordpress\""
        echo "date = \"$DATE\""
        echo "tags = $ALL_TAGS"
        echo "external-domains = $FORMATTED_DOMAINS"
        echo ""
    } >> "$OUTPUT_FILE"
done

echo # New line after progress
log_success "Generated TOML file: $OUTPUT_FILE"

# Show file info
FILE_SIZE=$(ls -lh "$OUTPUT_FILE" | awk '{print $5}')
log_success "File size: $FILE_SIZE"
log_success "Processed $RESULTS_COUNT posts"
