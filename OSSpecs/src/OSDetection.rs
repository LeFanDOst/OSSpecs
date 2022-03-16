#![allow(non_snake_case)]

use std::env;

pub fn isLinux() -> bool
{
	return env::consts::OS == "linux";
}
