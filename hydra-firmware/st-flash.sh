#!/bin/bash

# Currently not working.
# To flash the chip, use GDB load with ST Utils
st-flash write ./target/thumbv7em-none-eabihf/debug/hydra 0x08000000
