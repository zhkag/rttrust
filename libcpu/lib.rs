#![no_std]
#[cfg(all(target_arch = "arm", target_os = "none"))]
mod arm;
