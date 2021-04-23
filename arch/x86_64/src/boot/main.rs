use super::{cmdline, cpu};
use crate::x86kernel::{init, kernel_main};
use crate::x86include::kernel::die;
use drivers::{x86_64};

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        die();
    }

    x86_64::init();
    cmdline::cmdline_init();
    init::init();
    die()
}