#[naked]
pub unsafe fn neon_mov_v20_v0_4s() {
    core::arch::asm!(r#"
        mov.4s v20, v0
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_mov_v20_v1_4s() {
    core::arch::asm!(r#"
        mov.4s v20, v1
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_mov_v21_v0_4s() {
    core::arch::asm!(r#"
        mov.4s v21, v0
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_mov_v21_v1_4s() {
    core::arch::asm!(r#"
        mov.4s v21, v1
        ret
    "#, options(noreturn));
}
