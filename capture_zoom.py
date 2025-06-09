#!/usr/bin/env python3
import subprocess
import time
import sys
import os

# Start the game
print("Starting the game...")
game_proc = subprocess.Popen(["cargo", "run", "--bin", "claudetest3"], 
                            stdout=subprocess.PIPE, 
                            stderr=subprocess.STDOUT,
                            universal_newlines=True)

# Wait for game to fully start
print("Waiting for game to initialize...")
time.sleep(5)

# Use AppleScript to send key events to the game window
def send_key(key):
    script = f'''
    tell application "System Events"
        tell process "claudetest3"
            key code {key}
        end tell
    end tell
    '''
    subprocess.run(["osascript", "-e", script])

# Key codes for E (zoom out)
E_KEY = 14

print("Sending zoom out commands...")
for i in range(10):
    print(f"Zoom out {i+1}/10")
    send_key(E_KEY)
    time.sleep(1)

print("Waiting a bit more...")
time.sleep(3)

# Kill the game
print("Terminating game...")
game_proc.terminate()
game_proc.wait()

print("Done!")