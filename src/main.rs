#![no_std]
#![no_main]
use core::panic::PanicInfo;
/// Ce code implante un allocateur dynamique sans la libraire standard
/// On utilise le trait Global Allocator

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let result: i64;

    unsafe {
        core::arch::asm!(
            "mov rax, 12",      // Syscall brk()
            "mov rdi, 0",
            "syscall",
            "mov {}, rax",       // Stocker le résultat de rax dans la variable result
            out(reg) result,     // Associer le registre de sortie à result
            options(nostack, preserves_flags)
        );
    

    // result contient l'adresse de brk

        // Utilise la valeur de result dans un autre appel système
        core::arch::asm!(
            "mov rax, 1",        // Syscall pour write (stdout)
            "mov rdi, 1",        // Descripteur de fichier pour stdout
            "mov rsi, {0}",      // Adresse du buffer (result)
            "mov rdx, 8",        // Nombre d'octets à écrire (8 pour i64)
            "syscall",
            in(reg) &result,     // Passe l'adresse de "result" à rsi
            options(nostack, preserves_flags)
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
