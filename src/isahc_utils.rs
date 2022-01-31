use isahc::error::ErrorKind;

/// Get error string of `isashc::error::ErrorKind`.
///
/// # Arguments
///
/// * `e` - `isahc::error::ErrorKind`
pub fn get_error_str(e: ErrorKind) -> &'static str {
    match e {
        ErrorKind::BadClientCertificate => "A problem occurred with the local certificate",
        ErrorKind::BadServerCertificate => "The server certificate could not be validated",
        ErrorKind::ClientInitialization => "The HTTP client failed to initialize",
        ErrorKind::ConnectionFailed => "Failed to connect to the server",
        ErrorKind::InvalidContentEncoding => "The server either returned a response using an unknown or unsupported encoding format, or the response encoding was malformed",
        ErrorKind::InvalidCredentials => "Provided authentication credentials were rejected by the server",
        ErrorKind::InvalidRequest => "The request to be sent was invalid and could not be sent.",
        ErrorKind::Io => "An I/O error either sending the request or reading the response.",
        ErrorKind::NameResolution => "Failed to resolve a host name",
        ErrorKind::ProtocolViolation => "The server made an unrecoverable HTTP protocol violation",
        ErrorKind::RequestBodyNotRewindable => "Not able to rewind the body stream",
        ErrorKind::Timeout => "A request or operation took longer than the configured timeout time",
        ErrorKind::TlsEngine => "An error occurred in the secure socket engine",
        ErrorKind::TooManyRedirects => "Number of redirects hit the maximum configured amount.",
        _ => "Unknown error",
    }
}
