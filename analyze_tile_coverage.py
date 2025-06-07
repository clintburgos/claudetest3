#!/usr/bin/env python3
"""
Analyze tile coverage screenshots to detect black background areas.
Black areas indicate missing tiles that should be visible.
"""

import os
import sys
from PIL import Image
import numpy as np

def detect_black_areas(image_path, black_threshold=10):
    """
    Detect black or near-black pixels in an image.
    Returns percentage of black pixels and their locations.
    """
    img = Image.open(image_path).convert('RGB')
    img_array = np.array(img)
    
    # Detect pixels that are very dark (near black)
    # Black background would be RGB values all below threshold
    black_mask = np.all(img_array < black_threshold, axis=2)
    
    total_pixels = img_array.shape[0] * img_array.shape[1]
    black_pixels = np.sum(black_mask)
    black_percentage = (black_pixels / total_pixels) * 100
    
    # Find regions of black pixels
    if black_pixels > 0:
        # Find bounding box of black areas
        black_coords = np.argwhere(black_mask)
        if len(black_coords) > 0:
            top_left = black_coords.min(axis=0)
            bottom_right = black_coords.max(axis=0)
            return {
                'has_black': True,
                'percentage': black_percentage,
                'black_pixels': black_pixels,
                'total_pixels': total_pixels,
                'black_region': {
                    'top': int(top_left[0]),
                    'left': int(top_left[1]),
                    'bottom': int(bottom_right[0]),
                    'right': int(bottom_right[1])
                }
            }
    
    return {
        'has_black': False,
        'percentage': black_percentage,
        'black_pixels': black_pixels,
        'total_pixels': total_pixels
    }

def analyze_ui_regions(image_path):
    """
    Check if black areas are in expected UI regions (header/panels).
    """
    img = Image.open(image_path)
    width, height = img.size
    
    # Expected UI regions (approximate)
    ui_regions = {
        'header': {'top': 0, 'bottom': 60, 'left': 0, 'right': width},
        'sidebar': {'top': 60, 'bottom': height, 'left': 0, 'right': 200}
    }
    
    return ui_regions, (width, height)

def main():
    screenshot_dir = "tile_coverage_test"
    
    if not os.path.exists(screenshot_dir):
        print(f"Directory '{screenshot_dir}' not found!")
        print("Run: cargo run --bin test_tile_coverage")
        sys.exit(1)
    
    screenshots = sorted([f for f in os.listdir(screenshot_dir) if f.endswith('.png')])
    
    if not screenshots:
        print("No screenshots found!")
        sys.exit(1)
    
    print(f"Analyzing {len(screenshots)} screenshots for tile coverage issues...\n")
    
    issues_found = []
    
    for screenshot in screenshots:
        filepath = os.path.join(screenshot_dir, screenshot)
        print(f"Analyzing: {screenshot}")
        
        # Detect black areas
        result = detect_black_areas(filepath)
        ui_regions, img_size = analyze_ui_regions(filepath)
        
        if result['has_black']:
            print(f"  ⚠️  BLACK AREAS DETECTED: {result['percentage']:.1f}% of image")
            
            # Check if black area is outside expected UI regions
            black_region = result['black_region']
            
            # Exclude UI regions from analysis
            game_area_black = False
            
            # Check if black area extends beyond header
            if black_region['bottom'] > ui_regions['header']['bottom']:
                # Check if it's not just the sidebar
                if black_region['right'] > ui_regions['sidebar']['right']:
                    game_area_black = True
            
            if game_area_black:
                print(f"  ❌ BLACK AREA IN GAME VIEW: {black_region}")
                issues_found.append({
                    'file': screenshot,
                    'black_percentage': result['percentage'],
                    'region': black_region,
                    'severity': 'HIGH'
                })
            else:
                print(f"  ✓ Black areas only in UI regions (expected)")
        else:
            print(f"  ✓ No significant black areas ({result['percentage']:.2f}%)")
    
    # Summary
    print("\n" + "="*50)
    print("SUMMARY")
    print("="*50)
    
    if issues_found:
        print(f"\n❌ ISSUES FOUND: {len(issues_found)} screenshots have black areas in game view\n")
        
        # Sort by severity
        issues_found.sort(key=lambda x: x['black_percentage'], reverse=True)
        
        print("Most problematic screenshots:")
        for i, issue in enumerate(issues_found[:5]):
            print(f"{i+1}. {issue['file']} - {issue['black_percentage']:.1f}% black")
            print(f"   Region: top={issue['region']['top']}, left={issue['region']['left']}, "
                  f"bottom={issue['region']['bottom']}, right={issue['region']['right']}")
        
        print("\nThese screenshots show missing tile coverage!")
        print("The tile culling system needs to be adjusted to ensure tiles")
        print("always extend beyond the visible viewport.")
        
        # Identify patterns
        edge_issues = [i for i in issues_found if 'edge' in i['file'].lower()]
        zoom_issues = [i for i in issues_found if 'zoom' in i['file'].lower()]
        
        if edge_issues:
            print(f"\n⚠️  {len(edge_issues)} issues at map edges")
        if zoom_issues:
            print(f"⚠️  {len(zoom_issues)} issues when zoomed out")
            
    else:
        print("\n✅ SUCCESS: No black areas detected in game view!")
        print("Tile coverage appears to be working correctly.")
    
    return len(issues_found) == 0

if __name__ == "__main__":
    try:
        import PIL
        import numpy
    except ImportError:
        print("Required packages missing. Install with:")
        print("pip install Pillow numpy")
        sys.exit(1)
    
    success = main()
    sys.exit(0 if success else 1)