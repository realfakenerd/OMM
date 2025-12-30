#!/bin/bash
if [ -f "package.json" ] && [ -d "src-tauri" ]; then
  echo "Project initialized"
  exit 0
else
  echo "Project not initialized"
  exit 1
fi
