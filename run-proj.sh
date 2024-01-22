echo runing project
clear
cargo build && ./target/debug/svd2rust-story-line -i STM32F411.svd
echo done