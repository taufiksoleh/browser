#!/bin/bash
# CEF Browser Build Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
BUILD_TYPE="${1:-Release}"

echo "==================================="
echo "CEF Browser Build Script"
echo "==================================="
echo "Build Type: ${BUILD_TYPE}"
echo "Build Directory: ${BUILD_DIR}"
echo ""

# Check for required tools
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo "Error: $1 is not installed."
        exit 1
    fi
}

check_command cmake
check_command make

# Detect platform
OS_NAME=$(uname -s)
echo "Detected Platform: ${OS_NAME}"

# Install dependencies based on platform
install_deps() {
    case "${OS_NAME}" in
        Linux)
            echo "Checking Linux dependencies..."
            if command -v apt-get &> /dev/null; then
                # Debian/Ubuntu
                echo "Detected Debian/Ubuntu system"
                DEPS="build-essential cmake libgtk-3-dev libglib2.0-dev libnss3-dev libatk1.0-dev libatk-bridge2.0-dev libcups2-dev libxcomposite-dev libxdamage-dev libxrandr-dev libgbm-dev libasound2-dev libpangocairo-1.0-0 libpango-1.0-0 libcairo2"
                echo "Required packages: ${DEPS}"
                echo ""
                echo "Install with: sudo apt-get install ${DEPS}"
            elif command -v dnf &> /dev/null; then
                # Fedora/RHEL
                echo "Detected Fedora/RHEL system"
                echo "Install dependencies with: sudo dnf install cmake gtk3-devel nss-devel atk-devel at-spi2-atk-devel cups-devel alsa-lib-devel"
            elif command -v pacman &> /dev/null; then
                # Arch Linux
                echo "Detected Arch Linux system"
                echo "Install dependencies with: sudo pacman -S cmake gtk3 nss"
            fi
            ;;
        Darwin)
            echo "macOS detected. Ensure Xcode Command Line Tools are installed."
            echo "Run: xcode-select --install"
            ;;
        *)
            echo "Warning: Unsupported platform ${OS_NAME}"
            ;;
    esac
    echo ""
}

install_deps

# Create build directory
mkdir -p "${BUILD_DIR}"
cd "${BUILD_DIR}"

# Configure with CMake
echo "Configuring with CMake..."
cmake -DCMAKE_BUILD_TYPE="${BUILD_TYPE}" ..

# Build
echo ""
echo "Building..."
NPROC=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
cmake --build . --config "${BUILD_TYPE}" -j "${NPROC}"

echo ""
echo "==================================="
echo "Build Complete!"
echo "==================================="
echo ""
echo "Binary location: ${BUILD_DIR}/cef_browser"
echo ""
echo "To run the browser:"
echo "  cd ${BUILD_DIR}"
echo "  ./cef_browser"
echo ""
echo "Remote debugging available at: http://localhost:9222"
