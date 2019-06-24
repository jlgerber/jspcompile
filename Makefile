build:
	cargo build --release

install:
	cp target/release/jspcompile ~/bin/.

.PHONY: all
all: build install

test:
	cargo test --release

clean:
	rm -rf target	