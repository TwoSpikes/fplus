install: $(compiled)
	install ./target/release/fplus ${PREFIX}/bin
fplus: main.rs
	$(eval compiled = ./target/release/fplus)
	cargo build --release
debug: main.rs
	$(eval compiled = ./target/debug/fplus)
	cargo build
uninstall:
	rm -rf /usr/bin/fplus
clean:
	rm -rf ./Cargo.lock ./target/
