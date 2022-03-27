#![allow(non_snake_case)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::bare_urls)]

//! Detection of which OS the code is compiled/executed.
//!
//! The functions provided by this file permit to detect on which OS the code is compiled (and after, executed).
//!
//! # Examples
//!
//! ```
//! // To detect if you are on Linux or Windows.
//! extern crate OSSpecs;
//!
//! use OSSpecs::OSDetection;
//!
//! fn main()
//! {
//! 	if OSDetection::isLinux()
//! 	{
//! 		println!("We are on a Linux Distribution OS.");
//! 	}
//! 	else if OSDetection::isWindows()
//! 	{
//! 		println!("We are on Windows.");
//! 	}
//!
//! 	println!("Hello, world!");
//! }
//! ```
//!
//! ```
//! // Alternatively to these functions, you can also use this type of code structure :
//! let machine_kind = if cfg!(unix) {
//!   "unix"
//! } else if cfg!(windows) {
//!   "windows"
//! } else {
//!   "unknown"
//! };
//!
//! println!("I'm running on a {} machine!", machine_kind);
//!
//! // (copied from https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-macro)
//! ```
//!
//! ```
//! // Also, you can use this kind of code structure :
//! use std::env;
//!
//! // ----- snip -----
//!
//! fn foo()
//! {
//! 	if env::consts::OS == "linux"
//! 	{
//! 		// If you are on Linux...
//!
//! 		// ----- snip -----
//! 	}
//! 	else if env::consts::OS == "windows"
//! 	{
//! 		// If you are on Windows...
//!
//! 		// ----- snip -----
//! 	}
//! 	else
//! 	{
//! 		// If you are on an other OS...
//!
//! 		// ----- snip -----
//! 	}
//! }
//! ```

use std::env;

/// Check if the OS is a Linux Distribution.
///
/// This function returns a boolean value which say if the OS detected is a Linux Distribution or not.
pub fn isLinux() -> bool
{
	return env::consts::OS == "linux";
}

/// Check if the OS is Windows.
///
/// This function returns a boolean value which say if the OS detected is Windows or not.
pub fn isWindows() -> bool
{
	return env::consts::OS == "windows";
}

/// Check if the OS is Android.
///
/// This function returns a boolean value which say if the OS detected is Android or not.
pub fn isAndroid() -> bool
{
	return env::consts::OS == "android";
}

/// Check if the OS is MacOS.
///
/// This function returns a boolean value which say if the OS detected is MacOS or not.
pub fn isMacOS() -> bool
{
	return env::consts::OS == "macos";
}

/// Check if the OS is iOS.
///
/// This function returns a boolean value which say if the OS detected is iOS or not.
pub fn isiOS() -> bool
{
	return env::consts::OS == "ios";
}

/// Check if the OS is FreeBSD.
///
/// This function returns a boolean value which say if the OS detected is FreeBSD or not.
pub fn isFreeBSD() -> bool
{
	return env::consts::OS == "freebsd";
}

/// Check if the OS is DragonFly.
///
/// This function returns a boolean value which say if the OS detected is DragonFly or not.
pub fn isDragonFly() -> bool
{
	return env::consts::OS == "dragonfly";
}

/// Check if the OS is NetBSD.
///
/// This function returns a boolean value which say if the OS detected is NetBSD or not.
pub fn isNetBSD() -> bool
{
	return env::consts::OS == "netbsd";
}

/// Check if the OS is OpenBSD.
///
/// This function returns a boolean value which say if the OS detected is OpenBSD or not.
pub fn isOpenBSD() -> bool
{
	return env::consts::OS == "openbsd";
}

/// Check if the OS is Solaris.
///
/// This function returns a boolean value which say if the OS detected is Solaris or not.
pub fn isSolaris() -> bool
{
	return env::consts::OS == "solaris";
}