#!/bin/bash
# Run culling toggle test for 10 seconds

echo "Running culling toggle test..."
cargo run --bin test_culling_toggle 2>&1 | grep -E "(toggle|Culling|culling|spawn_all|Spawned|DISABLED|ENABLED|AUTO)" &
PID=$!

# Wait 10 seconds
sleep 10

# Kill the process
kill $PID 2>/dev/null

echo "Test completed."