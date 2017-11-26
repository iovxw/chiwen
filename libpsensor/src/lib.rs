extern crate libpsensor_sys as sys;

use std::ffi::CStr;

pub struct SensorList {
    pointer: *mut *mut sys::psensor,
    inner: Vec<Sensor>,
}

impl SensorList {
    pub fn new() -> SensorList {
        let mut pointer: *mut *mut sys::psensor = std::ptr::null_mut();
        unsafe {
            sys::psensor_amd_list_append(&mut pointer, 1);
            sys::psensor_nvidia_list_append(&mut pointer, 1);
            if sys::psensor_udisks2_is_supported() {
                sys::psensor_udisks2_list_append(&mut pointer, 1);
            } else if sys::psensor_atasmart_is_supported() {
                sys::psensor_atasmart_list_append(&mut pointer, 1);
            } else {
                sys::psensor_hddtemp_list_append(&mut pointer, 1);
            }
            sys::psensor_lmsensor_list_append(&mut pointer, 1);
        }
        let len = unsafe { sys::psensor_list_size(pointer) as usize };
        let tmp: &[*mut sys::psensor] = unsafe { std::slice::from_raw_parts_mut(pointer, len) };
        let mut vec = Vec::with_capacity(len);
        for sensor in tmp {
            let p = unsafe { Sensor::from_raw(*sensor) };
            vec.push(p);
        }

        SensorList {
            inner: vec,
            pointer: pointer,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn update(&self) -> Vec<(&Sensor, f64)> {
        unsafe {
            sys::psensor_amd_list_update(self.pointer);
            sys::psensor_nvidia_list_update(self.pointer);
            if sys::psensor_udisks2_is_supported() {
                sys::psensor_udisks2_list_update(self.pointer);
            } else if sys::psensor_atasmart_is_supported() {
                sys::psensor_atasmart_list_update(self.pointer);
            } else {
                sys::psensor_hddtemp_list_update(self.pointer);
            }
            sys::psensor_lmsensor_list_update(self.pointer);
        }
        let len = self.inner.len();
        let sensors: &[*mut sys::psensor] =
            unsafe { std::slice::from_raw_parts_mut(self.pointer, len) };
        let mut r = Vec::with_capacity(len);
        for (&sensor_pointer, sensor) in sensors.iter().zip(&self.inner) {
            let value = unsafe { sys::psensor_get_current_value(sensor_pointer) };
            r.push((sensor, value));
        }
        r
    }
}

impl Drop for SensorList {
    fn drop(&mut self) {
        unsafe {
            sys::psensor_list_free(self.pointer);
        }
    }
}

#[derive(Debug)]
pub struct Sensor {
    pub name: String,
    pub id: String,
    pub chip: String,
    pub kind: SensorType,
    pub max: f64,
    pub min: f64,
}

impl Sensor {
    unsafe fn from_raw(raw: *mut sys::psensor) -> Sensor {
        let name = CStr::from_ptr((*raw).name).to_string_lossy().into_owned();
        let id = CStr::from_ptr((*raw).id).to_string_lossy().into_owned();
        let chip = CStr::from_ptr((*raw).chip).to_string_lossy().into_owned();
        let kind = match SensorType::from_raw((*raw).type_) {
            SensorType::Other { is_temp: true } if chip.contains("CPU") => SensorType::Cpu,
            SensorType::Other { is_temp: true } if chip.contains("GPU") => SensorType::Gpu,
            x => x,
        };
        let mut max = (*raw).max;
        if max == std::f64::MIN_POSITIVE {
            max = std::f64::NAN
        }
        let mut min = (*raw).min;
        if min == std::f64::MIN_POSITIVE {
            min = std::f64::NAN
        }
        Sensor {
            name,
            id,
            chip,
            kind,
            max,
            min,
        }
    }
}

impl PartialEq for Sensor {
    fn eq(&self, other: &Sensor) -> bool {
        self.id == other.id
    }
}

impl Eq for Sensor {}

impl PartialOrd for Sensor {
    fn partial_cmp(&self, other: &Sensor) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Sensor {
    fn cmp(&self, other: &Sensor) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl std::hash::Hash for Sensor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SensorType {
    Hdd,
    Cpu,
    Gpu,
    Fan,
    Other { is_temp: bool },
}

impl SensorType {
    fn from_raw(raw: std::os::raw::c_uint) -> SensorType {
        use SensorType::*;
        if raw & sys::psensor_type_SENSOR_TYPE_NVCTRL != 0 {
            if raw & sys::psensor_type_SENSOR_TYPE_TEMP != 0 {
                return Gpu;
            } else if raw & sys::psensor_type_SENSOR_TYPE_RPM != 0 {
                return Fan;
            } else if raw & sys::psensor_type_SENSOR_TYPE_GRAPHICS != 0 {
                return Other { is_temp: false }; // Graphics usage
            } else if raw & sys::psensor_type_SENSOR_TYPE_VIDEO != 0 {
                return Other { is_temp: false }; // Video usage
            } else if raw & sys::psensor_type_SENSOR_TYPE_MEMORY != 0 {
                return Other { is_temp: false }; // Memory usage
            } else if raw & sys::psensor_type_SENSOR_TYPE_PCIE != 0 {
                return Other { is_temp: false }; // PCIe usage
            }
            return Other { is_temp: false }; // NVIDIA GPU usage
        }

        if raw & sys::psensor_type_SENSOR_TYPE_ATIADL != 0 {
            if raw & sys::psensor_type_SENSOR_TYPE_TEMP != 0 {
                return Gpu;
            } else if raw & sys::psensor_type_SENSOR_TYPE_RPM != 0 {
                return Fan;
            }
            return Other { is_temp: false }; // AMD GPU Usage
        }
        if raw & sys::psensor_type_SENSOR_TYPE_HDD_TEMP == sys::psensor_type_SENSOR_TYPE_HDD_TEMP {
            return Hdd;
        }
        if raw & sys::psensor_type_SENSOR_TYPE_CPU_USAGE ==
            sys::psensor_type_SENSOR_TYPE_CPU_USAGE
        {
            return Other { is_temp: false }; // CPU Usage
        }
        if raw & sys::psensor_type_SENSOR_TYPE_RPM != 0 {
            return Fan;
        }
        if raw & sys::psensor_type_SENSOR_TYPE_CPU != 0 {
            return Cpu;
        }
        if raw & sys::psensor_type_SENSOR_TYPE_TEMP != 0 {
            return Other { is_temp: true }; // Temperature
        }
        if raw & sys::psensor_type_SENSOR_TYPE_REMOTE != 0 {
            return Other { is_temp: false }; // Remote
        }
        if raw & sys::psensor_type_SENSOR_TYPE_MEMORY != 0 {
            return Other { is_temp: false }; // Memory
        }
        Other { is_temp: false }
    }
}
