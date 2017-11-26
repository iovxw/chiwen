#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub const psensor_type_SENSOR_TYPE_TEMP: psensor_type = 1;
pub const psensor_type_SENSOR_TYPE_RPM: psensor_type = 2;
pub const psensor_type_SENSOR_TYPE_PERCENT: psensor_type = 4;
pub const psensor_type_SENSOR_TYPE_REMOTE: psensor_type = 8;
pub const psensor_type_SENSOR_TYPE_LMSENSOR: psensor_type = 256;
pub const psensor_type_SENSOR_TYPE_NVCTRL: psensor_type = 512;
pub const psensor_type_SENSOR_TYPE_GTOP: psensor_type = 1024;
pub const psensor_type_SENSOR_TYPE_ATIADL: psensor_type = 2048;
pub const psensor_type_SENSOR_TYPE_ATASMART: psensor_type = 4096;
pub const psensor_type_SENSOR_TYPE_HDDTEMP: psensor_type = 8192;
pub const psensor_type_SENSOR_TYPE_UDISKS2: psensor_type = 8388608;
pub const psensor_type_SENSOR_TYPE_HDD: psensor_type = 16384;
pub const psensor_type_SENSOR_TYPE_CPU: psensor_type = 32768;
pub const psensor_type_SENSOR_TYPE_GPU: psensor_type = 65536;
pub const psensor_type_SENSOR_TYPE_FAN: psensor_type = 131072;
pub const psensor_type_SENSOR_TYPE_GRAPHICS: psensor_type = 262144;
pub const psensor_type_SENSOR_TYPE_VIDEO: psensor_type = 524288;
pub const psensor_type_SENSOR_TYPE_PCIE: psensor_type = 1048576;
pub const psensor_type_SENSOR_TYPE_MEMORY: psensor_type = 2097152;
pub const psensor_type_SENSOR_TYPE_AMBIENT: psensor_type = 4194304;
pub const psensor_type_SENSOR_TYPE_HDD_TEMP: psensor_type = 16385;
pub const psensor_type_SENSOR_TYPE_CPU_USAGE: psensor_type = 32772;
pub type psensor_type = ::std::os::raw::c_uint;

