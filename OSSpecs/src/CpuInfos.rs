#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
pub mod WindowsCpuInfos;

#[cfg(target_os = "linux")]
pub mod LinuxCpuInfos;

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