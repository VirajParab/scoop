# Scoop — common development targets
# Usage: make [target]

.PHONY: help install install-desktop install-ocr dev build frontend test test-math check typecheck clean clean-data reset-data

DESKTOP := apps/desktop
TAURI   := $(DESKTOP)/src-tauri
NPM     := npm --prefix $(DESKTOP)

help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "Scoop make targets\n\n"} \
		/^[a-zA-Z0-9_-]+:.*?##/ { printf "  %-16s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

install: install-desktop ## Install desktop npm dependencies

install-desktop: ## npm install in apps/desktop
	$(NPM) install

install-ocr: ## Install Tesseract OCR (Ubuntu/Debian; needs sudo)
	sudo apt-get update
	sudo apt-get install -y tesseract-ocr tesseract-ocr-eng
	@tesseract --version | head -1
	@echo "OCR ready. Restart Scoop (make dev)."

dev: ## Run Scoop in development (Tauri + Vite)
	$(NPM) run tauri -- dev

build: ## Production Tauri build (deb / AppImage when configured)
	$(NPM) run tauri -- build

frontend: ## Build frontend only (tsc + vite)
	$(NPM) run build

test: test-math ## Run all project tests

test-math: ## Run Rust math engine unit tests
	cd $(TAURI) && cargo test math_engine --lib

check: ## cargo check the Tauri crate
	cd $(TAURI) && cargo check

typecheck: ## TypeScript check (no emit)
	cd $(DESKTOP) && npx tsc --noEmit

clean: ## Remove build artifacts (node dist + cargo target)
	rm -rf $(DESKTOP)/dist
	rm -rf $(TAURI)/target
	@echo "Cleaned $(DESKTOP)/dist and $(TAURI)/target"

clean-data: ## Delete local Scoop data + cache (XDG)
	rm -rf "$(or $(XDG_DATA_HOME),$(HOME)/.local/share)/scoop"
	rm -rf "$(or $(XDG_CACHE_HOME),$(HOME)/.cache)/scoop"
	@echo "Removed Scoop data and cache dirs"

reset-data: clean-data ## Alias for clean-data
