//! UEFI boot entry for SOLARIS kernel

#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi::table::boot::{MemoryDescriptor, MemoryType};

#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // Imprime algo en pantalla UEFI
    uefi_services::init(&system_table).unwrap();
    let _ = system_table.stdout().reset(false);
    let _ = system_table.stdout().write_str("Booting SOLARIS via UEFI...\n");

    // Obtiene mapa de memoria (puede usarse para inicializar paginacion)
    let bt = system_table.boot_services();
    let mmap_size = bt.memory_map_size();
    let mut mmap_buf = [0u8; 4096 * 4];
    let (_key, desc_iter) = bt.memory_map(&mut mmap_buf)
        .expect("Failed to get UEFI memory map");

    for desc in desc_iter {
        let _phys = desc.phys_start;
        let _ty = desc.ty;
        let _pages = desc.page_count;
    }

    // Llamamos al inicio del kernel
    unsafe { super::stage2::launch_kernel(); }
}

