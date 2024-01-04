build:
	cargo build --release --frozen

install:
	cp target/release/poketex /usr/local/bin/
	mkdir -p /usr/local/share/poketex
	cp -rf colorscripts /usr/local/share/poketex

uninstall:
	rm /usr/local/bin/poketex
	rm -rf /usr/local/share/poketex

clean:
	rm -rf target

all: build install clean
