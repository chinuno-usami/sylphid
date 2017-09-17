pub mod sylphid;
use sylphid::Sylphid;
use std::mem::transmute;
use std::ffi::CStr;
use std::path::Path;

#[no_mangle]
pub extern fn fie_create() -> *mut Sylphid {
    let _sylphid = unsafe { transmute(Box::new(Sylphid::new())) };
    _sylphid
}

#[no_mangle]
pub extern fn fie_result_size(ptr: *mut Sylphid) -> u32 {
    let mut _sylphid = unsafe { &mut *ptr };
    _sylphid.result_size() as u32
}

#[no_mangle]
pub extern fn fie_result_at(ptr: *mut Sylphid, idx: u32) -> u32 {
    let mut _sylphid = unsafe { &mut *ptr };
    let vec = _sylphid.result_at(idx as usize);
    ((vec[0] as u32) << 2*8) &
    ((vec[1] as u32) << 8) &
    (vec[2] as u32)
}

#[no_mangle]
pub extern fn fie_load_from_raw(ptr: *mut Sylphid, width: u32, height: u32, buf: *const u8) {
    let mut _sylphid = unsafe { &mut *ptr };
    _sylphid.load_from_raw(width, height, buf);
}

#[no_mangle]
pub extern fn fie_load_from_file(ptr: *mut Sylphid, path: *const i8) {
    unsafe {
        let mut _sylphid = &mut *ptr;
        _sylphid.load_from_file(&Path::new(&CStr::from_ptr(path).to_string_lossy().into_owned()));
    }
}

#[no_mangle]
pub extern fn fie_loaded(ptr: *mut Sylphid) -> u8 {
    let mut _sylphid = unsafe { &mut *ptr };
    if _sylphid.loaded() {1} else {0}
}

#[no_mangle]
pub extern fn fie_run(ptr: *mut Sylphid, num: u32, iter_time: u32, min_dist: u32) {
    let mut _sylphid = unsafe { &mut *ptr };
    _sylphid.run(num, iter_time, min_dist);
}

#[no_mangle]
pub extern fn fie_destroy(ptr: *mut Sylphid) {
    let _sylphid: Box<Sylphid> = unsafe{ transmute(ptr) };
}