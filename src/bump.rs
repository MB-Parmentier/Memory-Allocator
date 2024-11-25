/// Structure unique représentant l'allocateur
/// heap_start et heap_end sont la plage accessible dans la heap
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

/// Implémenter une unité de la structure créée précédemment
/// Ce BumpAllocator
impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initialiser la plage mémoire sur laquelle l'allocateur peut agir.
    /// L'adresse de départ sera brk
    ///
    /// En cas d'utilisation d'une adresse mémoire invalide, erreur "invalid memory"
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}
