use core::arch::asm;

#[inline(never)]
fn syscall(message:String) {
    let msg_str = message.as_ptr();
    let len = message.len();
    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") msg_str,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}

fn main() {
    let message = "Hello world from Rust!\n".to_string();
    syscall(message);
}