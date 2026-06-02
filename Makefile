FNM := $(HOME)/.local/share/fnm/fnm

.PHONY: release

# Bump the version, commit, tag, and push.
# Usage: make release VERSION=1.2.3
release:
ifndef VERSION
	$(error VERSION is not set. Usage: make release VERSION=x.y.z)
endif
	@echo "Checking version format…"
	@echo "$(VERSION)" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$$' || \
		(echo "Error: VERSION must be x.y.z (got '$(VERSION)')"; exit 1)
	@echo "Patching package.json…"
	@eval "$$($(FNM) env --shell bash)" && npm version $(VERSION) --no-git-tag-version
	@echo "Patching src-tauri/Cargo.toml…"
	sed -i 's/^version = ".*"/version = "$(VERSION)"/' src-tauri/Cargo.toml
	@echo "Updating Cargo.lock…"
	cd src-tauri && cargo update --workspace --quiet
	@echo "Staging changes…"
	git add package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock
	git commit -m "chore: release v$(VERSION)"
	@echo "Tagging v$(VERSION)…"
	git tag -a v$(VERSION) -m "v$(VERSION)"
	@echo "Pushing commit and tag…"
	git push && git push origin v$(VERSION)
	@echo "Done. CI will build the installers and create a draft release."
