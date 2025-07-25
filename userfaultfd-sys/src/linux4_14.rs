use std::mem::transmute_copy;

use super::*;

// The following are preprocessor constants that bindgen can't figure out, so we enter them manually
// from <linux/userfaultfd.h>, and have tests to make sure they're accurate.

pub const UFFD_API: u64 = 0xAA;

pub const UFFD_API_FEATURES: u64 = UFFD_FEATURE_EVENT_FORK
    | UFFD_FEATURE_EVENT_REMAP
    | UFFD_FEATURE_EVENT_REMOVE
    | UFFD_FEATURE_EVENT_UNMAP
    | UFFD_FEATURE_MISSING_HUGETLBFS
    | UFFD_FEATURE_MISSING_SHMEM
    | UFFD_FEATURE_SIGBUS
    | UFFD_FEATURE_THREAD_ID;
pub const UFFD_API_IOCTLS: u64 = 1 << _UFFDIO_REGISTER | 1 << _UFFDIO_UNREGISTER | 1 << _UFFDIO_API;
pub const UFFD_API_RANGE_IOCTLS: u64 =
    1 << _UFFDIO_WAKE | 1 << _UFFDIO_COPY | 1 << _UFFDIO_ZEROPAGE;
pub const UFFD_API_RANGE_IOCTLS_BASIC: u64 = 1 << _UFFDIO_WAKE | 1 << _UFFDIO_COPY;

pub const UFFDIO_REGISTER_MODE_MISSING: u64 = 1 << 0;
pub const UFFDIO_REGISTER_MODE_WP: u64 = 1 << 1;

pub const UFFDIO_COPY_MODE_DONTWAKE: u64 = 1 << 0;
pub const UFFDIO_COPY_MODE_WP: u64 = 1 << 1;

pub const UFFDIO_ZEROPAGE_MODE_DONTWAKE: u64 = 1 << 0;

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
        static _const_UFFD_API: u64;
        static _const_UFFD_API_FEATURES: u64;
        static _const_UFFD_API_IOCTLS: u64;
        static _const_UFFD_API_RANGE_IOCTLS: u64;
        static _const_UFFD_API_RANGE_IOCTLS_BASIC: u64;
        static _const_UFFDIO_REGISTER_MODE_MISSING: u64;
        static _const_UFFDIO_REGISTER_MODE_WP: u64;
        static _const_UFFDIO_COPY_MODE_DONTWAKE: u64;
        static _const_UFFDIO_COPY_MODE_WP: u64;
        static _const_UFFDIO_ZEROPAGE_MODE_DONTWAKE: u64;
        static _const_UFFDIO_API: u32;
        static _const_UFFDIO_REGISTER: u32;
        static _const_UFFDIO_UNREGISTER: u32;
        static _const_UFFDIO_WAKE: u32;
        static _const_UFFDIO_COPY: u32;
        static _const_UFFDIO_ZEROPAGE: u32;
        static _const_UFFDIO_WRITEPROTECT: u32;
    }

    #[test]
    fn consts_correct() {
        unsafe {
            assert_eq!(UFFD_API, _const_UFFD_API, "UFFD_API");
            assert_eq!(
                UFFD_API_FEATURES, _const_UFFD_API_FEATURES,
                "UFFD_API_FEATURES"
            );
            assert_eq!(UFFD_API_IOCTLS, _const_UFFD_API_IOCTLS, "UFFD_API_IOCTLS");
            assert_eq!(
                UFFD_API_RANGE_IOCTLS, _const_UFFD_API_RANGE_IOCTLS,
                "UFFD_API_RANGE_IOCTLS"
            );
            assert_eq!(
                UFFD_API_RANGE_IOCTLS_BASIC, _const_UFFD_API_RANGE_IOCTLS_BASIC,
                "UFFD_API_RANGE_IOCTLS_BASIC"
            );
            assert_eq!(
                UFFDIO_REGISTER_MODE_MISSING, _const_UFFDIO_REGISTER_MODE_MISSING,
                "UFFDIO_REGISTER_MODE_MISSING"
            );
            assert_eq!(
                UFFDIO_REGISTER_MODE_WP, _const_UFFDIO_REGISTER_MODE_WP,
                "UFFDIO_REGISTER_MODE_WP"
            );
            assert_eq!(
                UFFDIO_COPY_MODE_DONTWAKE, _const_UFFDIO_COPY_MODE_DONTWAKE,
                "UFFDIO_COPY_MODE_DONTWAKE"
            );
            assert_eq!(
                UFFDIO_COPY_MODE_WP, _const_UFFDIO_COPY_MODE_WP,
                "UFFDIO_COPY_MODE_WP"
            );
            assert_eq!(
                UFFDIO_ZEROPAGE_MODE_DONTWAKE, _const_UFFDIO_ZEROPAGE_MODE_DONTWAKE,
                "UFFDIO_ZEROPAGE_MODE_DONTWAKE"
            );
            assert_eq!(UFFDIO_API, _const_UFFDIO_API, "UFFDIO_API");
            assert_eq!(UFFDIO_REGISTER, _const_UFFDIO_REGISTER, "UFFDIO_REGISTER");
            assert_eq!(
                UFFDIO_UNREGISTER, _const_UFFDIO_UNREGISTER,
                "UFFDIO_UNREGISTER"
            );
            assert_eq!(UFFDIO_WAKE, _const_UFFDIO_WAKE, "UFFDIO_WAKE");
            assert_eq!(UFFDIO_COPY, _const_UFFDIO_COPY, "UFFDIO_COPY");
            assert_eq!(UFFDIO_ZEROPAGE, _const_UFFDIO_ZEROPAGE, "UFFDIO_ZEROPAGE");
            assert_eq!(
                UFFDIO_WRITEPROTECT, _const_UFFDIO_WRITEPROTECT,
                "UFFDIO_WRITEPROTECT"
            );
        }
    }
}
