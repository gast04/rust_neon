

pub fn get_ld1_inst(dv: u32, sr: u32) -> u32 {
	// dv: destination neon register
	// sr: source arm register address

	// NOTE: this function has not been fully tested with all reg numbers

	// bits[0:5] v register number
	// bits[5:8] x register number

	// ld1 {v0.4s}, [x0], 0x10
	// adds 0x10 after reading to source register
	//let neon_ld1_raw: u32 = 0x4cdf7800;

	// ld1 {v0.4s}, [x0]
	// without increasing source register after read
	let neon_ld1_raw: u32 = 0x4c407800;
	// u32 neon_ld1_raw =   0x4cdf7801; // ld1 {v1.4s}, [x0], 0x10
	let inst = neon_ld1_raw | (sr<<5) | dv;
	return inst;
}

pub fn get_ret_inst() -> u32 {
	let ret: u32 = 0xd65f03c0;// 0xc0035fd6;
	return ret;
}

pub fn get_cmeq_inst(dv: u32, cv1: u32, cv2: u32) -> u32 {
	/*
      008ca06e cmeq v0.4s, v0.4s, v0.4s
      008ca16e cmeq v0.4s, v0.4s, v1.4s
      208ca26e cmeq v0.4s, v1.4s, v2.4s
      418ca36e cmeq v1.4s, v2.4s, v3.4s
      148ca16e cmeq v20.4s, v0.4s, v1.4s
	*/

	let cmeq_raw: u32 = 0x6ea08c00;
	let inst = cmeq_raw | (cv2 << 16) | (cv1 <<5) | dv; 
	return inst;
}