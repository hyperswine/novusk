#![no_std]

extern crate alloc;
#[macro_use]
extern crate kernel;

pub mod kernfs;

pub unsafe fn fs_init() {
    kernfs::kernelfs_init();
    printk!("   Kernel fs initialized");
}