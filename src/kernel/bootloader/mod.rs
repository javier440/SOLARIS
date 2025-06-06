//! Bootloader module for the SOLARIS kernel
//!
//! Handles the transition from BIOS/UEFI to kernel space,
//! sets up the initial environment, loads the kernel,
//! and provides early diagnostics before full initialization.

#![no_std]

pub mod stage0;  // early setup (BIOS or UEFI)
pub mod stage1;  // memory setup, GDT/IDT
pub mod stage2;  // load kernel sections, jump to entry

use core::panic::PanicInfo;

/// Entry point from assembly stub (real mode or 32-bit entry)
#[no_mangle]
pub extern "C" fn boot_main() -> ! {
    // Stage 0: Early boot (basic screen, diagnostics, CPU check)
    stage0::init();

    // Stage 1: Setup memory management (GDT, IDT, paging, stack)
    stage1::init();

    // Stage 2: Load kernel sections, prepare jump to kernel
    stage2::launch_kernel();

    loop {}
}

/// Panic handler for bootloader
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Optionally: Print panic to VGA text buffer
    loop {}
}

