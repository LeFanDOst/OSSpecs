#![allow(non_snake_case)]

//! OSSpecs, Operating System Specifications to cross-platform in Rust.
//!
//! This crate provides some functionalities provided by OS, but on different forms according to the OS.
//!
//!
//!
//!
//!

pub mod OSDetection;

#[cfg(target_os = "windows")]
pub mod WindowsCpuInfos;

#[cfg(target_os = "linux")]
pub mod LinuxCpuInfos;

pub mod CpuInfos;