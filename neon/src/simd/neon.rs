/*  Carefull, due to array registers are filled in the order:
    X0 -> addr to a
    X1 -> size of a
    X2 -> addr to b
    X3 -> size of b
    X4 -> addr to c
    X5 -> size of c
*/

#[naked]
pub unsafe fn neon_add_16b(a: &[u8], b: &[u8], c: &mut [u8]) {
    core::arch::asm!(r#"
        ld1.16b {{v0, v1, v2, v3}},[x0],#64
        ld1.16b {{v4, v5, v6, v7}},[x2],#64
        add v0.16b, v0.16b, v4.16b
        add v1.16b, v1.16b, v5.16b
        add v2.16b, v2.16b, v6.16b
        add v3.16b, v3.16b, v7.16b
        st1.16b {{v0, v1, v2, v3}},[x4],#64
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_add_4s(a: &[u32], b: &[u32], c: &mut [u32]) {
    core::arch::asm!(r#"
        ld1.4s {{v0, v1, v2, v3}},[x0],#64
        ld1.4s {{v4, v5, v6, v7}},[x2],#64
        add v0.4s, v0.4s, v4.4s
        add v1.4s, v1.4s, v5.4s
        add v2.4s, v2.4s, v6.4s
        add v3.4s, v3.4s, v7.4s
        st1.4s {{v0, v1, v2, v3}},[x4],#64
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_add_2d(a: &[u64], b: &[u64], c: &mut [u64]) {
    core::arch::asm!(r#"
        ld1.2d {{v0, v1, v2, v3}},[x0],#64
        ld1.2d {{v4, v5, v6, v7}},[x2],#64
        add v0.2d, v0.2d, v4.2d
        add v1.2d, v1.2d, v5.2d
        add v2.2d, v2.2d, v6.2d
        add v3.2d, v3.2d, v7.2d
        st1.2d {{v0, v1, v2, v3}},[x4],#64
        ret
    "#, options(noreturn));
}


#[naked]
pub unsafe fn neon_fill_v0_4s(a: &[u32]) {
    core::arch::asm!(r#"
        ld1.4s {{v0}},[x1]
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_fill_v1_4s(a: &[u32]) {
    core::arch::asm!(r#"
        ld1.4s {{v1}},[x2],#16
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_fill_v2_4s(a: &[u32]) {
    core::arch::asm!(r#"
        ld1.4s {{v2}},[x3],#16
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_cmp_v20_v0_v1_4s() {
	// parameter is used as result, mods v0
    core::arch::asm!(r#"
        cmeq v20.4s, v0.4s, v1.4s
        cmeq v0.4s, v0.4s, v0.4s
        cmeq v0.4s, v0.4s, v1.4s
        cmeq v0.4s, v1.4s, v2.4s
        cmeq v1.4s, v2.4s, v3.4s
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_add_v0_v1_4s() {
    core::arch::asm!(r#"
        add v0.4s, v0.4s, v1.4s
        ret
    "#, options(noreturn));
}

#[naked]
pub unsafe fn neon_sub_v0_v2_4s() {
    core::arch::asm!(r#"
        sub v0.4s, v0.4s, v2.4s
        ret
    "#, options(noreturn));
}

