# Scoop — common development targets
# Usage: make [target]

.PHONY: help install install-desktop install-ocr dev build build-deb build-appimage frontend test test-math check typecheck clean clean-data reset-data

DESKTOP := apps/desktop
TAURI   := $(DESKTOP)/src-tauri
NPM     := npm --prefix $(DESKTOP)
BUNDLE  := $(TAURI)/target/release/bundle

# linuxdeploy walks PATH and crashes on broken symlink loops
# (seen with /usr/local/bin/kubectx → kubectx/kubectx).
# Prefer newest nvm/fnm Node when present; never put /usr/local/bin on the bundler PATH.
NVM_NODE := $(shell ls -1d $(HOME)/.nvm/versions/node/*/bin 2>/dev/null | sort -V | tail -1)
FNM_NODE := $(shell ls -1d $(HOME)/.local/share/fnm/node-versions/*/installation/bin 2>/dev/null | sort -V | tail -1)
SAFE_PATH := $(shell printf '%s\n' $(NVM_NODE) $(FNM_NODE) $(HOME)/.cargo/bin $(HOME)/.local/bin /usr/bin /bin /usr/sbin /sbin | awk 'NF' | paste -sd:)

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

build: ## Production build (deb + AppImage) with linuxdeploy-safe PATH
	@echo "Building Scoop (PATH sanitized for linuxdeploy)…"
	PATH="$(SAFE_PATH)" APPIMAGE_EXTRACT_AND_RUN=1 NO_STRIP=true \
		$(NPM) run tauri -- build
	@echo ""
	@echo "Artifacts:"
	@ls -lah $(BUNDLE)/deb/*.deb 2>/dev/null || true
	@ls -lah $(BUNDLE)/appimage/*.AppImage 2>/dev/null || true

build-deb: ## Production .deb only (skips AppImage)
	PATH="$(SAFE_PATH)" $(NPM) run tauri -- build --bundles deb
	@ls -lah $(BUNDLE)/deb/*.deb

build-appimage: ## Production AppImage only
	PATH="$(SAFE_PATH)" APPIMAGE_EXTRACT_AND_RUN=1 NO_STRIP=true \
		$(NPM) run tauri -- build --bundles appimage
	@ls -lah $(BUNDLE)/appimage/*.AppImage

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
