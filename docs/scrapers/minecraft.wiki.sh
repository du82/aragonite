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

# Extract page content
get_page_content() {
    local URL="$1"
    curl -s -A "MinecraftWikiScraper/1.0" "$URL"
}

# Extract page title
get_page_title() {
    local CONTENT="$1"
    echo "$CONTENT" | grep -o '<span class="mw-page-title-main">[^<]*</span>' | \
    sed 's/<[^>]*>//g' | head -n 1 | clean_text
}

# Extract description (first few paragraphs)
get_description() {
    local CONTENT="$1"
    echo "$CONTENT" | \
    awk '/<div class="mw-parser-output">/{p=1;next} /<\/div>/{if(p){p=0;exit}} p' | \
    sed 's/<[^>]*>//g' | \
    grep -v '^[[:space:]]*$' | \
    grep -v "^This article is about" | \
    grep -v "^Jump to:" | \
    head -n 3 | \
    clean_text
}

# Extract categories
get_categories() {
    local CONTENT="$1"
    echo "$CONTENT" | \
    grep -o '<div id="mw-normal-catlinks".*</div>' | \
    grep -o 'href="/w/Category:[^"]*' | \
    sed 's/href="\/w\/Category://' | \
    tr '\n' ',' | sed 's/,$//'
}

# Extract tags
extract_tags() {
    local CONTENT="$1"
    local CATEGORIES=(
        "gameplay" "blocks" "biomes" "items" "mobs" "mechanics" "world_generation"
        "commands" "redstone" "technical" "multiplayer" "servers" "resources" "updates"
        "guides" "tutorials" "structures" "dimensions" "features" "combat" "tools"
    )

    local TAGS=()
    for category in "${CATEGORIES[@]}"; do
        if echo "$CONTENT" | grep -iq "\b${category}\b"; then
            TAGS+=("$category")
        fi
    done

    printf '%s\n' "${TAGS[@]}" | sort -u | head -n 8 | tr '\n' ',' | sed 's/,$//'
}

# Extract external domains from links
extract_domains() {
    local CONTENT="$1"
    local TEMP_FILE=$(mktemp)

    # Extract external links
    echo "$CONTENT" | grep -o 'href="https\?://[^"]*"' | \
    sed 's/href="//;s/"$//' | grep -v 'minecraft.wiki' > "$TEMP_FILE"

    # Process URLs to get domains
    local DOMAINS_FILE=$(mktemp)
    while read -r url; do
        [ -z "$url" ] && continue

        # Extract domain (remove protocol and everything after first slash)
        domain=$(echo "$url" | sed 's|^https\?://||' | sed 's|/.*$||' | sed 's/^www\.//')

        # Validate domain format
        if [ -n "$domain" ] && echo "$domain" | grep -q "\.[a-zA-Z0-9-]\{2,\}$"; then
            echo "$domain" >> "$DOMAINS_FILE"
        fi
    done < "$TEMP_FILE"

    # Get top 10 most frequent domains
    RESULT=$(sort "$DOMAINS_FILE" | uniq -c | sort -rn | head -n 10 | awk '{print $2}' | tr '\n' ',' | sed 's/,$//')

    # Cleanup
    rm -f "$TEMP_FILE" "$DOMAINS_FILE"

    echo "$RESULT"
}

# Extract related pages
extract_related_pages() {
    local CONTENT="$1"
    local TEMP_FILE=$(mktemp)

    # Extract links from "See also" section and navigation
    echo "$CONTENT" | \
    grep -o 'href="/w/[^"]*"' | \
    sed 's/href="//;s/"$//' > "$TEMP_FILE"

    # Process URLs
    local PROCESSED_FILE=$(mktemp)
    while read -r url; do
        [ -z "$url" ] && continue

        # Skip non-article pages and special namespaces
        if [[ "$url" =~ ^/w/ ]] && \
           ! [[ "$url" =~ ^/w/(File|Category|Template|Special): ]] && \
           ! [[ "$url" =~ \.(jpg|jpeg|png|gif|svg)$ ]]; then
            echo "https://minecraft.wiki$url" >> "$PROCESSED_FILE"
        fi
    done < "$TEMP_FILE"

    # Get unique URLs
    sort -u "$PROCESSED_FILE"

    # Cleanup
    rm -f "$TEMP_FILE" "$PROCESSED_FILE"
}

# Process a single URL
process_url() {
    local URL="$1"

    log_process "Processing: $URL"

    # Fetch page content
    local CONTENT=$(get_page_content "$URL")

    if [ -z "$CONTENT" ]; then
        log_warning "Empty content from $URL"
        return 1
    fi

    # Extract information
    local TITLE=$(get_page_title "$CONTENT")
    local DESCRIPTION=$(get_description "$CONTENT")
    local CATEGORIES=$(get_categories "$CONTENT")
    local TAGS=$(extract_tags "$CONTENT")
    local DOMAINS=$(extract_domains "$CONTENT")

    # Clean text
    local CLEAN_TITLE=$(clean_text "$TITLE")
    local CLEAN_DESCRIPTION=$(clean_text "$DESCRIPTION")

    # Write output
    {
        echo "[\"$URL\"]"
        echo "title = \"$CLEAN_TITLE - Minecraft Wiki\""
        echo "description = \"$CLEAN_DESCRIPTION\""
        echo "categories = [\"$(echo $CATEGORIES | sed 's/,/\", \"/g')\"]"
        echo "tags = [\"$(echo $TAGS | sed 's/,/\", \"/g')\"]"
        [ -n "$DOMAINS" ] && echo "external-references = [\"$(echo $DOMAINS | sed 's/,/\", \"/g')\"]" \
                         || echo "external-references = \"none\""
        echo "url = \"$URL\""
        echo ""
    } >> "$OUTPUT_FILE"

    # Find and queue new URLs
    while read -r new_url; do
        if [ -n "$new_url" ] && [ -z "${VISITED_URLS[$new_url]}" ]; then
            log_info "Found new URL: $new_url"
            URL_QUEUE+=("$new_url")
        fi
    done < <(extract_related_pages "$CONTENT" | head -n "$BATCH_SIZE")

    return 0
}

# Main script
if [ -z "$1" ]; then
    log_error "No starting URL provided"
    echo "Usage: $0 <minecraft_wiki_url>"
    exit 1
fi

# Check for curl
if ! command -v curl &> /dev/null; then
    log_error "curl is required but not installed."
    exit 1
fi

# Set variables
START_URL="$1"
OUTPUT_FILE="minecraft_wiki_content.toml"
LIMIT=500000
SLEEP_TIME=2
BATCH_SIZE=15

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