pub type __time_t = ::std::os::raw::c_long;
pub type __suseconds_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct measure {
    pub value: f64,
    pub time: timeval,
}
#[test]
fn bindgen_test_layout_measure() {
    assert_eq!(
        ::std::mem::size_of::<measure>(),
        24usize,
        concat!("Size of: ", stringify!(measure))
    );
    assert_eq!(
        ::std::mem::align_of::<measure>(),
        8usize,
        concat!("Alignment of ", stringify!(measure))
    );
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct psensor {
    pub name: *mut ::std::os::raw::c_char,
    pub id: *mut ::std::os::raw::c_char,
    pub chip: *mut ::std::os::raw::c_char,
    pub values_max_length: ::std::os::raw::c_int,
    pub measures: *mut measure,
    pub type_: ::std::os::raw::c_uint,
    pub max: f64,
    pub min: f64,
    pub sess_highest: f64,
    pub sess_lowest: f64,
    pub alarm_high_threshold: f64,
    pub alarm_low_threshold: f64,
    pub alarm_raised: ::std::os::raw::c_char,
    pub cb_alarm_raised:
        ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut psensor,
                                 arg2: *mut ::std::os::raw::c_void),
        >,
    pub cb_alarm_raised_data: *mut ::std::os::raw::c_void,
    pub provider_data: *mut ::std::os::raw::c_void,
    pub provider_data_free_fct:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
#[test]
fn bindgen_test_layout_psensor() {
    assert_eq!(
        ::std::mem::size_of::<psensor>(),
        136usize,
        concat!("Size of: ", stringify!(psensor))
    );
    assert_eq!(
        ::std::mem::align_of::<psensor>(),
        8usize,
        concat!("Alignment of ", stringify!(psensor))
    );
}

extern "C" {
    pub fn psensor_create(
        id: *mut ::std::os::raw::c_char,
        name: *mut ::std::os::raw::c_char,
        chip: *mut ::std::os::raw::c_char,
        type_: ::std::os::raw::c_uint,
        values_max_length: ::std::os::raw::c_int,
    ) -> *mut psensor;
    pub fn psensor_values_resize(s: *mut psensor, new_size: ::std::os::raw::c_int);
    pub fn psensor_free(sensor: *mut psensor);
    pub fn psensor_list_free(sensors: *mut *mut psensor);
    pub fn psensor_list_size(sensors: *mut *mut psensor) -> ::std::os::raw::c_int;
    pub fn psensor_list_get_by_id(
        sensors: *mut *mut psensor,
        id: *const ::std::os::raw::c_char,
    ) -> *mut psensor;
    pub fn psensor_value_to_str(
        type_: ::std::os::raw::c_uint,
        value: f64,
        use_celsius: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn psensor_measure_to_str(
        m: *const measure,
        type_: ::std::os::raw::c_uint,
        use_celsius: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
    pub fn psensor_list_add(sensors: *mut *mut psensor, sensor: *mut psensor) -> *mut *mut psensor;
    pub fn psensor_list_append(sensors: *mut *mut *mut psensor, sensor: *mut psensor);
    pub fn psensor_list_copy(arg1: *mut *mut psensor) -> *mut *mut psensor;
    pub fn psensor_set_current_value(sensor: *mut psensor, value: f64);
    pub fn psensor_set_current_measure(sensor: *mut psensor, value: f64, tv: timeval);
    pub fn psensor_get_current_value(arg1: *const psensor) -> f64;
    pub fn psensor_get_current_measure(sensor: *mut psensor) -> *mut measure;
    pub fn psensor_type_to_str(type_: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
    pub fn psensor_type_to_unit_str(
        type_: ::std::os::raw::c_uint,
        use_celsius: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn psensor_current_value_to_str(
        arg1: *const psensor,
        arg2: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
    pub fn psensor_log_measures(sensors: *mut *mut psensor);
    pub fn psensor_amd_is_supported() -> bool;
    pub fn psensor_amd_list_update(s: *mut *mut psensor);
    pub fn psensor_amd_list_append(s: *mut *mut *mut psensor, n: ::std::os::raw::c_int);
    pub fn psensor_amd_cleanup();
    pub fn psensor_nvidia_is_supported() -> bool;
    pub fn psensor_nvidia_list_update(s: *mut *mut psensor);
    pub fn psensor_nvidia_list_append(s: *mut *mut *mut psensor, n: ::std::os::raw::c_int);
    pub fn psensor_nvidia_cleanup();
    pub fn psensor_lmsensor_is_supported() -> bool;
    pub fn psensor_lmsensor_list_update(s: *mut *mut psensor);
    pub fn psensor_lmsensor_list_append(s: *mut *mut *mut psensor, n: ::std::os::raw::c_int);
    pub fn psensor_lmsensor_cleanup();
    pub fn psensor_atasmart_is_supported() -> bool;
    pub fn psensor_atasmart_list_append(s: *mut *mut *mut psensor, n: ::std::os::raw::c_int);
    pub fn psensor_atasmart_list_update(s: *mut *mut psensor);
    pub fn psensor_hddtemp_list_append(s: *mut *mut *mut psensor, n: ::std::os::raw::c_int);
    pub fn psensor_hddtemp_list_update(s: *mut *mut psensor);
    pub fn psensor_udisks2_is_supported() -> bool;
    pub fn psensor_udisks2_list_append(s: *mut *mut *mut psensor, n: ::std::os::raw::c_int);
    pub fn psensor_udisks2_list_update(s: *mut *mut psensor);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        unsafe {
            assert!(psensor_nvidia_is_supported());
            assert!(psensor_lmsensor_is_supported());
            assert!(psensor_udisks2_is_supported());

            assert!(!psensor_amd_is_supported());
            assert!(!psensor_atasmart_is_supported());
        }
        let mut pointer: *mut *mut psensor = std::ptr::null_mut();
        unsafe {
            psensor_amd_list_append(&mut pointer, 1);
            psensor_nvidia_list_append(&mut pointer, 1);
            if psensor_udisks2_is_supported() {
                psensor_udisks2_list_append(&mut pointer, 1);
            } else if psensor_atasmart_is_supported() {
                psensor_atasmart_list_append(&mut pointer, 1);
            } else {
                psensor_hddtemp_list_append(&mut pointer, 1);
            }
            psensor_lmsensor_list_append(&mut pointer, 1);
        }
    }
}
