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
		- Implémenter le FMT pour la structure
		- Implémenter une fonction "new" pour la structure
*/

#[derive(Clone)]
pub struct ProcessorsInfos
{
	pub Architecture: String,
	pub CPUsCount: u32,
	pub CPUFamily: u32,
}