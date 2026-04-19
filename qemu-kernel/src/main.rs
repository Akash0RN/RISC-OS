#![no_std]
#![no_main]

use core::panic::PanicInfo;

core::arch::global_asm!(include_str!("asm/boot.s"));

const UART0: *mut u8 = 0x1000_0000 as *mut u8;

fn uart_print(s: &str) {
    for byte in s.bytes() {
        unsafe { UART0.write_volatile(byte); }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart_print("RISC-V kernel booting...\n");
    uart_print("Hello from bare metal QEMU!\n");

    loop {
        unsafe { core::arch::asm!("wfi"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_print("KERNEL PANIC\n");
    loop {
        unsafe { core::arch::asm!("wfi"); }
    }
}
