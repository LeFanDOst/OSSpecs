#![cfg(target_os = "windows")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use wmi::*;
use serde::Deserialize;

pub enum ArchitectureTypeProcessorWin32
{
	x86 = 0,
	MIPS = 1,
	Alpha = 2,
	PowerPC = 3,
	ARM = 5,
	ia64 = 6,
	x64 = 9,
	Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Win32_Processor
{
	pub AddressWidth: u16,
	pub Architecture: u16,
	pub AssetTag: String,
	pub Availability: u16,
	pub Caption: String,
	pub Characteristics: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConfigManagerErrorCode: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConfigManagerUserConfig: Option<bool>,
	pub CpuStatus: u16,
	pub CreationClassName: String,
	pub CurrentClockSpeed: u32,
	pub CurrentVoltage: u16,
	pub DataWidth: u16,
	pub Description: String,
	pub DeviceID: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ErrorCleared: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ErrorDescription: Option<String>,
	pub ExtClock: u32,
	pub Family: u16,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InstallDate: Option<datetime::WMIDateTime>,
	pub L2CacheSize: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub L2CacheSpeed: Option<u32>,
	pub L3CacheSize: u32,
	pub L3CacheSpeed: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LastErrorCode: Option<u32>,
	pub Level: u16,
	pub LoadPercentage: u16,
	pub Manufacturer: String,
	pub MaxClockSpeed: u32,
	pub Name: String,
	pub NumberOfCores: u32,
	pub NumberOfEnabledCore: u32,
	pub NumberOfLogicalProcessors: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OtherFamilyDescription: Option<String>,
	pub PartNumber: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PNPDeviceID: Option<String>,
	pub PowerManagementCapabilities: Vec<u16>,
	pub PowerManagementSupported: bool,
	pub ProcessorId: String,
	pub ProcessorType: u16,
	pub Revision: u16,
	pub Role: String,
	pub SecondLevelAddressTranslationExtensions: bool,
	pub SerialNumber: String,
	pub SocketDesignation: String,
	pub Status: String,
	pub StatusInfo: u16,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Stepping: Option<String>,
	pub SystemCreationClassName: String,
	pub SystemName: String,
	pub ThreadCount: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub UniqueId: Option<String>,
	pub UpgradeMethod: u16,
	pub Version: String,
	pub VirtualizationFirmwareEnabled: bool,
	pub VMMonitorModeExtensions: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub VoltageCaps: Option<u32>,
}

impl Win32_Processor
{
	pub fn ArchitectureToEnumType(self) -> ArchitectureTypeProcessorWin32
	{
		return match self.Architecture {
			0 => ArchitectureTypeProcessorWin32::x86,
			1 => ArchitectureTypeProcessorWin32::MIPS,
			2 => ArchitectureTypeProcessorWin32::Alpha,
			3 => ArchitectureTypeProcessorWin32::PowerPC,
			5 => ArchitectureTypeProcessorWin32::ARM,
			6 => ArchitectureTypeProcessorWin32::ia64,
			9 => ArchitectureTypeProcessorWin32::x64,
			_ => ArchitectureTypeProcessorWin32::Unknown,
		};
	}
}

pub fn getProcessorsInformationsWin32() -> Result<Vec<Win32_Processor>, std::io::Error>
{
	let com_con = match COMLibrary::new() {
		Ok(tmp) => tmp,
		_ => {
			return Err(std::io::Error::new(
			    std::io::ErrorKind::InvalidInput,
			    "ERROR : WMIError thrown (COM Connection).",
			));
		},
	};
	
	let wmi_con = match WMIConnection::new(com_con.into()) {
		Ok(tmp) => tmp,
		_ => {
			return Err(std::io::Error::new(
			    std::io::ErrorKind::InvalidInput,
			    "ERROR : WMIError thrown (WMI Connection).",
			));
		},
	};
	
	let tabProcosRes: Vec<Win32_Processor> = match wmi_con.query() {
		Ok(tmp) => tmp,
		Err(_err) => {
			return Err(std::io::Error::new(
			    std::io::ErrorKind::InvalidInput,
			    "ERROR : WMIError thrown (Processors Datas Query).",
			));
		},
	};
	
	return Ok(tabProcosRes);
}