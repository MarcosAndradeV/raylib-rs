#!/usr/bin/env bash
set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RAYLIB_SRC_DIR="${SCRIPT_DIR}/raylib-patch"
BUILD_DIR="${RAYLIB_SRC_DIR}/build"
DEST_DIR="${SCRIPT_DIR}/raylib-6.0_linux_amd64"

echo "=== Building Raylib static library with JPG support and -fPIC ==="

# Clean stale build cache if present
if [ -d "${BUILD_DIR}" ]; then
    echo "Cleaning existing build directory..."
    rm -rf "${BUILD_DIR}"
fi

# Configure Raylib with CMake:
# - CMAKE_BUILD_TYPE=Release
# - CMAKE_POSITION_INDEPENDENT_CODE=ON (-fPIC)
# - WITH_PIC=ON (-fPIC)
# - CUSTOMIZE_BUILD=ON
# - SUPPORT_FILEFORMAT_JPG=ON
# - BUILD_EXAMPLES=OFF
cmake -B "${BUILD_DIR}" -S "${RAYLIB_SRC_DIR}" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_POSITION_INDEPENDENT_CODE=ON \
    -DWITH_PIC=ON \
    -DCUSTOMIZE_BUILD=ON \
    -DSUPPORT_FILEFORMAT_JPG=ON \
    -DBUILD_EXAMPLES=OFF

# Build Raylib static library
cmake --build "${BUILD_DIR}" --config Release

echo "=== Copying built artifacts to ${DEST_DIR} ==="

# Ensure target directories exist
mkdir -p "${DEST_DIR}/lib"
mkdir -p "${DEST_DIR}/include"

# Remove nested include dir if previously created by cp -r
rm -rf "${DEST_DIR}/include/include"

# Find built static library
LIB_PATH=""
if [ -f "${BUILD_DIR}/raylib/libraylib.a" ]; then
    LIB_PATH="${BUILD_DIR}/raylib/libraylib.a"
elif [ -f "${BUILD_DIR}/libraylib.a" ]; then
    LIB_PATH="${BUILD_DIR}/libraylib.a"
fi

if [ -z "${LIB_PATH}" ]; then
    echo "Error: libraylib.a not found in ${BUILD_DIR}" >&2
    exit 1
fi

# Copy libraylib.a to destination lib directory and destination root
cp -v "${LIB_PATH}" "${DEST_DIR}/lib/libraylib.a"
cp -v "${LIB_PATH}" "${DEST_DIR}/libraylib.a"
cp -v "${LIB_PATH}" "${DEST_DIR}/raylib.a"

# Copy header (.h) files from raylib-patch/src and raylib-patch/build/raylib/include
cp -v "${RAYLIB_SRC_DIR}"/src/*.h "${DEST_DIR}/include/"
if [ -d "${BUILD_DIR}/raylib/include" ]; then
    cp -v "${BUILD_DIR}"/raylib/include/*.h "${DEST_DIR}/include/"
fi
cp -v "${RAYLIB_SRC_DIR}"/src/*.h "${DEST_DIR}/"

echo "=== Raylib build and copy completed successfully! ==="
