MODE := release

ifeq ($(MODE), release)
    MODE_ARG := -r
endif

check:
	@cargo c $(MODE_ARG)

build:
	@cargo b $(MODE_ARG)

run1:
	@cargo r $(MODE_ARG) --bin in_one_weekend

clean:
	@cargo clean

.PHONY: check build run1 clean