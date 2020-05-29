use ::std::os::raw::{
    c_char,
    c_int,
    c_void,
    c_long,
    c_ulong,
};
pub type pid_t = c_int;
pub type size_t = c_ulong;
pub type ssize_t = c_long;
pub type wchar_t = c_int;


pub const MAC_ATTR_IGNORER_CAT: u32 = 0x01;
pub const MAC_ATTR_IGNOREW_CAT: u32 = 0x02;
pub const MAC_ATTR_IGNOREX_CAT: u32 = 0x04;

pub const MAC_ATTR_IGNORER_LVL: u32 = 0x08;
pub const MAC_ATTR_IGNOREW_LVL: u32 = 0x10;
pub const MAC_ATTR_IGNOREX_LVL: u32 = 0x20;

// #define MAC_EMPTY(m) (((m)->lev == 0) && ((m)->cat == 0))
// #define MAC_LABEL_EMPTY(m) (MAC_EMPTY(&m->mac) && (!(m)->type))

pub const MAC_ATTR_IGNORER: u32 = MAC_ATTR_IGNORER_CAT | MAC_ATTR_IGNORER_LVL;
pub const MAC_ATTR_IGNOREW: u32 = MAC_ATTR_IGNOREW_CAT | MAC_ATTR_IGNOREW_LVL;
pub const MAC_ATTR_IGNOREX: u32 = MAC_ATTR_IGNOREX_CAT | MAC_ATTR_IGNOREX_LVL;

pub const MAC_ATTR_IGNORE: u32 = (MAC_ATTR_IGNORER) | (MAC_ATTR_IGNOREW) | (MAC_ATTR_IGNOREX);

// #define MAC_ATTR_CHECK(mac, a) ((mac).attr & MAC_ATTR_##a)

pub const MAC_MAX_LEVEL: u32 = 255;
//#define MAC_MAX_CATEGORY ((uint64_t)(-1))


// #define MAC_TYPE_VAL(v)  ((v)>>16)
// #define MAC_TYPE(v)      ((v)&0xf0000)
pub const MAC_TYPE_SUBJECT: u32 = 0x00000; // 0
pub const MAC_TYPE_OBJECT: u32 = 0x10000;  // 65536
pub const MAC_TYPE_EQU_W: u32 = MAC_TYPE_OBJECT | MAC_ATTR_IGNOREW;
pub const MAC_TYPE_EQU: u32 = MAC_TYPE_OBJECT | MAC_ATTR_IGNOREW | MAC_ATTR_IGNORER | MAC_ATTR_IGNOREX;
pub const MAC_TYPE_LOW: u32 = MAC_TYPE_OBJECT | MAC_ATTR_IGNORER | MAC_ATTR_IGNOREX;

pub const MAC_FMT_NUM: u32 = 0;
pub const MAC_FMT_TXT: u32 = 1;
pub const MAC_FMT_LEV: u32 = 2;
pub const MAC_FMT_CAT: u32 = 4;
pub const MAC_FMT_TYPE: u32 = 8;


pub type mac_lev_t = u8;
pub type mac_cat_t = u64;
pub type mac_type_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mac {
    _unused: [u8; 0],
}
pub type mac_t = *mut __mac;


extern "C" {
    pub fn mac_init(t: mac_type_t) -> mac_t;
    pub fn mac_free(mac: mac_t);

    pub fn mac_dup(mac: mac_t) -> mac_t;
    pub fn mac_copy(dst: mac_t, src: mac_t) -> c_int;

    pub fn mac_cmp(src: mac_t, dst: mac_t) -> c_int;
    pub fn mac_valid(arg1: mac_t) -> c_int;

    pub fn mac_copy_int(buf: *const c_void) -> mac_t;
    pub fn mac_copy_ext(buf: *mut c_void, mac: mac_t, size: size_t) -> ssize_t;
    pub fn mac_to_text(arg1: mac_t, size: *mut ssize_t, flags: c_int) -> *mut c_char;
    pub fn mac_to_wtext(mac: mac_t, size: *mut ssize_t, flags: c_int) -> *mut wchar_t;
    pub fn mac_from_text(mac: mac_t, text: *const c_char) -> c_int;
    pub fn mac_size(mac: mac_t) -> ssize_t;

    pub fn mac_set_lev(mac: mac_t, lev: mac_lev_t) -> c_int;
    pub fn mac_set_cat(mac: mac_t, cat: mac_cat_t) -> c_int;
    pub fn mac_set_type(mac: mac_t, t: mac_type_t) -> c_int;
    pub fn mac_set_file(filename: *const c_char, mac: mac_t) -> c_int;
    pub fn mac_set_fd(fd: c_int, mac: mac_t) -> c_int;

    pub fn mac_get_file(filename: *const c_char) -> mac_t;
    pub fn mac_empty(mac: mac_t) -> c_int;
    pub fn mac_get_fd(fd: c_int) -> mac_t;
    pub fn mac_set_pid(pid: pid_t, mac: mac_t) -> c_int;
    pub fn mac_get_pid(pid: pid_t) -> mac_t;
    pub fn mac_get_lev(mac: mac_t) -> mac_lev_t;
    pub fn mac_get_cat(mac: mac_t) -> mac_cat_t;
    pub fn mac_get_type(mac: mac_t) -> mac_type_t;

    pub fn mac_permit(mac1: mac_t, mac2: mac_t, op: c_int) -> c_int;
    pub fn mac_file_permit(mac1: mac_t, mac2: mac_t, op: c_int) -> c_int;
}
