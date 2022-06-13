#![feature(aarch64_target_feature)]
#![feature(asm)]
#![feature(naked_functions)]

extern crate rand;

mod utils;
mod code_gen;

fn main() {
    println!("Staring NEON jitter...");

    let size = 1024 * 1024;
	let neon_code = utils::mmap_space(size);
	println!("{:?}", neon_code);

	// generate neon instructions (carefull with byte order)
    let mut i_offset: u32 = 0;
    code_gen::create_code(neon_code, &mut i_offset);

	// dump all code to a file
	utils::dump_code("code.dump", neon_code, i_offset as usize);

	// create random source data
	let mut src1: [u32; 4] = [0; 4];
	for i in 0..4 {
		let n1: u32 = rand::random();
        src1[i] = n1 & 0x0fffffff;
		print!("{:#02x} ", src1[i]);
    }
	println!("");

	// call code, using naked function
	code_gen::call_code(neon_code, &src1);

	println!("end of fancy neon jitter");
}
