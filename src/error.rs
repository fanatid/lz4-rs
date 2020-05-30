use std::ffi::CStr;
use std::fmt;
use std::io;

use lz4_sys::{LZ4FErrorCode, LZ4FErrorCodes, LZ4F_getErrorCode, LZ4F_getErrorName};

#[derive(Debug)]
pub struct LZ4FError {
    code: LZ4FErrorCodes,
    name: &'static str,
}

impl LZ4FError {
    pub fn check(code: LZ4FErrorCode) -> io::Result<usize> {
        unsafe {
            match LZ4F_getErrorCode(code) {
                LZ4FErrorCodes::NoError => Ok(code as usize),
                error_code => {
                    let error_name = CStr::from_ptr(LZ4F_getErrorName(code))
                        .to_str()
                        .expect("Invalid error name C string");

                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        LZ4FError {
                            code: error_code,
                            name: error_name,
                        },
                    ))
                }
            }
        }
    }
}

impl fmt::Display for LZ4FError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LZ4F error ({}): {}", self.code as u32, self.name)
    }
}

impl std::error::Error for LZ4FError {}
