use ::std::os::raw::{
    c_char,
    c_int,
    c_long,
};
pub type pid_t = c_int;
pub type ssize_t = c_long;


// #define PARSEC_CAP_TO_MASK(x) (1 << (x))

pub const PARSEC_CAP_FILE_CAP: u32 = 0; // 1
pub const PARSEC_CAP_AUDIT: u32 = 1; // 2
pub const PARSEC_CAP_SETMAC: u32 = 2; // 4
pub const PARSEC_CAP_CHMAC: u32 = 3; // 8
pub const PARSEC_CAP_IGNMACLVL: u32 = 4; // 10
pub const PARSEC_CAP_IGNMACCAT: u32 = 5; // 20
pub const PARSEC_CAP_SIG: u32 = 6; // 40
pub const PARSEC_CAP_UPDATE_ATIME: u32 = 7; // 80
pub const PARSEC_CAP_PRIV_SOCK: u32 = 8; // 100
pub const PARSEC_CAP_READSEARCH: u32 = 9; // 200
pub const PARSEC_CAP_CAP: u32 = 10; // 400
pub const PARSEC_CAP_MAC_SOCK: u32 = 11; // 800

pub const PARSEC_CAP_UNSAFE_SETXATTR: u32 = 12; // 1000
pub const PARSEC_CAP_IGNMACINT: u32 = 13; // 2000
pub const PARSEC_CAP_SUMAC: u32 = 14; // 4000


// #define PARSEC_CAP_FS_MASK \
//    ( \
//        PARSEC_CAP_TO_MASK(PARSEC_CAP_FILE_CAP) | \
//        PARSEC_CAP_TO_MASK(PARSEC_CAP_CHMAC) | \
//        PARSEC_CAP_TO_MASK(PARSEC_CAP_IGNMACLVL) | \
//        PARSEC_CAP_TO_MASK(PARSEC_CAP_IGNMACCAT) | \
//        PARSEC_CAP_TO_MASK(PARSEC_CAP_UPDATE_ATIME) | \
//        PARSEC_CAP_TO_MASK(PARSEC_CAP_READSEARCH) \
//    )

// #define parsec_cap_is_fs_cap(c) (PARSEC_CAP_TO_MASK(c) & PARSEC_CAP_FS_MASK)


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cap_struct {
    _unused: [u8; 0],
}
pub type cap_t = *mut _cap_struct;

extern "C" {
    pub fn mcap_get_proc() -> cap_t;
    pub fn mcap_set_proc(cap: cap_t) -> c_int;

    pub fn mcap_from_text(text: *const c_char) -> cap_t;
    pub fn mcap_to_text(cap: cap_t, size: *mut ssize_t) -> *mut c_char;

    pub fn mcapgetp(pid: pid_t, cap_d: cap_t) -> c_int;
    pub fn mcapsetp(pid: pid_t, cap_d: cap_t) -> c_int;

    pub static mut _mcap_names: [*const c_char; 0_usize];
    pub static mut _cap_names: [*const c_char; 0_usize];
}
