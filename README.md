# Novusk

Novusk - A kernel written in Rust inspired by Linux

# Submodules
This project makes high use of submodules

# Wanted Features
- Optimised for RISC V only. No x86 and ARM. Though ARM is of interest for now on rpi.
- Drivers targeted at specific hardware and builtin to the kernel
- Standard monolithic kernel features based on latest standards and features -> btrfs, object orientation over files, loose coupling between layers for modularity but stronger theming.

# Documentation Structure
Each rust dir has its own associated README.md detailing what the source code is about.

# Target JSON
{
"arch": "riscv64-unknown-none",
"data-layout": "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
"executables": true,
"linker-flavor": "ld.lld",
"linker": "rust-lld",
"linker-is-gnu": true,
"pre-link-args": {
"ld.lld": [
"-Tarch/riscv64/src/boot/linker.ld"
]
},
"llvm-target": "aarch64-unknown-none",
"no-compiler-rt": true,
"features": "+a53,+strict-align",
"max-atomic-width": 128,
"os": "novusk",
"panic": "abort",
"panic-strategy": "abort",
"relocation-model": "pic",
"target-c-int-width": "32",
"target-endian": "little",
"target-pointer-width": "64",
"disable-redzone": true,
"abi-blacklist": [
"stdcall",
"fastcall",
"vectorcall",
"thiscall",
"win64",
"sysv64"
]
}
