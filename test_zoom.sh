#!/bin/bash
# Test script for camera zoom functionality

echo "Testing camera zoom functionality..."
echo "Press Q to zoom in, E to zoom out"
echo "Press 1 for min zoom, 2 for max zoom"
echo "Check the logs to verify zoom behavior"
echo ""

cargo run --example test_camera_fixes 2>&1 | grep -E "(Camera:|Zoom clamped:|Set to|Camera limits)" &

# Give user time to interact
echo "Running for 30 seconds. Test the zoom controls..."
sleep 30

# Kill the background process
kill $!

echo "Test complete!"