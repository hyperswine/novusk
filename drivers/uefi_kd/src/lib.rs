#![no_std]

#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;

extern crate gop;

#[macro_use]
extern crate uefi;

pub mod init;
pub mod fs;
pub mod screen;

pub static mut UEFI_MAJOR_VERSION: u16 = 0;
pub static mut UEFI_MINOR_VERSION: u16 = 0;
