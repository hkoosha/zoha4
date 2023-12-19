.DEFAULT_GOAL = r

# RUSTFLAGS := '-Awarnings'
# RUSTFLAGS := ''

export

.PHONY: c
c:
	@for i in $(shell seq 0 100); do echo; done

.PHONY: clean
clean:
	cargo clean

.PHONY: clippy
clippy:
	cargo clippy -- -A clippy::needless_return

.PHONY: run
run: c
	cargo run

.PHONY: build
build: c
	cargo build -r

.PHONY: install
install:
	install -C -v -g root -o root ./target/release/zoha4 /usr/bin/

.PHONY: help
help: build
	./target/release/zoha4 -h

.PHONY: dry-run
dry-run: build
	./target/release/zoha4 --dry-run

.PHONY: dry-run-k
dry-run-k: build
	./target/release/zoha4 --dry-run -k

.PHONY: rk
rk: build
	cargo run -r -- -k

.PHONY: rk
r: build
	cargo run -r

