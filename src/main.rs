#![no_std]
#![no_main]
use core::panic::PanicInfo;
/// Ce code implante un allocateur dynamique sans la libraire standard
/// On utilise le trait Global Allocator

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut result: usize;
    let new_brk: usize;
    let chunk_size: usize; // à déplacer dans les arguments /!\
    chunk_size = 300; // TEST à supprimer !!!!!!!!!!!!!!!!!!!

    unsafe {
        core::arch::asm!(
            "mov rax, 0xc",      // Syscall brk()
            "mov rdi, 0",
            "syscall",
            "mov {}, rax",       // Stocker le résultat de rax dans la variable result
            out(reg) result,     // Associer le registre de sortie à result
            options(nostack, preserves_flags)
        );
        
        // result contient l'adresse de brk

        // On modifie brk pour créer une heap
        // D'abord, calculer la nouvelle adresse
        new_brk = result + chunk_size;

        core::arch::asm!(
            "mov rax, 12",       // Syscall brk
            "syscall",           // Exécuter l'appel système APRÈS l'obtention du rdi
            in("rdi") new_brk,   // Passe la valeur de new_brk directement dans rdi
            out("rax") result,   // Le résultat sera dans rax
            options(nostack, preserves_flags)
        );
        
        // Afficher brk pour voir s'il a bougé
        core::arch::asm!(
            "mov rax, 0xc",      // Syscall brk()
            "mov rdi, 0",
            "syscall",
            options(nostack, preserves_flags)
        );
    }

    // À chaque nouvelle allocation
    // On vérifie que l'adresse du futur chunk ne dépassera pas brk
    // Si le chunk dépasse, on bouge brk
    loop {}
    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
