use crate::types::{StatusCode, ErrorResult};
use crate::isahc_utils::*;
use isahc::error::ErrorKind;

/// Create an `ErrorResult` from input of `ErrorKind`.
/// 
/// # Arguments
/// 
/// * `code` - `StatusCode`
/// * `kind` - `isahc::error::ErrorKind` represents error code to get the associated error string from
pub fn create_error_result_kind(code: StatusCode, kind: ErrorKind) -> Result<(), ErrorResult> {
    Err(ErrorResult { code: code as u16, msg: get_error_str(kind).to_string() })
}

/// Create an `ErrorResult` from input of string slice.
/// 
/// # Arguments
/// 
/// * `code` - `StatusCode`
/// * `msg` - message string to add as an error description
pub fn create_error_result_str(code: StatusCode, msg: &str) -> Result<(), ErrorResult> {
    Err(ErrorResult { code: code as u16, msg: msg.to_string() })
}
