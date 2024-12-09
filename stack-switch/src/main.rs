use core::arch::asm;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64
}

fn hello() -> ! {
    println!("Hello from Rust, and next I will continue to loop, don't expect me to return.");
    loop {}
}

unsafe fn fiber_context_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new
    );
}

fn main () {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0u8; SSIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        let sb_aligned = (stack_bottom as usize & !0xf) as *mut u8;
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;
        fiber_context_switch(&mut ctx);
    }
}
