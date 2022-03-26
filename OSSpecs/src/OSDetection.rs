#![allow(non_snake_case)]

use std::env;

/*
Alternatively to these functions, you can also use this type of code structure :


let machine_kind = if cfg!(unix) {
  "unix"
} else if cfg!(windows) {
  "windows"
} else {
  "unknown"
};

println!("I'm running on a {} machine!", machine_kind);

(copied from https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-macro)
*/

pub fn isLinux() -> bool
{
	return env::consts::OS == "linux";
}

pub fn isWindows() -> bool
{
	return env::consts::OS == "windows";
}

pub fn isAndroid() -> bool
{
	return env::consts::OS == "android";
}

pub fn isMacOS() -> bool
{
	return env::consts::OS == "macos";
}

pub fn isiOS() -> bool
{
	return env::consts::OS == "ios";
}

pub fn isFreeBSD() -> bool
{
	return env::consts::OS == "freebsd";
}

pub fn isDragonFly() -> bool
{
	return env::consts::OS == "dragonfly";
}

pub fn isNetBSD() -> bool
{
	return env::consts::OS == "netbsd";
}

pub fn isOpenBSD() -> bool
{
	return env::consts::OS == "openbsd";
}

pub fn isSolaris() -> bool
{
	return env::consts::OS == "solaris";
}
