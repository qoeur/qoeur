use std::ffi::CString;

pub macro cstring($content:expr) {
  CString::new($content).unwrap().into_raw() as *const _
}
