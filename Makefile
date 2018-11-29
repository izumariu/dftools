RM=rm -rf

target/debug/dftools: Cargo.toml src/main.rs
	cargo build

run: target/debug/dftools
	@$<

clean:; $(RM) Cargo.lock target/
.PHONY: clean run
