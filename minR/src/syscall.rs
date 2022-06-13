//! syscall handlers

#[cfg(target_arch = "aarch64")] mod aarch64;
#[cfg(target_arch = "aarch64")] use aarch64::*;

/// Standard descriptors
//const STDIN_FILENO:  i32 = 0;
const STDOUT_FILENO: i32 = 1;
//const STDERR_FILENO: i32 = 2;


#[inline(always)]
pub fn write_stdout(buf: impl AsRef<[u8]>) {
    unsafe {
        write(STDOUT_FILENO, buf.as_ref().as_ptr(), buf.as_ref().len());
    }
}

#[inline(always)]
pub fn exit(code: i32) -> ! {
    unsafe {
        exit_group(code);
    }
}

