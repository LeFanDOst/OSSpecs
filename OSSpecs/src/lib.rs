#![allow(non_snake_case)]

pub mod OSDetection;

#[cfg(target_os = "windows")]
pub mod WindowsCpuInfos;

#[cfg(target_os = "linux")]
pub mod LinuxCpuInfos;