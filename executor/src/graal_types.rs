use std::ffi::{c_char, c_int};

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type __graal_uword = u64; // Equivalent to `unsigned long long` on 64-bit systems

// Forward declaration of the graal_isolate_t structure (opaque pointer)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct graal_isolate_t;

// Forward declaration of the graal_isolatethread_t structure (opaque pointer)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct graal_isolatethread_t;

// graal_create_isolate_params_t structure
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct graal_create_isolate_params_t {
    pub version: c_int,                             // Version of this struct
    pub reserved_address_space_size: __graal_uword, // Size of address space to reserve

    // Fields introduced in version 2
    pub auxiliary_image_path: *const c_char, // Path to auxiliary image
    pub auxiliary_image_reserved_space_size: __graal_uword, // Reserved space for auxiliary image

    // Fields introduced in version 3
    pub _reserved_1: c_int,            // Internal usage
    pub _reserved_2: *mut *mut c_char, // Internal usage
    pub pkey: c_int,                   // Isolate protection key

    // Fields introduced in version 4
    pub _reserved_3: c_char, // Internal usage
    pub _reserved_4: c_char, // Internal usage

    // Fields introduced in version 5 (reserved for future use)
    pub _reserved_5: c_char, // Internal usage
}

impl graal_create_isolate_params_t {
    pub fn new() -> Self {
        Self {
            version: 5, // Ensure the version matches __graal_create_isolate_params_version
            reserved_address_space_size: 0, // Set appropriate reserved address space
            auxiliary_image_path: std::ptr::null(), // No auxiliary image path
            auxiliary_image_reserved_space_size: 0, // No reserved space for auxiliary image
            _reserved_1: 0,
            _reserved_2: std::ptr::null_mut(),
            pkey: 0, // NO_PROTECTION_DOMAIN
            _reserved_3: 0,
            _reserved_4: 0,
            _reserved_5: 0,
        }
    }
}
