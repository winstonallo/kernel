#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner::test_runner)]
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use kernel::{memory, printkln};

mod panic;

entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kernel::init();

    use x86_64::{VirtAddr, structures::paging::Translate};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [0xb8000, 0x201008, 0x0100_0020_1a10, boot_info.physical_memory_offset];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        printkln!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    printkln!("42");

    kernel::hlt_loop();
}
