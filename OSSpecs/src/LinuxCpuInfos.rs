#![cfg(target_os = "linux")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::bare_urls)]

use std::process::Command;

use std::fmt;

#[derive(Clone)]
pub struct Linux_Processors_FewDetails
{
	pub CPU: u32,
	pub NODE: u32,
	pub SOCKET: u32,
	pub CORE: u32,
	pub L1d_L1i_L2_L3: String,
	pub ONLINE: bool,
}

impl fmt::Debug for Linux_Processors_FewDetails
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
        write!(
			f,
			"[Linux_Processors_FewDetails : \n
			CPU: {:?} \n
			NODE: {:?} \n
			SOCKET: {:?} \n
			CORE: {:?} \n
			L1d:L1i:L2:L3: {:?} \n
			ONLINE: {:?} \n
			]",
			self.CPU,
			self.NODE,
			self.SOCKET,
			self.CORE,
			self.L1d_L1i_L2_L3,
			self.ONLINE,
		)
    }
}

impl Linux_Processors_FewDetails
{
	pub fn new(CPU: u32, NODE: u32, SOCKET: u32, CORE: u32, L1d_L1i_L2_L3: String, ONLINE: bool) -> Linux_Processors_FewDetails
	{
		return Linux_Processors_FewDetails {
			CPU,
			NODE,
			SOCKET,
			CORE,
			L1d_L1i_L2_L3,
			ONLINE,
		};
	}
}

pub fn getFewDetailsOnLinuxProcessors() -> Result<Vec<Linux_Processors_FewDetails>, Box<dyn std::error::Error>>
{
	let output = Command::new("sh").arg("-c").arg("lscpu -e").output()?;
	let lscpuECmd = output.stdout;
	
	let lscpuECmdStr = match std::str::from_utf8(lscpuECmd.as_slice()) {
		Ok(tmp) => tmp,
		Err(_err) => {
			return Err(Box::new(std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				"ERROR : UTF8Error thrown (Conversion Processors Datas Into String).",
			)));
		},
	};
	
	let cpuInfosEString = String::from(lscpuECmdStr);
	let mut arrRes : Vec<Linux_Processors_FewDetails> = Vec::new();
	
	let mut cmpt : i32 = 0;
	
	for line in cpuInfosEString.lines()
	{
		if cmpt > 0
		{
			let mut lineTemp = String::from(line);
			
			let mut updating = lineTemp; // Clear useless whitespaces -- Beginning.
			let mut tabUpdating : Vec<char> = Vec::new();
			
			let mut i : usize = 0;
			let sizeTemp : usize = updating.len();
			
			while i < sizeTemp
			{
				tabUpdating.push(updating.remove(0));
				
				i += 1;
			}
			
			for i in 0..tabUpdating.len()
			{
				if tabUpdating[i] == ' '
				{
					if i > 0 && tabUpdating[(i - 1)] != ' '
					{
						updating.push(tabUpdating[i]);
					}
				}
				else
				{
					updating.push(tabUpdating[i]);
				}
			}
			
			lineTemp = updating; // Clear useless whitespaces -- Ending.
			
			let arrTemp : Vec<&str> = lineTemp.split(' ').collect();
			
			arrRes.push(
				Linux_Processors_FewDetails::new(
					arrTemp[0].parse::<u32>()?,
					arrTemp[1].parse::<u32>()?,
					arrTemp[2].parse::<u32>()?,
					arrTemp[3].parse::<u32>()?,
					String::from(arrTemp[4].clone()),
					if String::from(arrTemp[5].clone()) == String::from("yes") { true } else { false },
				)
			);
		}
		
		cmpt += 1;
	}
	
	return Ok(arrRes);
}

#[derive(Clone)]
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
	pub L1d_cache: u32,
	pub L1i_cache: u32,
	pub L2_cache: u32,
	pub L3_cache: u32,
	pub NUMA_node0_CPU_s: String,
	pub FewDetails: Vec<Linux_Processors_FewDetails>,
}

