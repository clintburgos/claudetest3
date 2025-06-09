#!/usr/bin/env python3

import subprocess
import time
import os
import signal

def test_zoom():
    print("Starting zoom test...")
    
    # Start the game
    proc = subprocess.Popen(['cargo', 'run', '--bin', 'claudetest3'])
    
    try:
        # Wait for game to start
        print("Waiting for game to load...")
        time.sleep(5)
        
        # Click the Start Game button (assuming it's roughly centered)
        print("Clicking Start Game...")
        os.system("osascript -e 'tell application \"System Events\" to click at {640, 450}'")
        
        # Wait for game to transition to playing state
        time.sleep(3)
        
        # Now zoom out by pressing 'e' multiple times
        print("Starting zoom out sequence...")
        for i in range(15):
            print(f"Zoom out iteration {i+1}/15")
            os.system("osascript -e 'tell application \"System Events\" to key code 14'")  # 'e' key
            time.sleep(0.5)
        
        # Wait to capture final state
        print("Capturing final state...")
        time.sleep(3)
        
    finally:
        # Kill the game
        print("Stopping game...")
        os.kill(proc.pid, signal.SIGTERM)
        time.sleep(1)
    
    print("Test complete! Check the latest log directory for screenshots.")

if __name__ == "__main__":
    test_zoom()