CARGO := cargo

.PHONY: format
format:
	$(CARGO) fmt

.PHONY: test
test: format
	$(CARGO) test
