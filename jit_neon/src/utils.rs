use libc::*;
use std::{ptr, env};
use std::fs::File;
use std::io::Write;

pub fn mmap_space(size: usize) -> *mut u32 {

    let neon_code: *mut c_void;
   	unsafe {
		neon_code = libc::mmap( ptr::null_mut(), size,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            -1, 0);

        if neon_code == libc::MAP_FAILED || neon_code == ptr::null_mut() {
            panic!("Could not mmap JIT space");
        }
    }

	println!("{:?}", neon_code);
	return neon_code as *mut u32;
}

pub fn dump_code(file_name: &str, code: *mut u32, size: usize) {

    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join(file_name);
	println!("Dump code to: {:?}", temp_file);

    let mut file = File::create(temp_file).unwrap();

    unsafe {
		for i in 0..size {
			file.write(&(*code.offset(i as isize)).to_le_bytes()).unwrap();
		}
	}
}