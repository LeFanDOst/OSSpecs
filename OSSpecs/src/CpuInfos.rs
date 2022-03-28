#![allow(non_snake_case)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::bare_urls)]

//! Get the CPUs's informations.
//!
//! This file provides a structure and a function which permit to get some CPUs's informations.
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

use std::fmt;

/*#[cfg(target_os = "windows")]
pub mod WindowsCpuInfos;

#[cfg(target_os = "linux")]
pub mod LinuxCpuInfos;*/

/*
	Données nécessaires à rajouter à la structure :
		- Nombre de Threads / Processeur
		- Toutes les données similaires entre Linux et Windows
	
	À faire :
		- Fonction cross-platform (et possédant le même nom entre Windows et Linux) pour accéder à ces informations
		- Implémenter une fonction "new" pour la structure
*/

#[derive(Clone)]
pub struct ProcessorsInfos
{
	pub Architecture: String,
	pub CPUsCount: u32,
	pub CPUFamily: u32,
	pub NumberOfCoresPerCPU: Vec<u32>,
	pub TotalNumberOfThreads: u32,
	pub AvgL1CacheSize: u32,
	pub AvgL2CacheSize: u32,
	pub AvgL3CacheSize: u32,
}

impl fmt::Debug for ProcessorsInfos
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
        write!(
			f,
			"[ProcessorsInfos : \n
			Architecture: {:?} \n
			CPUsCount: {:?} \n
			CPUFamily: {:?} \n
			NumberOfCoresPerCPU: {:?} \n
			TotalNumberOfThreads: {:?} \n
			AvgL1CacheSize: {:?} \n
			AvgL2CacheSize: {:?} \n
			AvgL3CacheSize: {:?} \n
			]",
			self.Architecture,
			self.CPUsCount,
			self.CPUFamily,
			self.NumberOfCoresPerCPU,
			self.TotalNumberOfThreads,
			self.AvgL1CacheSize,
			self.AvgL2CacheSize,
			self.AvgL3CacheSize,
		)
    }
}

impl ProcessorsInfos
{
	pub fn new(
		Architecture: String,
		CPUsCount: u32,
		CPUFamily: u32,
		NumberOfCoresPerCPU: Vec<u32>,
		TotalNumberOfThreads: u32,
		AvgL1CacheSize: u32,
		AvgL2CacheSize: u32,
		AvgL3CacheSize: u32,
	) -> ProcessorsInfos {
		return ProcessorsInfos {
			Architecture,
			CPUsCount,
			CPUFamily,
			NumberOfCoresPerCPU,
			TotalNumberOfThreads,
			AvgL1CacheSize,
			AvgL2CacheSize,
			AvgL3CacheSize,
		};
	}
}