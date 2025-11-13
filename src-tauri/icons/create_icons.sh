#!/bin/bash
# Create minimal valid PNG files (1x1 blue pixel)
# PNG magic bytes + minimal 1x1 blue image

# Base64 of a minimal 1x1 blue PNG
MINIMAL_PNG="iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="

echo "$MINIMAL_PNG" | base64 -d > 32x32.png
echo "$MINIMAL_PNG" | base64 -d > 128x128.png
echo "$MINIMAL_PNG" | base64 -d > 128x128@2x.png
echo "$MINIMAL_PNG" | base64 -d > icon.png
echo "$MINIMAL_PNG" | base64 -d > icon.ico

echo "Minimal placeholder icons created"
ls -lh
