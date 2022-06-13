#[naked]
pub unsafe fn neon_ins_v0S0_v21S0() {
    core::arch::asm!(r#"
        ins v0.s[0], v21.s[0]
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_ins_v0S1_v21S1() {
    core::arch::asm!(r#"
        ins v0.s[1], v21.s[1]
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_ins_v0S2_v21S2() {
    core::arch::asm!(r#"
        ins v0.s[2], v21.s[2]
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_ins_v0S3_v21S3() {
    core::arch::asm!(r#"
        ins v0.s[3], v21.s[3]
        ret
    "#, options(noreturn));
}

