#!/bin/bash
echo "Testing edge tile visibility at minimum zoom..."
cargo run --example test_zoom_edges 2>&1 | grep -E "(Camera limits|corner tile|Edge tile|Culling:|PHASE 1)" | head -20