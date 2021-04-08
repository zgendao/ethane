//! Implementation of Unix domain socket transport (Unix only)

use super::super::{ConnectionError, Request, Subscribe};
use log::{debug, error, trace};
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::str;
use thiserror::Error;

/// An interprocess connection using a unix domain socket (Unix only)
pub struct Uds {
    path: String,
    read_stream: BufReader<UnixStream>,
    write_stream: UnixStream,
}

impl Uds {
    pub fn new(path: &str) -> Result<Self, UdsError> {
        debug!("Opening connection to unix domain socket: {}", path);
        let write_stream = UnixStream::connect(path).map_err(UdsError::UdsInit)?;
        let read_stream = write_stream.try_clone().map_err(UdsError::UdsInit)?;
        Ok(Self {
            path: path.to_owned(),
            read_stream: BufReader::new(read_stream),
            write_stream,
        })
    }

    fn read_json(&mut self) -> Result<String, UdsError> {
        let mut buffer = Vec::<u8>::new();
        loop {
            let _read_bytes = self
                .read_stream
                .read_until(b'}', &mut buffer)
                .map_err(UdsError::Read)?;
            let utf8_slice = str::from_utf8(&buffer).map_err(UdsError::Utf8)?;
            if utf8_slice.matches('{').count() == utf8_slice.matches('}').count() {
                trace!("Reading from Unix domain socket: {}", utf8_slice);
                break Ok(utf8_slice.to_string());
            }
        }
    }

    fn write(&mut self, message: String) -> Result<(), UdsError> {
        trace!("Writing to Unix domain socket: {}", &message);
        let _write = self
            .write_stream
            .write_all(message.as_bytes())
            .map_err(UdsError::Write)?;
        let _flush = self.write_stream.flush().map_err(UdsError::Write)?;
        Ok(())
    }
}

impl Request for Uds {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError> {
        let _write = self.write(cmd)?;
        self.read_json().map_err(ConnectionError::UdsError)
    }
}

impl Subscribe for Uds {
    fn read_next(&mut self) -> Result<String, ConnectionError> {
        self.read_json().map_err(ConnectionError::UdsError)
    }

    fn fork(&self) -> Result<Self, ConnectionError>
    where
        Self: Sized,
    {
        Self::new(&self.path).map_err(ConnectionError::from)
    }
}

impl Drop for Uds {
    fn drop(&mut self) {
        debug!("Closing unix domain socket connection");
        let close = self.write_stream.shutdown(Shutdown::Both);
        if let Err(err) = close {
            error!("{}", err);
        }
    }
}

/// An error type collecting what can go wrong with unix domain sockets
#[derive(Debug, Error)]
pub enum UdsError {
    #[error("Unix Domain Socket Init Error: {0}")]
    UdsInit(std::io::Error),
    #[error("Unix Domain Socket Close Error: {0}")]
    UdsClose(std::io::Error),
    #[error("Unix Domain Socket Read Error: {0}")]
    Read(std::io::Error),
    #[error("Unix Domain Socket Utf8 Error: {0}")]
    Utf8(std::str::Utf8Error),
    #[error("Unix Domain Socket Write Error: {0}")]
    Write(std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;

    const TEST_IPC: &str = "/tmp/ethane_test.ipc";

    fn spawn_test_uds_server() {
        let unix_listener = UnixListener::bind(TEST_IPC).unwrap();
        std::thread::spawn(move || {
            for incoming in unix_listener.incoming() {
                match incoming {
                    Ok(mut stream) => {
                        let mut buffer = Vec::<u8>::new();
                        let mut reader = BufReader::new(&mut stream);

                        let _read = reader.read_until(b'}', &mut buffer).unwrap();
                        let _write = (&mut stream).write_all(buffer.as_slice()).unwrap();
                        let _flush = (&mut stream).flush().unwrap();
                    }
                    Err(err) => panic!(err),
                }
            }
        });
    }

    #[test]
    fn uds_connection() {
        spawn_test_uds_server();
        let message = "{\"test\": true}";
        let mut uds = Uds::new(TEST_IPC).unwrap();
        let _write = uds.write(String::from(message)).unwrap();

        let _delete_socket = std::fs::remove_file(TEST_IPC).unwrap();
        assert_eq!(uds.read_json().unwrap(), message);
    }
}
