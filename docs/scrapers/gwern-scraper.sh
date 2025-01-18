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
    sed 's/^[[:space:]]*//g' | \
    sed 's/[[:space:]]*$//g' | \
    sed 's/[[:space:]]\+/ /g' | \
    tr '\n' ' ' | \
    sed 's/"//g'  # Remove quotes entirely
}

# Extract frontmatter from markdown
get_frontmatter() {
    local MD_CONTENT="$1"
    echo "$MD_CONTENT" | awk '/^---/{p=1;next}/^---/{p=0;exit}p'
}

# Get value from frontmatter
get_frontmatter_value() {
    local FRONTMATTER="$1"
    local KEY="$2"
    echo "$FRONTMATTER" | grep "^$KEY:" | sed "s/^$KEY:[[:space:]]*//g" | sed 's/^[[:space:]]*//g' | sed 's/[[:space:]]*$//g'
}

# Get content from Gwern.net
fetch_gwern_content() {
    local URL="$1"
    curl -s -A "GwernScraper/1.0" "$URL"
}

# Extract first few paragraphs of content
get_description() {
    local CONTENT="$1"
    echo "$CONTENT" | \
    awk '/^---/{p=1;next}/^---/{p=0;next}!p' | \
    sed '/^#/d' | \
    sed '/^>/d' | \
    sed '/^[[:space:]]*$/d' | \
    head -n 5 | \
    clean_text
}

# Extract tags from content
extract_tags() {
    local TITLE="$1"
    local CONTENT="$2"
    local CATEGORIES=(
        "research" "analysis" "science" "technology" "ai" "machine-learning"
        "psychology" "philosophy" "literature" "statistics" "programming"
        "mathematics" "economics" "rationality" "cognitive-science" "biology"
        "genetics" "optimization" "intelligence" "education" "consciousness"
        "computation" "culture" "data" "experiment" "methodology" "neuroscience"
        "productivity" "review" "security" "systems" "writing" "bayesian"
        "essays" "notes" "papers" "blog" "technical" "history" "meta"
    )

    local TAGS=()
    for category in "${CATEGORIES[@]}"; do
        if echo "$CONTENT $TITLE" | grep -iq "\b${category}\b"; then
            TAGS+=("$category")
        fi
    done

    printf '%s\n' "${TAGS[@]}" | sort -u | head -n 8 | tr '\n' ',' | sed 's/,$//'
}

