//! JSON-RPC transport layer for the LSP server.
//!
//! Reads `Content-Length` framed JSON-RPC messages from a `BufRead` source
//! and writes framed responses to a `Write` sink.

#![forbid(unsafe_code)]

use std::io::{self, BufRead, Write};

/// Maximum message size the server will accept (1 MB).
const MAX_MESSAGE_BYTES: usize = 1_048_576;

/// Read a single LSP message (Content-Length framed JSON-RPC) from `input`.
///
/// Returns `None` on EOF.
pub fn read_message(input: &mut impl BufRead) -> io::Result<Option<String>> {
    // Read headers until empty line.
    let mut content_length: Option<usize> = None;
    let mut header_buf = String::new();

    loop {
        header_buf.clear();
        let n = input.read_line(&mut header_buf)?;
        if n == 0 {
            return Ok(None); // EOF
        }

        let line = header_buf.trim();
        if line.is_empty() {
            break; // End of headers.
        }

        if let Some(val) = line.strip_prefix("Content-Length:") {
            if let Ok(len) = val.trim().parse::<usize>() {
                content_length = Some(len);
            }
        }
        // Ignore other headers (e.g. Content-Type).
    }

    let length = content_length.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Missing Content-Length header")
    })?;

    if length > MAX_MESSAGE_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Message too large: {length} bytes (max {MAX_MESSAGE_BYTES})"),
        ));
    }

    let mut body = vec![0u8; length];
    input.read_exact(&mut body)?;

    String::from_utf8(body).map(Some).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Write a single LSP message (Content-Length framed JSON-RPC) to `output`.
pub fn write_message(output: &mut impl Write, body: &str) -> io::Result<()> {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    output.write_all(header.as_bytes())?;
    output.write_all(body.as_bytes())?;
    output.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_message_basic() {
        let body = r#"{"jsonrpc":"2.0","id":1}"#;
        let frame = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        let mut input = Cursor::new(frame.into_bytes());
        let msg = read_message(&mut input).unwrap().unwrap();
        assert_eq!(msg, body);
    }

    #[test]
    fn test_read_message_eof() {
        let mut input = Cursor::new(Vec::<u8>::new());
        let msg = read_message(&mut input).unwrap();
        assert!(msg.is_none());
    }

    #[test]
    fn test_read_message_too_large() {
        let frame = format!("Content-Length: {}\r\n\r\n", MAX_MESSAGE_BYTES + 1);
        let mut input = Cursor::new(frame.into_bytes());
        let err = read_message(&mut input).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_write_message_roundtrip() {
        let body = r#"{"result":"ok"}"#;
        let mut buf = Vec::new();
        write_message(&mut buf, body).unwrap();

        let mut input = Cursor::new(buf);
        let msg = read_message(&mut input).unwrap().unwrap();
        assert_eq!(msg, body);
    }

    #[test]
    fn test_read_message_missing_content_length() {
        let frame = b"X-Custom: foo\r\n\r\nhello";
        let mut input = Cursor::new(frame.to_vec());
        let err = read_message(&mut input).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
