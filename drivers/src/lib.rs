#![no_std]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;
extern crate libn;

pub mod text;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;