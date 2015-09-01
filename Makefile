build:
	cargo build

run:
	cargo run --bin v2r2

test:
	cargo test

.PHONY: devrel launch stop start-dev% stop-dev%
NUM_NODES=3
nodes := $(shell eval "for ((i=1; i<=${NUM_NODES}; i++)); do echo dev\$${i}; done")

# Create a devrel release with `make devrel`
$(eval devrel: $(foreach n,${nodes},dev/${n}))

# Start all dev nodes with `make launch`
$(eval launch: stop $(foreach n,${nodes},start-${n}))

dev/%: build
	mkdir -p $@
	cp target/debug/v2r2 $@
	cp config.json $@
	cd $@; \
	 $(PWD)/target/debug/devconfig $(notdir $@)

# Start a single node (e.g. `make start-dev1`)
start-dev%: devrel
	cd $(patsubst start-dev%,dev/dev%,$@); \
	  ./v2r2 & \
	  echo $$! > v2r2.pid

# Stop a single node (e.g. `make stop-dev1`)
stop-dev%:
	kill $(patsubst stop-%,`cat dev/%/v2r2.pid`,$@);

# Stop all v2r2 nodes on localhost
stop:
	killall v2r2; exit 0
