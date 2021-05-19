use super::{cpu, main::bmain};
use super::boot::{die};
use core::fmt::Write;
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};
use uefi_services;
use uefi::proto::console::text::Output;
use uefi_kd::{UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION};
use crate::drivers::uefi_init;
use crate::kernel::{printk, userspace::early};

unsafe fn print_early_info() {
    info!("UEFI Version: {}.{}", UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION);
}

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    early::early_user_init();
    printk::printk_init();

    uefi_services::init(&system_table).expect_success("Couldn't initialize UEFI services");

    system_table.stdout()
        .reset(false)
        .expect_success("Failed to reset UEFI stdout");
    kinfo!("Kernel printing initialized");
    kinfo!("Early ueserspace initialized");
    kinfo!("UEFI services initialized");

    uefi_init(image, system_table);
    kinfo!("Finished UEFI drivers initialization");
    print_early_info();
    bmain();
}
