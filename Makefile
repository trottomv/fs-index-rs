.DEFAULT_GOAL := help

.PHONY: check
check:  ## Check app configuration
	cargo check
	cargo audit bin ~/.cargo/bin/fs-index

.PHONY: run
fix:  ## Fix formatting and linting
	cargo fmt

.PHONY: precommit
precommit:  ## Fix code formatting and linting
	pre-commit run --all-files

.PHONY: precommit_update
precommit_update:  ## Update pre_commit
	python3 -m pre_commit autoupdate

.PHONY: coverage
coverage:  ## Run test
	cargo tarpaulin \
		--bin fs-index \
		--fail-under 90 \
		--offline \
		--out Html \
		--output-dir .coverage \
		--run-types Tests \
		--skip-clean

.PHONY: test
test:  ## Run test
	cargo test \
		--bin fs-index \
		--offline

.PHONY: help
help:
	@echo "[Help] Makefile list commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
