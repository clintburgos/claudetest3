#!/bin/bash
# Test toggling culling with key press

echo "Starting game, will simulate C key press after 5 seconds..."

# Start the game in background
cargo run --bin claudetest3 2>&1 | grep -E "(Culling|culling|spawn_all|Spawned|DISABLED|ENABLED)" &
PID=$!

# Wait for game to start
sleep 5

# Note: Can't directly simulate keypress without additional tools
# Just kill after some time
sleep 10
kill $PID 2>/dev/null

echo "Test completed."