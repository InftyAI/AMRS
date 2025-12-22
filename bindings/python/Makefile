POETRY := poetry
RUFF := .venv/bin/ruff
PYTEST := .venv/bin/pytest

.PHONY: build
build: lint
	$(POETRY) build

.PHONY: publish
publish: build
	$(POETRY) publish --username=__token__ --password=$(INFTYAI_PYPI_TOKEN)

.PHONY: lint
lint:
	$(RUFF) check .

.PHONY: format
format:
	$(RUFF) format .
	$(RUFF) check --fix .

.PHONY: test
test: lint
	$(PYTEST) tests/unit --timeout=15

.PHONY: test-integration
test-integration: lint
	$(PYTEST) tests/integration --timeout=30

.PHONY: test-all
test-all: test test-integration
