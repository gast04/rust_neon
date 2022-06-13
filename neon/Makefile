all:
	cargo build
	adb push target/aarch64-linux-android/debug/neon /data/local/tmp
	adb shell /data/local/tmp/neon

dbg: all 
	adb forward tcp:23948 tcp:23946
	# assumes ida dbgserver to on device
	#adb shell su -c /data/local/tmp/android_server64

macarm:
	cargo build
	scp target/aarch64-linux-android/debug/neon macarm_denv:KNIStests/NeonVectorized
	ssh macarm_denv '/usr/local/bin/adb push /Users/macarm/KNIStests/NeonVectorized/neon /data/local/tmp'
	ssh macarm_denv '/usr/local/bin/adb shell /data/local/tmp/neon'
