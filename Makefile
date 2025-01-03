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

run2:
	@cargo r $(MODE_ARG) --bin the_next_week

clean:
	@cargo clean

.PHONY: check build run1 run2 clean
