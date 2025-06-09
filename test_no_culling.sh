#!/bin/bash
# Run no_culling_test for a few seconds and capture output

echo "Starting no_culling_test..."
cargo run --bin no_culling_test &
PID=$!

# Wait 3 seconds
sleep 3

# Kill the process
kill $PID 2>/dev/null

echo "Test completed."