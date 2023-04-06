install: fplus
			install ./target/release/fplus ${PREFIX}/bin
fplus: main.rs
			cargo build --release
uninstall:
			rm -rf /usr/bin/fplus
clean:
			rm -rf ./Cargo.lock ./target/
