#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
pub mod WindowsCpuInfos;

#[cfg(target_os = "linux")]
pub mod LinuxCpuInfos;