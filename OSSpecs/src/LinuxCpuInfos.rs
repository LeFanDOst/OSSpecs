#![cfg(target_os = "linux")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::process::Command;
use std::string::String;

pub struct Linux_Processor
{
	pub Architecture: String,
	pub CPU_op_mode_s: String,
	pub ByteOrder: String,
	pub CPU_s: u32,
	pub On_line_CPU_s_list: String,
	pub Thread_s_per_core: u32,
	pub Core_s_per_socket: u32,
	pub Socket_s: u32,
	pub NUMA_node_s: u32,
	pub Vendor_ID: String,
	pub CPU_family: u32,
	pub Model: u32,
	pub Stepping: u32,
	pub CPU_MHz: f32,
	pub BogoMIPS: f32,
	pub Virtualization: String,
	pub L1d_cache: String,
	pub L1i_cache: String,
	pub L2_cache: String,
	pub L3_cache: String,
	pub NUMA_node0_CPU_s: String,
}

pub fn getProcessorsInformationsLinux() -> Result<(), std::io::Error>
{
	let output = Command::new("sh").arg("-c").arg("lscpu").output()?;
	let lscpuCmd = output.stdout;
	
	let lscpuCmdStr = match std::str::from_utf8(lscpuCmd.as_slice()) {
		Ok(tmp) => tmp,
		Err(err) => {
			println!("{:?}", err);
			
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				"ERROR : UTF8Error thrown (Conversion Processors Datas Into String).",
			));
		},
	};
	
	println!("{:?}", lscpuCmdStr);
	
	let cpuInfosString = String::from(lscpuCmdStr);
	let mut arrTemp : Vec<String> = Vec::new();
	
	for line in cpuInfosString.lines()
	{
		let mut lineTemp = String::from(line);
		let mut character : char = '.';
		
		while character != ':'
		{
			character = lineTemp.remove(0);
		}
		
		lineTemp.remove(0);
		lineTemp.remove(0);
		
		while lineTemp.starts_with(" ")
		{
			lineTemp.remove(0);
		}
		
		println!("{:?}", lineTemp);
		
		arrTemp.push(lineTemp);
	}
	
	let mut res : Linux_Processor;
	
	res.Architecture = arrTemp[0].clone();
	
	return Ok(());
}