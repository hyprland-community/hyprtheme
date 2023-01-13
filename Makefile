build:
	cargo build --release

install:
	sudo cp target/release/hyprtheme /usr/bin/hyprtheme

all: build install
 
help:
	@echo "usage: make [build|install|all|help]"
