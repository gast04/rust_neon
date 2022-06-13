all:
	cargo build
	echo ""
	#hexdump -C /tmp/code.dump

	adb push target/aarch64-linux-android/debug/jit_neon /data/local/tmp
	adb shell /data/local/tmp/jit_neon
	adb shell xxd /data/local/tmp/code.dump
