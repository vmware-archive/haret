build:
	cargo build

run:
	cargo run --bin haret

test:
	cargo test

lint:
	@echo cargo clipppy
	@cargo clippy || (echo "Install clippy with 'cargo install clippy'"; exit 1)

format:
	@which rustfmt || cargo install rustfmt
	cargo fmt -- --write-mode=overwrite

SHELL := /bin/bash
.PHONY: devrel launch stop start-dev% stop-dev%
NUM_NODES=3
nodes := $(shell eval "for ((i=1; i<=${NUM_NODES}; i++)); do echo dev\$${i}; done")

# Create a devrel release with `make devrel`
$(eval devrel: $(foreach n,${nodes},dev/${n}))

# Start all dev nodes with `make launch`
$(eval launch: stop $(foreach n,${nodes},start-${n}))

dev/%: build
	mkdir -p $@
	cp target/debug/haret $@
	cp config.toml $@
	cd $@; \
	 $(PWD)/target/debug/devconfig $(notdir $@)

# Start a single node (e.g. `make start-dev1`)
start-dev%:
	cd $(patsubst start-dev%,dev/dev%,$@); \
	  ./haret & \
	  echo $$! > haret.pid

# Stop a single node (e.g. `make stop-dev1`)
stop-dev%:
	kill $(patsubst stop-%,`cat dev/%/haret.pid`,$@);

# Stop all haret nodes on localhost
stop:
	killall haret; exit 0
