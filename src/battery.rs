use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    DIGCF_ALLCLASSES, DIGCF_PRESENT, HDEVINFO, SP_DEVINFO_DATA, SetupDiDestroyDeviceInfoList,
    SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceInstanceIdW,
    SetupDiGetDevicePropertyW,
};
use windows::Win32::Devices::Properties::{DEVPROP_TYPE_BYTE, DEVPROP_TYPE_STRING, DEVPROPTYPE};
use windows::Win32::Foundation::DEVPROPKEY;
use windows::core::GUID;

const DEVPKEY_DEVICE_BATTERY_PERCENT: DEVPROPKEY = DEVPROPKEY {
    fmtid: GUID::from_u128(0x104ea319_6ee2_4701_bd47_8ddbf425bbe5),
    pid: 2,
};

const DEVPKEY_DEVICE_FRIENDLY_NAME: DEVPROPKEY = DEVPROPKEY {
    fmtid: GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 14,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub battery_percent: u8,
}

pub fn fetch_connected_devices() -> Vec<DeviceInfo> {
    let mut result = Vec::new();

    unsafe {
        let dev_info_set =
            match SetupDiGetClassDevsW(None, None, None, DIGCF_PRESENT | DIGCF_ALLCLASSES) {
                Ok(set) => set,
                Err(_) => return result,
            };

        let mut index = 0;
        loop {
            let mut dev_info_data = SP_DEVINFO_DATA {
                cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
                ..Default::default()
            };

            if SetupDiEnumDeviceInfo(dev_info_set, index, &mut dev_info_data).is_err() {
                break;
            }
            index += 1;

            let Some(battery_percent) = get_device_battery(dev_info_set, &dev_info_data) else {
                continue;
            };

            let Some(id) = get_device_instance_id(dev_info_set, &dev_info_data) else {
                continue;
            };

            let name = get_device_name(dev_info_set, &dev_info_data).unwrap_or_default();

            if !result.iter().any(|d: &DeviceInfo| d.id == id) {
                result.push(DeviceInfo {
                    id,
                    name,
                    battery_percent: battery_percent.min(100),
                });
            }
        }

        let _ = SetupDiDestroyDeviceInfoList(dev_info_set);
    }

    result
}

unsafe fn get_device_battery(
    dev_info_set: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Option<u8> {
    let mut prop_type = DEVPROPTYPE::default();
    let mut buffer = [0u8; 4];
    let mut required_size = 0u32;

    let status = unsafe {
        SetupDiGetDevicePropertyW(
            dev_info_set,
            dev_info_data,
            &DEVPKEY_DEVICE_BATTERY_PERCENT,
            &mut prop_type,
            Some(&mut buffer),
            Some(&mut required_size),
            0,
        )
    };

    let _ = required_size;

    if status.is_ok() && prop_type == DEVPROP_TYPE_BYTE {
        Some(buffer[0])
    } else {
        None
    }
}

unsafe fn get_device_name(
    dev_info_set: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Option<String> {
    let mut prop_type = DEVPROPTYPE::default();
    let mut buffer = [0u16; 256];
    let mut required_size = 0u32;

    let status = unsafe {
        let buffer_bytes = std::slice::from_raw_parts_mut(
            buffer.as_mut_ptr() as *mut u8,
            buffer.len() * std::mem::size_of::<u16>(),
        );

        SetupDiGetDevicePropertyW(
            dev_info_set,
            dev_info_data,
            &DEVPKEY_DEVICE_FRIENDLY_NAME,
            &mut prop_type,
            Some(buffer_bytes),
            Some(&mut required_size),
            0,
        )
    };

    let _ = required_size;

    if status.is_ok() && prop_type == DEVPROP_TYPE_STRING {
        let trimmed = buffer.split(|&c| c == 0).next().unwrap_or(&[]);
        if !trimmed.is_empty() {
            let name = OsString::from_wide(trimmed).to_string_lossy().into_owned();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

unsafe fn get_device_instance_id(
    dev_info_set: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Option<String> {
    let mut buffer = [0u16; 256];
    let mut required_size = 0u32;

    let ok = unsafe {
        SetupDiGetDeviceInstanceIdW(
            dev_info_set,
            dev_info_data,
            Some(&mut buffer),
            Some(&mut required_size),
        )
        .is_ok()
    };

    if ok {
        let trimmed = buffer.split(|&c| c == 0).next().unwrap_or(&[]);
        let id = OsString::from_wide(trimmed).to_string_lossy().into_owned();
        if !id.is_empty() {
            return Some(id);
        }
    }
    None
}
