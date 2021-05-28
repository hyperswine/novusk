use init::kmain::kernel_init;
use super::{KERNEL_INFO, modules};
use super::cpu::{cpu_init, ARCH, BRAND};
use super::dev::{device_init, DEVICE_INFO};
use super::io::{io_init};
use kerror::kerror;
use userspace::required::START_USERSPACE;
use crate::boot::boot::die;
use cpu::x64::{look_for_amd, look_for_intel};

use raw_cpuid::CpuId;

#[no_mangle]
pub unsafe extern "C" fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }

    cpu_init();
    kinfo!("CPU initialized");
    printk!("    CPU architecture: {:?}", ARCH);
    printk!("    CPU brand: {:?}", BRAND);

    io_init();
    kinfo!("I/O initialized");

    device_init();
    kinfo!("Device initialized");
    printk!("   Running on {}", DEVICE_INFO.device_name);

    modules::modules_init();
    kinfo!("x86 modules initialized");

    kernel_init();
    kinfo!("Novusk Initialized");

    if START_USERSPACE == false {
        printk!("\nThere is no userspace application, ending kernel...");
        printk!("   Read Documnetation/os-dev/setup.md");
        die();
    } else {
        printk!("Starting userspace...");
        userspace::init::userspace_init();
    }
}
