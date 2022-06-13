
pub fn print_array_u8(a: &[u8]) {
    for i in 0..64 {
        print!("{:#02x} ", a[i]);
        if i % 16 == 15 {
            println!("");
		}
	}
}

pub fn print_array_u32(a: &[u32], size: u32) {
    for i in 0..size {
        print!("{:#02x} ", a[i as usize]);
        if i % size == size-1 {
            println!("");
		}
	}
}

pub fn print_array_u64(a: &[u64]) {
    for i in 0..8 {
        print!("{:#02x} ", a[i]);
        if i % 2 == 1 {
            println!("");
		}
	}
}
