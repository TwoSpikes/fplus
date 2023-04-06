.PHONY: clean install uninstall

clean:
			rm -rf ./Cargo.lock ./target/
./target/release/deps/fplus-5e7bf815de1dcda8.o: main.rs
			cargo rustc --release -- --emit=obj
install:
			install ./target/release/fplus ${PREFIX}/bin
uninstall:
			rm -rf /usr/bin/fplus
