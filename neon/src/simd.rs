#[cfg(target_arch = "aarch64")] mod neon;
#[cfg(target_arch = "aarch64")] use neon::*;

#[cfg(target_arch = "aarch64")] mod neon_mov;
#[cfg(target_arch = "aarch64")] use neon_mov::*;

#[cfg(target_arch = "aarch64")] mod neon_read;
#[cfg(target_arch = "aarch64")] use neon_read::*;

#[cfg(target_arch = "aarch64")] mod neon_ins;
#[cfg(target_arch = "aarch64")] use neon_ins::*;

/*	ARM size convention:
	B (8-bit) byte 
	H (16-bit) half-word
	S (32-bit) word
	D (64-bit) double-word
*/

#[inline(always)]
pub fn add_16b(a: &[u8], b: &[u8], c: &mut [u8]) {
    unsafe {
		neon_add_16b(a, b, c);
    }
}

#[inline(always)]
pub fn add_4s(a: &[u32], b: &[u32], c: &mut [u32]) {
    unsafe {
		neon_add_4s(a, b, c);
    }
}

#[inline(always)]
pub fn add_2d(a: &[u64], b: &[u64], c: &mut [u64]) {
    unsafe {
		neon_add_2d(a, b, c);
    }
}

#[inline(always)]
pub fn fill_v0_4s(a: &[u32]) {
	unsafe {
		neon_fill_v0_4s(a);
	}
}

#[inline(always)]
pub fn mov_4s(d: u32, s: u32) {
    unsafe {
        if d == 20 && s == 1 {
            neon_mov_v20_v1_4s();
        }
        else if d == 21 && s == 0 {
            neon_mov_v21_v0_4s();
        }
        else {
            panic!("mov not Implemented!");
        }
    }
}

#[inline(always)]
pub fn fill_v1_4s(a: &[u32]) {
	unsafe {
		neon_fill_v1_4s(a);
	}
}

#[inline(always)]
pub fn fill_v2_4s(a: &[u32]) {
	unsafe {
		neon_fill_v2_4s(a);
	}
}

#[inline(always)]
pub fn cmp_v20_v0_v1_4s() {
	unsafe {
		neon_cmp_v20_v0_v1_4s();
	}
}

#[inline(always)]
pub fn read_4s(s: u8, a: &mut [u32]) {
    unsafe {
        if s == 0 {
            neon_read_v0_4s(a);
        }
        else if s == 1 {
            neon_read_v1_4s(a);
        }
        else if s == 20 {
            neon_read_v20_4s(a);
        }
        else if s == 21 {
            neon_read_v21_4s(a);
        }
        else {
            panic!("read not Implemented! {}", s);
        }
    }
}

#[inline(always)]
pub fn add_v0_v1_4s() {
    unsafe {
		neon_add_v0_v1_4s();
	}
}

#[inline(always)]
pub fn sub_v0_v2_4s() {
    unsafe {
		neon_sub_v0_v2_4s();
	}
}

#[inline(always)]
pub fn ins_1s(d: u8, s: u8, i: u8) {
    unsafe {
        if d == 0 && s == 21 && i == 0{
            neon_ins_v0S0_v21S0();
        }
		else if d == 0 && s == 21 && i == 1 {
            neon_ins_v0S1_v21S1();
        }
		else if d == 0 && s == 21 && i == 2 {
            neon_ins_v0S2_v21S2();
        }
		else if d == 0 && s == 21 && i == 3 {
            neon_ins_v0S3_v21S3();
        }
	
		else {
            panic!("ins not Implemented! {}, {}, {}", d, s, i);
        }
    }
}

