#![allow(non_snake_case)]

extern crate OSSpecs;

use OSSpecs::OSDetection;

fn main()
{
	if OSDetection::isLinux()
	{
		println!("We are on a Linux Distribution OS.");
	}
	else
	{
		println!("We are not on a Linux Distribution OS.");
	}
	
    println!("Hello, world!");
}
