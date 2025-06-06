//! Multiboot v1 boot support for BIOS + GRUB

#![no_std]

#[repr(C, packed)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u8; 16],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    // ... continuara
}

/// Punto de entrada desde GRUB (Multiboot v1)
#[no_mangle]
pub extern "C" fn multiboot_main(_magic: u32, info_ptr: u32) -> ! {
    if _magic != 0x2BADB002 {
        panic!("Not booted with GRUB (magic incorrect)");
    }

    let info = unsafe { &*(info_ptr as *const MultibootInfo) } 

    super::stage0::init();
    super::stage1::init();
    super::stage2::launch_kernel();
}

