install:
	cargo build
	mkdir -p ~/.cdd/services
	cp -f target/debug/cdd-rust ~/.cdd/services/cdd-rust