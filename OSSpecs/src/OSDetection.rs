#![allow(non_snake_case)]

use std::env;

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
