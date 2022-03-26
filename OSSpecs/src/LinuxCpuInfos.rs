#![cfg(target_os = "linux")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

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
	pub CPU MHz: f32,
	pub BogoMIPS: f32,
	pub Virtualization: String,
	pub L1d cache: String,
	pub L1i cache: String,
	pub L2 cache: String,
	pub L3 cache: String,
	pub NUMA node0 CPU(s): String,
}