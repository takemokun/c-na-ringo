.PHONY: setup test build install verify

setup: test install verify
	@echo ""
	@echo "🍎 ringo-srs setup complete!"
	@echo ""
	@echo "  Binary: ./bin/ringo-srs"
	@echo "  All /ringo-srs-* skills are ready to use."

test:
	cargo test --manifest-path ringo-srs/Cargo.toml

install:
	cargo install --path ringo-srs --root .

verify:
	./bin/ringo-srs --help > /dev/null