# Find new URLs in content
find_urls() {
    local CONTENT="$1"
    local MAX_URLS="$2"
    local TEMP_FILE=$(mktemp)

    # Extract markdown-style links
    echo "$CONTENT" | grep -o '\[.*\]([^)]*)' | sed 's/.*(\(.*\))/\1/g' | sed 's/ .*$//' > "$TEMP_FILE"

    # Add plain URLs
    echo "$CONTENT" | grep -o 'https\?://gwern.net/[^ )"\047>]*' >> "$TEMP_FILE"

    # Add relative URLs
    echo "$CONTENT" | grep -o '\](/[^)]*' | sed 's/\]\(//' | sed 's/ .*$//' >> "$TEMP_FILE"

    # Process the collected URLs
    local FILTERED_FILE=$(mktemp)
    while read -r url; do
        [ -z "$url" ] && continue

        # Convert relative URLs to absolute
        if [[ "$url" == /* ]]; then
            url="https://gwern.net$url"
        fi

        # Only process gwern.net URLs
        if [[ "$url" != https://gwern.net/* ]]; then
            continue
        fi

        # Skip media files and unwanted content
        if [[ "$url" =~ \.(jpg|jpeg|png|gif|mp4|webm|js|css|zip|pdf|doc|docx)$ ]] || \
           [[ "$url" =~ /static/ ]] || \
           [[ "$url" =~ /images/ ]] || \
           [[ "$url" =~ /doc/ ]] || \
           [[ "$url" =~ /css/ ]] || \
           [[ "$url" =~ /js/ ]]; then
            continue
        fi

        # Clean up URL and save
        echo "$url" | sed 's/#.*//g' | sed 's/?.*//g' | sed 's/\/$//' | sed 's/ .*$//' >> "$FILTERED_FILE"
    done < "$TEMP_FILE"

    # Output unique URLs up to the limit
    sort -u "$FILTERED_FILE" | head -n "$MAX_URLS"

    # Cleanup
    rm -f "$TEMP_FILE" "$FILTERED_FILE"
}

# Process a single URL
process_url() {
    local URL="$1"
    local MD_URL="$URL.md"

    log_process "Processing: $URL"

    # Fetch markdown content
    log_process "Fetching: $MD_URL"
    local CONTENT=$(fetch_gwern_content "$MD_URL")

    if [ -z "$CONTENT" ]; then
        log_warning "Empty content from $MD_URL"
        return 1
    fi

    # Extract frontmatter
    local FRONTMATTER=$(get_frontmatter "$CONTENT")
    local TITLE=$(get_frontmatter_value "$FRONTMATTER" "title")
    local DESCRIPTION=$(get_frontmatter_value "$FRONTMATTER" "description")

    if [ -z "$TITLE" ]; then
        TITLE=$(basename "$URL")
    fi

    if [ -z "$DESCRIPTION" ]; then
        DESCRIPTION=$(get_description "$CONTENT")
    fi

    # Clean text
    local CLEAN_TITLE=$(clean_text "$TITLE")
    local CLEAN_DESCRIPTION=$(clean_text "$DESCRIPTION")

    # Extract tags
    local TAGS=$(extract_tags "$CLEAN_TITLE" "$CONTENT")

    # Write output
    {
        echo "[\"$URL\"]"
        echo "title = \"$CLEAN_TITLE - Gwern.net\""
        echo "description = \"$CLEAN_DESCRIPTION\""
        echo "clearnet = \"$URL\""
        echo "status = \"Operational\""
        echo "category = \"research\""
        echo "tags = [\"$(echo $TAGS | sed 's/,/\", \"/g')\"]"
        echo ""
    } >> "$OUTPUT_FILE"

    # Find and queue new URLs
    while read -r new_url; do
        if [ -n "$new_url" ] && [ -z "${VISITED_URLS[$new_url]}" ]; then
            log_info "Found new URL: $new_url"
            URL_QUEUE+=("$new_url")
        fi
    done < <(find_urls "$CONTENT" 5)

    return 0
}

# Main script

if [ -z "$1" ]; then
    log_error "No starting URL provided"
    echo "Usage: $0 <gwern_url>"
    exit 1
fi

# Check for curl
if ! command -v curl &> /dev/null; then
    log_error "curl is required but not installed."
    exit 1
fi

# Set variables
START_URL="$1"
OUTPUT_FILE="gwern_content.toml"
LIMIT=500
SLEEP_TIME=2
BATCH_SIZE=5

# Initialize arrays and counters
declare -a URL_QUEUE=("$START_URL")
declare -A VISITED_URLS
PROCESSED_COUNT=0
BATCH_COUNT=0

# Clear output file
> "$OUTPUT_FILE"

log_info "Starting crawl from: '$START_URL'"
log_process "Generating TOML file: $OUTPUT_FILE"

# Process URLs in batches
while [ ${#URL_QUEUE[@]} -gt 0 ] && [ $PROCESSED_COUNT -lt $LIMIT ]; do
    # Process current batch
    BATCH_COUNT=0
    while [ $BATCH_COUNT -lt $BATCH_SIZE ] && [ ${#URL_QUEUE[@]} -gt 0 ] && [ $PROCESSED_COUNT -lt $LIMIT ]; do
        CURRENT_URL="${URL_QUEUE[0]}"
        URL_QUEUE=("${URL_QUEUE[@]:1}")

        if [ -z "${VISITED_URLS[$CURRENT_URL]}" ]; then
            log_info "Processing URL $PROCESSED_COUNT/$LIMIT: $CURRENT_URL"
            if process_url "$CURRENT_URL"; then
                VISITED_URLS[$CURRENT_URL]="1"
                PROCESSED_COUNT=$((PROCESSED_COUNT + 1))
                BATCH_COUNT=$((BATCH_COUNT + 1))
                sleep $SLEEP_TIME
            fi
        fi
    done

    log_info "Completed batch of $BATCH_COUNT URLs"
    log_info "Queue size: ${#URL_QUEUE[@]}"
    log_info "Total processed: $PROCESSED_COUNT"
done

log_success "Generated TOML file: $OUTPUT_FILE"
log_success "File size: $(ls -lh "$OUTPUT_FILE" | awk '{print $5}')"
log_success "Processed $PROCESSED_COUNT pages"
log_success "Total unique URLs found: ${#VISITED_URLS[@]}"
