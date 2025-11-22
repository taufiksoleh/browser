# CEF Browser Makefile
# Convenience targets for building and testing

.PHONY: all build release debug clean test lint format smoke-test help install deps

# Default build type
BUILD_TYPE ?= Release
BUILD_DIR := build
BUILD_DIR_DEBUG := build-debug
BUILD_DIR_TEST := build-test

# Number of parallel jobs
NPROC := $(shell nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

# Default target
all: build

# =============================================================================
# Build Targets
# =============================================================================

build: ## Build in Release mode
	@mkdir -p $(BUILD_DIR)
	@cd $(BUILD_DIR) && cmake -DCMAKE_BUILD_TYPE=Release ..
	@cmake --build $(BUILD_DIR) -j$(NPROC)
	@echo ""
	@echo "Build complete: $(BUILD_DIR)/cef_browser"

release: build ## Alias for build

debug: ## Build in Debug mode
	@mkdir -p $(BUILD_DIR_DEBUG)
	@cd $(BUILD_DIR_DEBUG) && cmake -DCMAKE_BUILD_TYPE=Debug ..
	@cmake --build $(BUILD_DIR_DEBUG) -j$(NPROC)
	@echo ""
	@echo "Debug build complete: $(BUILD_DIR_DEBUG)/cef_browser"

# =============================================================================
# Run Targets
# =============================================================================

run: build ## Build and run the browser
	@cd $(BUILD_DIR) && ./cef_browser

run-debug: debug ## Build and run in debug mode
	@cd $(BUILD_DIR_DEBUG) && ./cef_browser

run-devtools: build ## Run with DevTools on startup
	@cd $(BUILD_DIR) && ./cef_browser --remote-debugging-port=9222
	@echo "DevTools available at: http://localhost:9222"

# =============================================================================
# Test Targets
# =============================================================================

test: ## Build and run unit tests
	@mkdir -p $(BUILD_DIR_TEST)
	@cd $(BUILD_DIR_TEST) && cmake -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON ..
	@cmake --build $(BUILD_DIR_TEST) -j$(NPROC)
	@cd $(BUILD_DIR_TEST) && ctest --output-on-failure

test-verbose: ## Run tests with verbose output
	@mkdir -p $(BUILD_DIR_TEST)
	@cd $(BUILD_DIR_TEST) && cmake -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON ..
	@cmake --build $(BUILD_DIR_TEST) -j$(NPROC)
	@cd $(BUILD_DIR_TEST) && ctest -V

smoke-test: build ## Run smoke tests
	@chmod +x tests/smoke_test.sh
	@./tests/smoke_test.sh

# =============================================================================
# Code Quality
# =============================================================================

lint: ## Run clang-tidy on source files
	@echo "Running clang-tidy..."
	@for file in src/*.cpp; do \
		echo "Checking: $$file"; \
		clang-tidy $$file -- -std=c++17 -I src 2>/dev/null || true; \
	done
	@echo "Lint complete"

format: ## Format code with clang-format
	@echo "Formatting source files..."
	@for file in src/*.cpp src/*.h; do \
		echo "Formatting: $$file"; \
		clang-format -i $$file; \
	done
	@echo "Formatting complete"

format-check: ## Check code formatting without modifying
	@echo "Checking code formatting..."
	@FAIL=0; \
	for file in src/*.cpp src/*.h; do \
		if ! clang-format --dry-run --Werror $$file 2>/dev/null; then \
			echo "Needs formatting: $$file"; \
			FAIL=1; \
		fi; \
	done; \
	if [ $$FAIL -eq 1 ]; then \
		echo ""; \
		echo "Run 'make format' to fix formatting issues"; \
		exit 1; \
	else \
		echo "All files properly formatted"; \
	fi

# =============================================================================
# Utility Targets
# =============================================================================

clean: ## Remove build directories
	@rm -rf $(BUILD_DIR) $(BUILD_DIR_DEBUG) $(BUILD_DIR_TEST)
	@echo "Clean complete"

clean-cef: ## Remove downloaded CEF (will re-download on next build)
	@rm -rf $(BUILD_DIR)/cef $(BUILD_DIR)/cef.tar.bz2
	@rm -rf $(BUILD_DIR_DEBUG)/cef $(BUILD_DIR_DEBUG)/cef.tar.bz2
	@echo "CEF cache cleaned"

deps: ## Install system dependencies (Linux)
	@echo "Installing dependencies..."
	@if command -v apt-get >/dev/null 2>&1; then \
		sudo apt-get update && sudo apt-get install -y \
			build-essential cmake ninja-build \
			libgtk-3-dev libglib2.0-dev libnss3-dev \
			libatk1.0-dev libatk-bridge2.0-dev libcups2-dev \
			libxcomposite-dev libxdamage-dev libxrandr-dev \
			libgbm-dev libasound2-dev libpangocairo-1.0-0 \
			clang-format libgtest-dev; \
	elif command -v dnf >/dev/null 2>&1; then \
		sudo dnf install -y cmake ninja-build gtk3-devel \
			nss-devel atk-devel cups-devel alsa-lib-devel \
			clang-tools-extra gtest-devel; \
	elif command -v pacman >/dev/null 2>&1; then \
		sudo pacman -S --noconfirm cmake ninja gtk3 nss \
			clang gtest; \
	else \
		echo "Unknown package manager. Please install dependencies manually."; \
	fi

install: build ## Install browser to /usr/local/bin
	@sudo cmake --install $(BUILD_DIR) --prefix /usr/local
	@echo "Installed to /usr/local"

# =============================================================================
# Documentation
# =============================================================================

docs: ## Generate documentation (if doxygen available)
	@if command -v doxygen >/dev/null 2>&1; then \
		doxygen Doxyfile 2>/dev/null || echo "No Doxyfile found"; \
	else \
		echo "Doxygen not installed"; \
	fi

# =============================================================================
# Help
# =============================================================================

help: ## Show this help
	@echo "CEF Browser - Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Examples:"
	@echo "  make              # Build release"
	@echo "  make run          # Build and run"
	@echo "  make test         # Run unit tests"
	@echo "  make smoke-test   # Run smoke tests"
	@echo "  make format       # Format source code"
