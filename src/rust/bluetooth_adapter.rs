use ffi;
use std::cell::Cell;
use std::error::Error;
use std::ptr;
use utils;

#[derive(Clone, Debug)]
struct IAdapter {
    adapter: Cell<*mut ffi::BluetoothAdapter>,
}

#[derive(Debug)]
pub struct Adapter {
    i: Box<IAdapter>,
}

unsafe impl Send for Adapter {}
unsafe impl Sync for Adapter {}

impl Adapter {
    #[inline(always)]
    pub fn adapter(&self) -> *mut ffi::BluetoothAdapter {
        self.i.adapter.get()
    }

    pub fn get_adapter() -> Result<Adapter, Box<Error>> {
        let adapter = unsafe { ffi::bluetooth_adapter_get_adapter() };
        check_null!(&adapter, "No adapter found!");
        Ok(Adapter {
            i: Box::new(IAdapter { adapter: Cell::new(adapter) })
        })
    }

    pub fn get_id(&self) -> String {
        "BlurdroidAdapter".to_string()
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        let address = unsafe { ffi::bluetooth_adapter_get_address(self.adapter()) };
        check_null!(&address, "No address found!");
        let res = try!(utils::convert_cstr(&address, "No address found!"));
        unsafe { ffi::jni_free_string(address) };
        Ok(res)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        let name = unsafe { ffi::bluetooth_adapter_get_name(self.adapter()) };
        check_null!(&name, "No name found!");
        let res = try!(utils::convert_cstr(&name, "No name found!"));
        unsafe { ffi::jni_free_string(name) };
        Ok(res)
    }

    pub fn get_device_list(&self) -> Result<Vec<String>, Box<Error>> {
        let mut v = vec!();
        unsafe {
            let devices = ffi::bluetooth_adapter_get_devices(self.adapter());
            check_null!(&devices, "No devices found!");
            let max = ffi::bluetooth_adapter_get_devices_size(self.adapter()) as isize;
            for i in 0..max {
                let d_ptr = *devices.offset(i);
                let d = match utils::c_str_to_slice(&d_ptr) {
                    None => continue,
                    Some(dev) => dev.to_owned(),
                };
                v.push(d.clone());
            }
            if max > 0 {
                ffi::jni_free_string_array(devices, max as i32);
            }
        }
        Ok(v)
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn set_alias(&self, _value: String) -> Result<(), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn set_powered(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn set_discoverable(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn set_pairable(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn set_pairable_timeout(&self, _value: u32) -> Result<(), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn set_discoverable_timeout(&self, _value: u32) -> Result<(), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(utils::NOT_SUPPORTED_ERROR))
    }
}

impl Clone for Adapter {
    fn clone(&self) -> Adapter {
        unsafe { ffi::bluetooth_adapter_inc_refcount(self.adapter()) };
        Adapter {
            i: self.i.clone(),
        }
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        unsafe {
            ffi::bluetooth_adapter_dec_refcount(self.adapter());
            ffi::bluetooth_adapter_free_adapter(self.adapter());
        }
    }
}