impl fmt::Debug for Linux_Processor
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
        write!(
			f,
			"[Linux_Processor : \n
			Architecture: {:?} \n
			CPU op-mode(s): {:?} \n
			Byte Order: {:?} \n
			CPU(s): {:?} \n
			On-line CPU(s) list: {:?} \n
			Thread(s) per core: {:?} \n
			Core(s) per socket: {:?} \n
			Socket(s): {:?} \n
			NUMA node(s): {:?} \n
			Vendor ID: {:?} \n
			CPU family: {:?} \n
			Model: {:?} \n
			Stepping: {:?} \n
			CPU MHz: {:?} \n
			BogoMIPS: {:?} \n
			Virtualization: {:?} \n
			L1d cache: {:?} \n
			L1i cache: {:?} \n
			L2 cache: {:?} \n
			L3 cache: {:?} \n
			NUMA node0 CPU(s): {:?} \n
			Few Details: {:?} \n
			]",
			self.Architecture,
			self.CPU_op_mode_s,
			self.ByteOrder,
			self.CPU_s,
			self.On_line_CPU_s_list,
			self.Thread_s_per_core,
			self.Core_s_per_socket,
			self.Socket_s,
			self.NUMA_node_s,
			self.Vendor_ID,
			self.CPU_family,
			self.Model,
			self.Stepping,
			self.CPU_MHz,
			self.BogoMIPS,
			self.Virtualization,
			self.L1d_cache,
			self.L1i_cache,
			self.L2_cache,
			self.L3_cache,
			self.NUMA_node0_CPU_s,
			self.FewDetails,
		)
    }
}

impl Linux_Processor {
    pub fn new(
        Architecture: String,
        CPU_op_mode_s: String,
        ByteOrder: String,
        CPU_s: u32,
        On_line_CPU_s_list: String,
        Thread_s_per_core: u32,
        Core_s_per_socket: u32,
        Socket_s: u32,
        NUMA_node_s: u32,
        Vendor_ID: String,
        CPU_family: u32,
        Model: u32,
        Stepping: u32,
        CPU_MHz: f32,
        BogoMIPS: f32,
        Virtualization: String,
        L1d_cache: u32,
        L1i_cache: u32,
        L2_cache: u32,
        L3_cache: u32,
        NUMA_node0_CPU_s: String,
        FewDetails: Vec<Linux_Processors_FewDetails>,
    ) -> Linux_Processor {
        return Linux_Processor {
            Architecture,
            CPU_op_mode_s,
            ByteOrder,
            CPU_s,
            On_line_CPU_s_list,
            Thread_s_per_core,
            Core_s_per_socket,
            Socket_s,
            NUMA_node_s,
            Vendor_ID,
            CPU_family,
            Model,
            Stepping,
            CPU_MHz,
            BogoMIPS,
            Virtualization,
            L1d_cache,
            L1i_cache,
            L2_cache,
            L3_cache,
            NUMA_node0_CPU_s,
            FewDetails,
        };
    }
}

pub fn getProcessorsInformationsLinux() -> Result<Linux_Processor, Box<dyn std::error::Error>>
{
	let output = Command::new("sh").arg("-c").arg("lscpu -b").output()?;
	let lscpuCmd = output.stdout;
	
	let lscpuCmdStr = match std::str::from_utf8(lscpuCmd.as_slice()) {
		Ok(tmp) => tmp,
		Err(_err) => {
			return Err(Box::new(std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				"ERROR : UTF8Error thrown (Conversion Processors Datas Into String).",
			)));
		},
	};
	
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
		
		arrTemp.push(lineTemp);
	}
	
	return Ok(
		Linux_Processor::new(
			arrTemp[0].clone(),
			arrTemp[1].clone(),
			arrTemp[2].clone(),
			arrTemp[4].parse::<u32>()?,
			arrTemp[5].clone(),
			arrTemp[6].parse::<u32>()?,
			arrTemp[7].parse::<u32>()?,
			arrTemp[8].parse::<u32>()?,
			arrTemp[9].parse::<u32>()?,
			arrTemp[10].clone(),
			arrTemp[11].parse::<u32>()?,
			arrTemp[12].parse::<u32>()?,
			arrTemp[14].parse::<u32>()?,
			arrTemp[15].parse::<f32>()?,
			arrTemp[16].parse::<f32>()?,
			arrTemp[18].clone(),
			arrTemp[19].parse::<u32>()?,
			arrTemp[20].parse::<u32>()?,
			arrTemp[21].parse::<u32>()?,
			arrTemp[22].parse::<u32>()?,
			arrTemp[23].clone(),
			getFewDetailsOnLinuxProcessors()?,
		)
	);
}