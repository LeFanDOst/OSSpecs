#![allow(non_snake_case)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::bare_urls)]

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