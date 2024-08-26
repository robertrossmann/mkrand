CURRENT_DIR := $(notdir $(shell pwd))

all: compile

compile:
	cargo build --release

link:
	cargo install --path .

clean:
	rm -rf bin target
	git clean -Xf
