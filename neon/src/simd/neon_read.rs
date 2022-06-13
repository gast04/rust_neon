#[naked]
pub unsafe fn neon_read_v0_4s(a: &mut [u32]) {
	// parameter is used as result, mods v0
    core::arch::asm!(r#"
        st1.4s {{v0}},[x0],#16
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_read_v1_4s(a: &mut [u32]) {
	// parameter is used as result, mods v0
    core::arch::asm!(r#"
        st1.4s {{v1}},[x0],#16
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_read_v20_4s(a: &mut [u32]) {
	// parameter is used as result, mods v0
    core::arch::asm!(r#"
        st1.4s {{v20}},[x0],#16
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_read_v21_4s(a: &mut [u32]) {
	// parameter is used as result, mods v0
    core::arch::asm!(r#"
        st1.4s {{v21}},[x0],#16
        ret
    "#, options(noreturn));
}
