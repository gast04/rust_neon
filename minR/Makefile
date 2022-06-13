all:
	cargo build --release
	scp target/aarch64-unknown-none-softfloat/release/minR macarm_denv: 
	ssh macarm_denv "/usr/local/bin/adb -s emulator-5554 push minR /data/local/tmp"
	ssh macarm_denv "/usr/local/bin/adb -s emulator-5554 shell /data/local/tmp/minR && echo $$?"

