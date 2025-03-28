default:
	cargo build --release
	rust-objcopy -O binary target/aarch64-unknown-none/release/rpi3-baremetal kernel8.img

clean:
	rm -rf target
	cargo clean
	cargo build --release
	rust-objcopy -O binary target/aarch64-unknown-none/release/rpi3-baremetal kernel8.img

run: 
	cargo build --release
	rust-objcopy -O binary target/aarch64-unknown-none/release/rpi3-baremetal kernel8.img
	qemu-system-aarch64 -M raspi3b -kernel kernel8.img  -serial stdio