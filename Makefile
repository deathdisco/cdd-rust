install:
	cargo build
	mkdir -p ~/.cdd/bin
	cp -f target/debug/cdd-rust ~/.cdd/bin/cdd-rust

test:
	cargo run -- list-models ./examples/structs.rs