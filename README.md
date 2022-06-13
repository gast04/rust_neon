Binary based on twitch stream from gamozo:
https://www.twitch.tv/videos/1430829311
-> starting by about an hour into the video



# Commands
* `rustc --print target-list`
* `rustc --target aarch64-unknown-none -Z unstable-options --print target-spec-json`
* `rustc --target aarch64-unknown-none-softfloat -Z unstable-options --print target-spec-json`


# Readings for NEON
* https://branchfree.org/2019/03/26/an-intel-programmer-jumps-over-the-wall-first-impressions-of-arm-simd-programming/
* https://branchfree.org/2019/04/01/fitting-my-head-through-the-arm-holes-or-two-sequences-to-substitute-for-the-missing-pmovmskb-instruction-on-arm-neon/
* https://gamozolabs.github.io/fuzzing/2018/10/14/vectorized_emulation.html
	* https://medium.com/@Razican/learning-simd-with-rust-by-finding-planets-b85ccfb724c3

* https://modexp.wordpress.com/2018/10/30/arm64-assembly/#registers

