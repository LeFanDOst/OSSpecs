//#![target_os("windows")]
#![cfg(target_os = "windows")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//use wmi::{COMLibrary, WMIConnection};
use wmi::*;
use serde::Deserialize;

//use std::fmt;

/*let com_con = COMLibrary::new()?;
let wmi_con = WMIConnection::new(com_con.into())?;*/

#[derive(Deserialize, Debug)]
pub struct Win32_Processor
{
	pub AddressWidth: u16,
	pub Architecture: u16,
	pub AssetTag: String,
	pub Availability: u16,
	pub Caption: String,
	pub Characteristics: u32,
	//#[serde(skip_serializing_if = "Option::is_none")]
	//pub ConfigManagerErrorCode: Option<u32>,
	//pub ConfigManagerUserConfig: bool,
	pub CpuStatus: u16,
	pub CreationClassName: String,
	pub CurrentClockSpeed: u32,
	pub CurrentVoltage: u16,
	pub DataWidth: u16,
	pub Description: String,
	pub DeviceID: String,
	//pub ErrorCleared: bool,
	//pub ErrorDescription: String,
	pub ExtClock: u32,
	pub Family: u16,
	//pub InstallDate: datetime::WMIDateTime,
	pub L2CacheSize: u32,
	//#[serde(skip_serializing_if = "Option::is_none")]
	//pub L2CacheSpeed: Option<u32>,
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

/*impl fmt::Debug for Win32_Processor
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
        /*let naive = NaiveDateTime::from_timestamp(self.time as i64, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        write!(f, "[Header : \n version {:?} \n previous block hash {:?} \n merkle root {:?} \n time {:?} \n bits {:?} \n nonce {:?} \n]", self.version, self.previous_block_hash, self.merkle_root, datetime, self.bits, self.nonce)*/
        write!(f,
		"AddressWidth : {:?} \n
Architecture : {:?} \n
AssetTag : {:?} \n
Availability : {:?} \n
Caption : {:?} \n
Characteristics : {:?} \n
ConfigManagerErrorCode : {:?} \n
ConfigManagerUserConfig : {:?} \n
CpuStatus : {:?} \n
CreationClassName : {:?} \n
CurrentClockSpeed : {:?} \n
CurrentVoltage : {:?} \n
DataWidth : {:?} \n
Description : {:?} \n
DeviceID : {:?} \n
ErrorCleared : {:?} \n
ErrorDescription : {:?} \n
ExtClock : {:?} \n
Family : {:?} \n
InstallDate : {:?} \n
L2CacheSize : {:?} \n
L2CacheSpeed : {:?} \n
L3CacheSize : {:?} \n
L3CacheSpeed : {:?} \n
LastErrorCode : {:?} \n
Level : {:?} \n
LoadPercentage : {:?} \n
Manufacturer : {:?} \n
MaxClockSpeed : {:?} \n
Name : {:?} \n
NumberOfCores : {:?} \n
NumberOfEnabledCore : {:?} \n
NumberOfLogicalProcessors : {:?} \n
OtherFamilyDescription : {:?} \n
PartNumber : {:?} \n
PNPDeviceID : {:?} \n
PowerManagementCapabilities : {:?} \n
PowerManagementSupported : {:?} \n
ProcessorId : {:?} \n
ProcessorType : {:?} \n
Revision : {:?} \n
Role : {:?} \n
SecondLevelAddressTranslationExtensions : {:?} \n
SerialNumber : {:?} \n
SocketDesignation : {:?} \n
Status : {:?} \n
StatusInfo : {:?} \n
Stepping : {:?} \n
SystemCreationClassName : {:?} \n
SystemName : {:?} \n
ThreadCount : {:?} \n
UniqueId : {:?} \n
UpgradeMethod : {:?} \n
Version : {:?} \n
VirtualizationFirmwareEnabled : {:?} \n
VMMonitorModeExtensions : {:?} \n
VoltageCaps : {:?} \n
===== \n",
			self.AddressWidth,
			self.Architecture,
			self.AssetTag,
			self.Availability,
			self.Caption,
			self.Characteristics,
			self.ConfigManagerErrorCode,
			self.ConfigManagerUserConfig,
			self.CpuStatus,
			self.CreationClassName,
			self.CurrentClockSpeed,
			self.CurrentVoltage,
			self.DataWidth,
			self.Description,
			self.DeviceID,
			self.ErrorCleared,
			self.ErrorDescription,
			self.ExtClock,
			self.Family,
			self.InstallDate,
			self.L2CacheSize,
			self.L2CacheSpeed,
			self.L3CacheSize,
			self.L3CacheSpeed,
			self.LastErrorCode,
			self.Level,
			self.LoadPercentage,
			self.Manufacturer,
			self.MaxClockSpeed,
			self.Name,
			self.NumberOfCores,
			self.NumberOfEnabledCore,
			self.NumberOfLogicalProcessors,
			self.OtherFamilyDescription,
			self.PartNumber,
			self.PNPDeviceID,
			self.PowerManagementCapabilities,
			self.PowerManagementSupported,
			self.ProcessorId,
			self.ProcessorType,
			self.Revision,
			self.Role,
			self.SecondLevelAddressTranslationExtensions,
			self.SerialNumber,
			self.SocketDesignation,
			self.Status,
			self.StatusInfo,
			self.Stepping,
			self.SystemCreationClassName,
			self.SystemName,
			self.ThreadCount,
			self.UniqueId,
			self.UpgradeMethod,
			self.Version,
			self.VirtualizationFirmwareEnabled,
			self.VMMonitorModeExtensions,
			self.VoltageCaps,
		)
    }
}*/

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
		Err(err) => {
			println!("{:?}", err);
			
			return Err(std::io::Error::new(
			    std::io::ErrorKind::InvalidInput,
			    "ERROR : WMIError thrown (Processors Datas Query).",
			));
		},
	};
	
	return Ok(tabProcosRes);
}