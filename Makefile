build:
	cargo build --release

install:
	cp target/release/jspcompile ~/bin/.

test:
	cargo test --release	