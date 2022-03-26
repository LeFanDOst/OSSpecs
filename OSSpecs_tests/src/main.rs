#![allow(non_snake_case)]

extern crate OSSpecs;

use OSSpecs::OSDetection;
#[cfg(target_os = "windows")]
use OSSpecs::WindowsCpuInfos;

fn main()
{
	if OSDetection::isLinux()
	{
		println!("We are on a Linux Distribution OS.");
	}
	else if OSDetection::isWindows()
	{
		println!("We are on Windows.");
	}
	else if OSDetection::isMacOS()
	{
		println!("We are on MacOS.");
	}
	else
	{
		println!("We are on an other OS. Please, check the \"OSDetection.rs\" file to see which OS can be detect with this crate.");
	}
	
	let tabProcos = WindowsCpuInfos::getProcessorsInformationsWin32().unwrap();
	
	println!("{:?}", tabProcos);
	
    println!("Hello, world!");
}
