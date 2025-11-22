#!/bin/bash
echo "Checking C++ code formatting..."
CHANGED=0
for file in $(find src -name '*.cpp' -o -name '*.h'); do
  if ! clang-format-15 --dry-run --Werror "$file" 2>/dev/null; then
    echo "Formatting issue in: $file"
    clang-format-15 --dry-run "$file" 2>&1 | head -20
    CHANGED=1
  fi
done
if [ $CHANGED -eq 1 ]; then
  echo ""
  echo "Code formatting issues found. Run: clang-format -i src/*.cpp src/*.h"
  exit 1
fi
echo "All files properly formatted!"
