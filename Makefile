build:
	cargo build

run:
	cargo run --bin v2r2

test:
	cargo test

.PHONY: devrel launch stop
NUM_NODES=3
nodes := $(shell eval "for ((i=1; i<=${NUM_NODES}; i++)); do echo dev/dev\$${i}; done")
$(eval devrel: ${nodes})
$(eval launch: stop $(foreach n,${nodes},launch-${n}))

dev/%: build
	mkdir -p $@
	cp target/debug/v2r2 $@
	cp config.json $@
	cd $@; \
	 $(PWD)/target/debug/devconfig $(notdir $@)

launch-dev/%: devrel
	cd $(patsubst launch-%,%,$@); \
	  ./v2r2 &

stop:
	killall v2r2; exit 0
