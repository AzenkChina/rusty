cargo run
cargo build --release
cargo objcopy --target thumbv6m-none-eabi --bin firmware --release -- -O binary firmware.bin