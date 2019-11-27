cargo build --release
cargo objcopy --target thumbv7em-none-eabihf --bin firmware --release -- -O binary firmware.bin