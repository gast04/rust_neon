#![feature(aarch64_target_feature)]
#![feature(asm)]
#![feature(naked_functions)]

// READING:
// Book: ARM 64-Bit Assembly Language
//       CHAPTER 10 Advanced SIMD Instructions 


extern crate rand;

mod simd;
mod logger;

fn test_code() {
    println!("Run some neon code!");

    // create random arrays of size 64
	// 128bit registers, we fill 4 with one array
	// -> 8b:  16u8 * 4
	//	  32b: 4u32 * 4 
	//	  64b: 2u64 * 4 
    let mut src1: [u8; 64] = [0; 64];
    let mut src2: [u8; 64] = [0; 64];
    let mut dest1: [u8; 64] = [0; 64];

	let mut src3: [u32; 16] = [0; 16];
	let mut src4: [u32; 16] = [0; 16];
    let mut dest2: [u32; 16] = [0; 16];

	let mut src5: [u64; 8] = [0; 8];
	let mut src6: [u64; 8] = [0; 8];
    let mut dest3: [u64; 8] = [0; 8];

    for i in 0..64 {
        let n1: u8 = rand::random();
        let n2: u8 = rand::random();
        src1[i] = n1;
		src2[i] = n2;
    }

	for i in 0..16 {
		let n1: u32 = rand::random();
        let n2: u32 = rand::random();
        src3[i] = n1;
        src4[i] = n2;
    }

	for i in 0..8 {
		let n1: u64 = rand::random();
        let n2: u64 = rand::random();
        src5[i] = n1;
        src6[i] = n2;
    }

    println!("SRC1:");
	logger::print_array_u64(&src5);

    println!("SRC2:");
	logger::print_array_u64(&src6);
	
	simd::add_16b(&src1, &src2, &mut dest1);
	simd::add_4s(&src3, &src4, &mut dest2);
	simd::add_2d(&src5, &src6, &mut dest3);

    println!("Res:");
	logger::print_array_u64(&dest3);
}


fn main() {

	// setup memory for four 32bit-process's
	let mut s_zeros: [u32; 4] = [0; 4];
	let mut src1: [u32; 4] = [0; 4];
	let mut src2: [u32; 4] = [0; 4];
	let mut src3: [u32; 4] = [0; 4];

	for i in 0..4 {
		let n1: u32 = rand::random();
        let n2: u32 = rand::random();
        let n3: u32 = rand::random();

        src1[i] = n1 & 0x0fffffff;
        src2[i] = n2 & 0x0fffffff;
        src3[i] = n3 & 0x000fffff;

		if i == 2 || i == 0 {
			src2[i] = src1[i];
        }
    }
	
	// fill initial memory
	println!("src1:");
	logger::print_array_u32(&src1, 4);
	println!("src2:");
	logger::print_array_u32(&src2, 4);
	println!("src3:");
	logger::print_array_u32(&src3, 4);
	println!("");

	// run code vectorized here
	/*
		// sample code
		u32 a = <rand>;
		u32 b = <rand>;
		if (a == b) 
			a = 0;
		else
			a += b;
	*/

    // fill with random values
    simd::fill_v0_4s(&src1);
    simd::fill_v1_4s(&src2);
    simd::fill_v2_4s(&src3);
    simd::cmp_v20_v0_v1_4s();

    // depending on the cmp result, execute if for entries or not
    // we store the current values in an intermediate V register, and
    // after the if we store the non executed values

    // store values for non if execution
    simd::mov_4s(21, 0);

    // execute if branch, on all values
    simd::add_v0_v1_4s();
    simd::sub_v0_v2_4s();

    // if branch done, restore other non if values:
    let mut c_reg: [u32; 4] = [0; 4];
    simd::read_4s(20, &mut c_reg);

    // single set values in v register
    for i in 0..4 {
        if c_reg[i] != 0 {
            // restore value, for non taken branch
			simd::ins_1s(0, 21, i as u8);
        }
    }

    /*
    next steps:
        - flat memory layout for memory access
        - mark branch start-end with labels, to know when a VM needs
          to be woken up again
        - code cleanUp, think about Jitting as the predefined mov, add, read
          instructions for all registers are really ugly...
    */

    // read all values at once, cause print uses the NEON registers...
    let mut v20_reg: [u32; 4] = [0; 4];
    simd::read_4s(20, &mut v20_reg);
    let mut v21_reg: [u32; 4] = [0; 4];
    simd::read_4s(21, &mut v21_reg);
    let mut v0_reg: [u32; 4] = [0; 4];
    simd::read_4s(0, &mut v0_reg);

    print!("v0:  ");
    logger::print_array_u32(&v0_reg, 4);
    print!("v20: ");
    logger::print_array_u32(&v20_reg, 4);
    print!("v21: ");
    logger::print_array_u32(&v21_reg, 4);
}
