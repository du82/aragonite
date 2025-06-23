#!/bin/bash

set -e

# Navigate to the project directory
cd "$(dirname "$0")"

# Check if server is running and get its PID
SERVER_PID=$(pgrep -f "target/debug/aragonite" || echo "")

# Pull the latest changes from GitHub
echo -e "\n[1/4] Pulling latest changes from GitHub..."
git pull

# Stop the server if it's running
if [ -n "$SERVER_PID" ]; then
    echo -e "\n[2/4] Stopping the server (PID: $SERVER_PID)..."
    kill -SIGINT $SERVER_PID

    # Wait for server to shut down gracefully
    echo "Waiting for server to shut down..."
    for i in {1..10}; do
        if ! ps -p $SERVER_PID > /dev/null; then
            break
        fi
        sleep 1
    done

    # Force kill if still running
    if ps -p $SERVER_PID > /dev/null; then
        echo "Server still running after 10 seconds, forcing termination..."
        kill -9 $SERVER_PID
    fi

    echo "Server stopped"
else
    echo -e "\n[2/4] No server currently running"
fi

# Clean and rebuild the project
echo -e "\n[3/4] Cleaning and rebuilding the project..."
cargo clean
cargo build

# Run the server
echo -e "\n[4/4] Starting Aragonite server..."
cargo run
