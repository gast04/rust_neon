#[cfg(target_arch = "aarch64")] mod neon;
#[cfg(target_arch = "aarch64")] use neon::*;

pub fn add_instruction(code: *mut u32, i_off: &mut u32, inst: u32) {
	unsafe {
		*(code.offset(*i_off as isize)) = inst;
	}
	*i_off += 1;
}

#[naked]
pub fn call_code(pc: *mut u32, src: &[u32]) {

	// TODO: the jitted code need some setup code which also needs to be jitted
	unsafe {
	core::arch::asm!(r#"
		mov x13, x0
		mov x0, x1
		br x13
	"#, options(noreturn));
	}
	// no ret as the jitted code contains a ret, blr would overwrite the link
	// register and we would never return to the main function
}

pub fn create_code(neon_code: *mut u32, i_offset: &mut u32) {

    // generate neon instructions (carefull with byte order)
	add_instruction(neon_code, i_offset, get_ld1_inst(0, 0));
	add_instruction(neon_code, i_offset, get_ld1_inst(1, 0));
	add_instruction(neon_code, i_offset, get_cmeq_inst(20, 0, 1));

	// what to do on branch?

	//add_instruction(neon_code, i_offset, get_ld1_inst(2, 0));
	//add_instruction(neon_code, i_offset, get_ld1_inst(3, 0));

	// if branch
	// add v0, v0, v1 registers together
	// endif

	// how to restore non modified registers?
	// what if we dont restore it and we discard the vm and dont count
	// its execution anymore?
	// keep all sync with vm0
	add_instruction(neon_code, i_offset, get_ret_inst());
}
