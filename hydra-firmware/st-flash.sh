#!/bin/bash

# Build the binary and flashes the chip.
cargo objcopy --profile flash -- -O binary hydra.bin
st-flash write ./hydra.bin 0x08000000
