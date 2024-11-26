#![no_std]
#![no_main]
use core::panic::PanicInfo;
pub mod bump;
/// Ce code implante un allocateur dynamique sans la libraire standard
/// On utilise le trait Global Allocator

/// Structure représentant les parties de la heap libérées
/// Elles seront donc réutilisables pour les prochaines allocations
#[derive(Debug)]
struct Chunk {
    addr: u8,
    size: u8,
}

#[derive(Debug, Default, Copy, Clone)]
let mut freed_chunks: [Chunk; 30] = [Chunk::default(); 30];

/// Parcourir le tableau à la recherche d'un chunk adapté
/// Retourner l'adresse du chunk à réutiliser
#[no_mangle]
pub extern "C" fn fill(chunk_size: usize) -> u8 {
    let mut new_addr: u8 = 0x00; // Valeur par défaut, si elle reste
    // Alors aucun chunk ne convient et il faudra bouger brk
    for c in freed_chunks.iter() {
        if freed_chunks[c].size > chunk_size:
            // grand mais utilisable
            new_addr = freed_chunks[c].size;
        elif freed_chunks[c].size == chunk_size:
            new_addr
    }
    new_addr
}

#[no_mangle]
pub extern "C" fn dealloc() -> ! {
    // Parcourir le tableau à la recherche d'un chunk adapté
}

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
