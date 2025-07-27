default: release


debug:
	cargo build --debug

release:
	cargo build --release

test:
	cargo test

VERSION = $(shell grep version Cargo.toml | awk '{ print $$3 }' | tr -d '"')
OWNER	?= root
GROUP	?= root

distribution: srcrr-$(VERSION).tgz

srcrr-$(VERSION).tgz: target/release/srcrr srcrr.bash LICENSE
	rm -rf work
	mkdir -p work/bin
	mkdir -p work/share/srcrr
	cp target/release/srcrr work/bin/
	cp srcrr.bash work/share/srcrr/
	cp LICENSE work/share/srcrr/
	cd work && tar \
		--owner $(OWNER) \
		--group $(GROUP) \
		-czf ../srcrr-$(VERSION).tgz *
	rm -r work

DESTDIR	?= /usr/local

install: distribution
	tar xzvf srcrr-$(VERSION).tgz -C $(DESTDIR)

clean:
	rm -rf work
	rm -f srcrr-$(VERSION).tgz

clean-all: clean
	cargo clean
