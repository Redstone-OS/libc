//! LibC - C Library for Redstone OS
//!
//! Fornece wrappers para syscalls do kernel.

#![no_std]

/// Syscall numbers
pub const SYS_WRITE: u64 = 1;
pub const SYS_EXIT: u64 = 60;

/// File descriptors
pub const STDOUT: u64 = 1;
pub const STDERR: u64 = 2;

/// Faz uma syscall via int 0x80
///
/// # Safety
///
/// Esta função é unsafe porque faz syscall direta ao kernel.
#[inline]
unsafe fn syscall3(num: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let ret: u64;
    core::arch::asm!(
        "int 0x80",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// Escreve dados em um file descriptor
///
/// # Arguments
///
/// * `fd` - File descriptor (1 = stdout, 2 = stderr)
/// * `buf` - Buffer com dados a escrever
///
/// # Returns
///
/// Número de bytes escritos, ou -1 em caso de erro
pub fn write(fd: u64, buf: &[u8]) -> isize {
    unsafe { syscall3(SYS_WRITE, fd, buf.as_ptr() as u64, buf.len() as u64) as isize }
}

/// Escreve uma string em stdout
pub fn print(s: &str) {
    write(STDOUT, s.as_bytes());
}

/// Escreve uma string em stdout com newline
pub fn println(s: &str) {
    print(s);
    print("\n");
}

/// Termina o processo
///
/// # Arguments
///
/// * `code` - Código de saída (0 = sucesso)
pub fn exit(code: i32) -> ! {
    unsafe {
        syscall3(SYS_EXIT, code as u64, 0, 0);
    }

    // Nunca deve chegar aqui, mas compilador exige
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Macro para print formatado (simplificado)
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        // Por enquanto, apenas strings literais
        // TODO: Implementar formatação completa
    }};
}

/// Macro para println formatado (simplificado)
#[macro_export]
macro_rules! println {
    ($lit:expr) => {
        $crate::println($lit)
    };
}
