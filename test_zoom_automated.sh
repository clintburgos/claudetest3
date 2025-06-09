#!/bin/bash

echo "Starting zoom test..."

# Start the game in background
cargo run --bin claudetest3 &
PID=$!

# Wait for game to start
sleep 5

# Use osascript to simulate clicking "Start Game" button
# This assumes the button is roughly in the center of the screen
osascript -e 'tell application "System Events" to click at {640, 400}'

# Wait for game to load
sleep 3

# Now simulate pressing 'e' key multiple times to zoom out
# Each press zooms out a bit
for i in {1..20}; do
    echo "Zooming out - iteration $i"
    osascript -e 'tell application "System Events" to key code 14' # 'e' key
    sleep 0.5
done

# Wait a bit more to capture final state
sleep 3

# Kill the game
kill $PID 2>/dev/null || true

echo "Test complete. Check the screenshots in the latest log directory."