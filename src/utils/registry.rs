use anyhow::{Context, Result};
use windows::Win32::System::Registry::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_SUCCESS;
use std::ptr;

pub fn read_dword(key: HKEY, subkey: &str, value: &str) -> Result<u32> {
    unsafe {
        let mut hkey = HKEY::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegOpenKeyExW(
            key,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        
        if result != ERROR_SUCCESS {
            return Err(anyhow::anyhow!("Failed to open registry key: {}", subkey));
        }

        let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
        let mut data: u32 = 0;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let mut data_type = REG_DWORD;

        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            Some(ptr::null_mut()),
            Some(&mut data_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        if result == ERROR_SUCCESS {
            Ok(data)
        } else {
            Err(anyhow::anyhow!("Failed to read registry value: {}", value))
        }
    }
}

pub fn write_dword(key: HKEY, subkey: &str, value: &str, data: u32) -> Result<()> {
    unsafe {
        let mut hkey = HKEY::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegOpenKeyExW(
            key,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_WRITE,
            &mut hkey,
        );
        
        if result != ERROR_SUCCESS {
            return Err(anyhow::anyhow!("Failed to open registry key for writing: {}", subkey));
        }

        let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegSetValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            0,
            REG_DWORD,
            Some(&data as *const u32 as *const u8),
            std::mem::size_of::<u32>() as u32,
        );

        let _ = RegCloseKey(hkey);

        if result == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to write registry value: {}", value))
        }
    }
}

pub fn read_string(key: HKEY, subkey: &str, value: &str) -> Result<String> {
    unsafe {
        let mut hkey = HKEY::default();
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        
        let result = RegOpenKeyExW(
            key,
            PCWSTR(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        
        if result != ERROR_SUCCESS {
            return Err(anyhow::anyhow!("Failed to open registry key: {}", subkey));
        }

        let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
        let mut data_size: u32 = 0;
        let mut data_type = REG_SZ;

        // First call to get size
        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            Some(ptr::null_mut()),
            Some(&mut data_type),
            None,
            Some(&mut data_size),
        );

        if result != ERROR_SUCCESS {
            let _ = RegCloseKey(hkey);
            return Err(anyhow::anyhow!("Failed to query registry value size: {}", value));
        }

        let mut buffer: Vec<u16> = vec![0; (data_size / 2) as usize];
        
        let result = RegQueryValueExW(
            hkey,
            PCWSTR(value_wide.as_ptr()),
            Some(ptr::null_mut()),
            Some(&mut data_type),
            Some(buffer.as_mut_ptr() as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        if result == ERROR_SUCCESS {
            let s = String::from_utf16_lossy(&buffer);
            Ok(s.trim_end_matches('\0').to_string())
        } else {
            Err(anyhow::anyhow!("Failed to read registry string value: {}", value))
        }
    }
}
