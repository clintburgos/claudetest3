#\!/bin/bash
# Script to test zoom debugging with keyboard simulation

echo "Starting debug_visible_tiles..."
cargo run --bin debug_visible_tiles > zoom_debug_full.log 2>&1 &
DEBUG_PID=$\!

echo "Waiting for initialization..."
sleep 3

echo "Testing zoom out with osascript keyboard events..."
# Focus on the Bevy window first (assuming it's named "Debug Visible Tiles")
osascript -e 'tell application "System Events" to tell process "debug_visible_tiles" to set frontmost to true' 2>/dev/null

# Press D to get initial debug output
sleep 1
osascript -e 'tell application "System Events" to key code 2' # D key

# Now hold E key to zoom out
sleep 2
echo "Zooming out (holding E)..."
osascript -e 'tell application "System Events" to key down "e"'
sleep 3
osascript -e 'tell application "System Events" to key up "e"'

# Press D again to see zoom changes
sleep 1
osascript -e 'tell application "System Events" to key code 2' # D key

# Zoom out more
sleep 1
echo "Zooming out more..."
osascript -e 'tell application "System Events" to key down "e"'
sleep 3
osascript -e 'tell application "System Events" to key up "e"'

# Final debug output
sleep 1
osascript -e 'tell application "System Events" to key code 2' # D key

sleep 2

echo -e "\n=== Zoom Debug Results ==="
grep -E "(Camera:.*zoom|visible_tiles=|Actual rendered|Expected grid|edge tile.*on screen:|Missing)" zoom_debug_full.log | tail -30

kill $DEBUG_PID
