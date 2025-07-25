use std::mem::transmute_copy;

use super::*;

pub use linux4_14::{
    UFFDIO_API, UFFDIO_COPY, UFFDIO_COPY_MODE_DONTWAKE, UFFDIO_COPY_MODE_WP, UFFDIO_REGISTER,
    UFFDIO_REGISTER_MODE_MISSING, UFFDIO_REGISTER_MODE_WP, UFFDIO_UNREGISTER, UFFDIO_WAKE,
    UFFDIO_ZEROPAGE, UFFDIO_ZEROPAGE_MODE_DONTWAKE, UFFD_API, UFFD_API_FEATURES, UFFD_API_IOCTLS,
    UFFD_API_RANGE_IOCTLS_BASIC,
};

// The following are preprocessor constants that bindgen can't figure out, so we enter them manually
// from <linux/userfaultfd.h>, and have tests to make sure they're accurate.

pub const UFFD_API_RANGE_IOCTLS: u64 = linux4_14::UFFD_API_RANGE_IOCTLS | 1 << _UFFDIO_WRITEPROTECT;

pub const UFFDIO_WRITEPROTECT_MODE_WP: u64 = 1 << 0;
pub const UFFDIO_WRITEPROTECT_MODE_DONTWAKE: u64 = 1 << 1;

// 1:1 equivalent to _IOWR except that the third parameter become generic
const fn iowr<T>(arg1: u32, arg2: u32) -> u32 {
    unsafe { transmute_copy::<i32, u32>(&nix::request_code_readwrite!(arg1, arg2, size_of::<T>())) }
}

// 1:1 equivalent to _IOR except that the third parameter become generic
const fn ior<T>(arg1: u32, arg2: u32) -> u32 {
    unsafe { transmute_copy::<i32, u32>(&nix::request_code_read!(arg1, arg2, size_of::<T>())) }
}

pub const UFFDIO_API: u32 = iowr::<uffdio_api>(UFFDIO as u32, _UFFDIO_API as u32);
pub const UFFDIO_REGISTER: u32 = iowr::<uffdio_register>(UFFDIO as u32, _UFFDIO_REGISTER as u32);
pub const UFFDIO_UNREGISTER: u32 = ior::<uffdio_range>(UFFDIO as u32, _UFFDIO_UNREGISTER as u32);
pub const UFFDIO_WAKE: u32 = ior::<uffdio_range>(UFFDIO as u32, _UFFDIO_WAKE as u32);
pub const UFFDIO_COPY: u32 = iowr::<uffdio_copy>(UFFDIO as u32, _UFFDIO_COPY as u32);
pub const UFFDIO_ZEROPAGE: u32 = iowr::<uffdio_zeropage>(UFFDIO as u32, _UFFDIO_ZEROPAGE as u32);
pub const UFFDIO_WRITEPROTECT: u32 = iowr::<uffdio_writeprotect>(UFFDIO as u32, _UFFDIO_WRITEPROTECT as u32);

#[cfg(test)]
mod const_tests {
    use super::*;

    extern "C" {
        static _const_UFFDIO_WRITEPROTECT_MODE_WP: u64;
        static _const_UFFDIO_WRITEPROTECT_MODE_DONTWAKE: u64;
        static _const_UFFDIO_WRITEPROTECT: u32;
    }

    #[test]
    fn consts_correct() {
        unsafe {
            assert_eq!(
                UFFDIO_WRITEPROTECT_MODE_WP, _const_UFFDIO_WRITEPROTECT_MODE_WP,
                "UFFDIO_WRITEPROTECT_MODE_WP"
            );
            assert_eq!(
                UFFDIO_WRITEPROTECT_MODE_DONTWAKE, _const_UFFDIO_WRITEPROTECT_MODE_DONTWAKE,
                "UFFDIO_WRITEPROTECT_MODE_DONTWAKE"
            );
            assert_eq!(
                UFFDIO_WRITEPROTECT, _const_UFFDIO_WRITEPROTECT,
                "UFFDIO_WRITEPROTECT"
            );
        }
    }
}
