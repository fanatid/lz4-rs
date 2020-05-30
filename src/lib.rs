use std::ffi::CStr;

mod error;
mod frame;

// re-export
pub use frame::LZ4FEncoder;

/// Get version as usize: `major * 100 * 100 + minor * 100 + patch`.
///
/// ```
/// println!("{}", lz4::version_number());
/// ```
pub fn version_number() -> usize {
    unsafe { lz4_sys::LZ4_versionNumber() as usize }
}

/// Get version as str: `major.minor.patch`.
///
/// ```
/// println!("{}", lz4::version_string());
/// ```
pub fn version_string() -> &'static str {
    let cstr = unsafe { CStr::from_ptr(lz4_sys::LZ4_versionString()) };
    cstr.to_str().expect("Invalid version C string")
}
