
#[naked]
pub(super) unsafe extern fn write(fd: i32, buf: *const u8, size: usize) -> isize {
    core::arch::asm!(r#"
        mov x8, 0x40 // SYS_write
        svc 0
        ret
    "#, options(noreturn));
}

#[naked]
pub(super) unsafe extern fn read(fd: i32, buf: *const u8, size: usize)
        -> isize {
    core::arch::asm!(r#"
        mov x8, 0xXX // SYS_read
        svc 0
        ret
    "#, options(noreturn));
}

#[naked] // we rely on the calling convention, so no inlining
pub(super) unsafe extern fn exit_group(code: i32) -> ! {
    core::arch::asm!(r#"
        mov x8, 0x5e // SYS_exit
        svc 0
    "#, options(noreturn));
}

