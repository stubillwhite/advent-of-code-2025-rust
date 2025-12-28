# Constants
# ==============================================================================

COLOR_BLUE=\033[0;34m
COLOR_NONE=\033[0m

# Targets
# ==============================================================================

# Help
# ======================================

help:
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "$(COLOR_BLUE)%s|$(COLOR_NONE)%s\n", $$1, $$2}' \
		| column -t -s '|'

# Clean
# ======================================

.PHONY: clean
clean: ## Remove all artefacts
	@echo 'Cleaning application'
	@cargo clean

.PHONY: clean-all
clean-all: clean ## Remove all artefacts and dependencies
	@echo 'Cleaning dependencies'

# Running
# ======================================

run: ## Run the app
	@echo 'Running application'
	@cargo run
	@echo

test: ## Run the tests
	@echo 'Running tests'
	@cargo test
	@echo

check: ## Check the code
	@echo 'Running linter'
	@uv run ruff check
	@echo
	@echo 'Running static type checks'
	@uv run dmypy run -- src tests
	@echo

format: ## Format the code
	@echo 'Formatting code'
	@rustfmt --version
	@cargo fmt
	@echo

build: format test ## Build the code
	@echo 'Building artefacts'
	@uv build
