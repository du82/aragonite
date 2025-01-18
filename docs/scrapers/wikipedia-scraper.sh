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

# Get full article description
get_full_description() {
    local PAGEID=$1
    local EXTRACT_URL="https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=0&explaintext=1&exlimit=1&pageids=${PAGEID}&format=json"
    local RESPONSE=$(curl -s \
        -H "Accept: application/json" \
        -H "User-Agent: WikipediaTOMLScraper/1.0" \
        "$EXTRACT_URL")
    echo "$RESPONSE" | jq -r ".query.pages.\"${PAGEID}\".extract" | head -n 10 | \
    sed 's/== .* ==//g' | \
    sed 's/=== .* ===//g' | \
    clean_text
}

# Filter and limit tags
filter_tags() {
    local TAGS="$1"
    local FILTERED=""

    # Common words to exclude
    local EXCLUDE=(
        "wiki" "wikipedia" "the" "and" "but" "or" "nor" "for" "yet" "so"
        "a" "an" "to" "in" "on" "at" "by" "up" "off" "it" "its" "with"
        "from" "into" "onto" "upon" "out" "about" "over" "this" "that"
        "these" "those" "what" "which" "who" "whom" "whose" "when" "where"
        "why" "how" "all" "any" "both" "each" "few" "more" "most" "other"
        "some" "such" "than" "too" "very" "can" "will" "just" "should"
        "now" "one" "also" "however" "therefore" "hence" "thus" "article"
        "page" "list" "category" "categories" "articles" "pages" "stub" "related"
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
    printf '%s\n' "${VALID_TAGS[@]}" | sort -u | head -n 8 | tr '\n' ',' | sed 's/,$//' | sed 's/,/, /g'
}

# Extract meaningful tags from text
extract_tags() {
    local TITLE="$1"
    local CONTENT="$2"
    local COMBINED_TEXT="${TITLE,,} ${CONTENT,,}"  # Combine and lowercase

    # Common categories to check
    local CATEGORIES=(
        "history" "science" "technology" "politics" "art" "music"
        "literature" "philosophy" "religion" "sports" "entertainment"
        "medicine" "biology" "physics" "chemistry" "mathematics"
        "geography" "architecture" "culture" "education" "military"
        "economics" "business" "industry" "nature" "animal" "plant"
        "space" "engineering" "computing" "internet" "media" "food"
        "language" "law" "government" "society" "transportation"
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

    # Add contextual tags
    if echo "$COMBINED_TEXT" | grep -iq "\byear\b\|\bcentury\b\|\bperiod\b\|\bera\b"; then
        TAGS+=("historical")
    fi
    if echo "$COMBINED_TEXT" | grep -iq "\bspecies\b\|\bgenus\b\|\bfamily\b"; then
        TAGS+=("taxonomy")
    fi
    if echo "$COMBINED_TEXT" | grep -iq "\binvent\b\|\bdiscove\b\|\binnovat\b"; then
        TAGS+=("innovation")
    fi

    # Remove duplicates and format
    printf '%s\n' "${TAGS[@]}" | sort -u | tr '\n' ',' | sed 's/,$//' | sed 's/,/, /g'
}

# Get category tags from Wikipedia
get_category_tags() {
    local PAGEID=$1
    local CAT_URL="https://en.wikipedia.org/w/api.php?action=query&prop=categories&pageids=${PAGEID}&format=json&cllimit=10"
    local RESPONSE=$(curl -s \
        -H "Accept: application/json" \
        -H "User-Agent: WikipediaTOMLScraper/1.0" \
        "$CAT_URL")

    if [ -n "$RESPONSE" ]; then
        echo "$RESPONSE" | \
        jq -r ".query.pages.\"${PAGEID}\".categories[]?.title" 2>/dev/null | \
        sed 's/Category://' | \
        tr '[:upper:]' '[:lower:]' | \
        grep -v "articles\|pages\|wikipedia\|wikidata" | \
        grep -oE '[a-z]+' | \
        sort -u | \
        tr '\n' ',' | \
        sed 's/,$//' | \
        sed 's/,/, /g'
    fi
}

# Check for input
if [ -z "$1" ]; then
    log_error "No search term provided"
    echo "Usage: $0 <search_term>"
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
SEARCH_TERM="$1"
OUTPUT_FILE="${SEARCH_TERM// /_}.toml"
LIMIT=50

# Main process
log_info "Preparing to search for: '$SEARCH_TERM'"

# Create API URL for search
ENCODED_TERM=$(urlencode "$SEARCH_TERM")
SEARCH_URL="https://en.wikipedia.org/w/api.php?action=query&format=json&list=search&srwhat=text&srsearch=${ENCODED_TERM}&srlimit=${LIMIT}&utf8=1"

log_process "Fetching article list..."
log_info "URL: $SEARCH_URL"

# Get response with proper headers
RESPONSE=$(curl -s \
    -H "Accept: application/json" \
    -A "WikipediaTOMLScraper/1.0" \
    "$SEARCH_URL")

# Debug output
log_info "Raw response size: $(echo "$RESPONSE" | wc -c) bytes"
log_info "First 500 chars of response: ${RESPONSE:0:500}"

# Check for valid JSON
if ! echo "$RESPONSE" | jq . >/dev/null 2>&1; then
    log_error "Invalid JSON response from API"
    log_error "Response: $RESPONSE"
    exit 1
fi

# Get result counts
TOTAL_RESULTS=$(echo "$RESPONSE" | jq -r '.query.searchinfo.totalhits')
RESULTS_COUNT=$(echo "$RESPONSE" | jq -r '.query.search | length')

log_success "Found $TOTAL_RESULTS total matches (processing $RESULTS_COUNT)"
log_process "Generating TOML file: $OUTPUT_FILE"

# Clear output file
> "$OUTPUT_FILE"

# Process each result
CURRENT=0
echo "$RESPONSE" | jq -r '.query.search[] | @base64' | while read -r page; do
    CURRENT=$((CURRENT + 1))

    # Decode page data
    PAGE_DATA=$(echo "$page" | base64 --decode)
    TITLE=$(echo "$PAGE_DATA" | jq -r '.title')
    PAGEID=$(echo "$PAGE_DATA" | jq -r '.pageid')
    SNIPPET=$(echo "$PAGE_DATA" | jq -r '.snippet')

    # Show progress with title
    printf "\r\033[K${CYAN}[PROCESS]${NC} [$CURRENT/$RESULTS_COUNT] Processing: $TITLE"

    # Get and clean the data
    CLEAN_TITLE=$(clean_text "$TITLE")
    CLEAN_SNIPPET=$(clean_text "$SNIPPET")
    URL="https://en.wikipedia.org/wiki/$(urlencode "$TITLE")"

    # Get full description
    FULL_DESC=$(get_full_description "$PAGEID")
    CLEAN_FULL_DESC=$(clean_text "$FULL_DESC")

    # Extract tags
    CONTENT_TAGS=$(extract_tags "$CLEAN_TITLE" "$CLEAN_FULL_DESC")
    CATEGORY_TAGS=$(get_category_tags "$PAGEID")

    # Combine and filter tags
    COMBINED_TAGS="$CONTENT_TAGS, $CATEGORY_TAGS"
    FILTERED_TAGS=$(filter_tags "$COMBINED_TAGS")

    # Format tags for TOML
    if [ -n "$FILTERED_TAGS" ]; then
        ALL_TAGS="[\"${FILTERED_TAGS//,/\", \"}\"]"
    else
        ALL_TAGS="[]"
    fi

    # Write output
    {
        echo "[\"$URL\"]"
        echo "title = \"$CLEAN_TITLE - Wikipedia\""
        echo "description = \"$CLEAN_SNIPPET\""
        echo "full_description = \"$CLEAN_FULL_DESC\""
        echo "clearnet = \"$URL\""
        echo "status = \"Operational\""
        echo "category = \"wikipedia\""
        echo "tags = $ALL_TAGS"
        echo ""
    } >> "$OUTPUT_FILE"
done

echo # New line after progress
log_success "Generated TOML file: $OUTPUT_FILE"

# Show file info
FILE_SIZE=$(ls -lh "$OUTPUT_FILE" | awk '{print $5}')
log_success "File size: $FILE_SIZE"
log_success "Processed $RESULTS_COUNT articles"
