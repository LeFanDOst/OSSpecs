#![allow(non_snake_case)]

extern crate OSSpecs;

use OSSpecs::OSDetection;
#[cfg(target_os = "windows")]
use OSSpecs::WindowsCpuInfos;
#[cfg(target_os = "linux")]
use OSSpecs::LinuxCpuInfos;

fn main()
{
	if OSDetection::isLinux()
	{
		println!("We are on a Linux Distribution OS.");
		
		let procosInfos = LinuxCpuInfos::getProcessorsInformationsLinux().unwrap();
		
		println!("{:?}", procosInfos);
	}
	else if OSDetection::isWindows()
	{
		println!("We are on Windows.");
		
		/*let tabProcos = WindowsCpuInfos::getProcessorsInformationsWin32().unwrap();
		
		println!("{:?}", tabProcos);*/
	}
	else if OSDetection::isMacOS()
	{
		println!("We are on MacOS.");
	}
	else
	{
		println!("We are on an other OS. Please, check the \"OSDetection.rs\" file to see which OS can be detect with this crate.");
	}
	
    println!("Hello, world!");
}
