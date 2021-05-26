//! Implementation of Unix domain socket transport (Unix only)

use super::super::{ConnectionError, Request, Subscribe};
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::str;

/// An interprocess connection using a unix domain socket (Unix only)
pub struct Uds {
    path: String,
    read_stream: BufReader<UnixStream>,
    write_stream: UnixStream,
}

impl Uds {
    pub fn new(path: &str) -> Result<Self, ConnectionError> {
        let write_stream =
            UnixStream::connect(path).map_err(|e| ConnectionError::UdsError(e.to_string()))?;
        let read_stream = write_stream
            .try_clone()
            .map_err(|e| ConnectionError::UdsError(e.to_string()))?;
        Ok(Self {
            path: path.to_owned(),
            read_stream: BufReader::new(read_stream),
            write_stream,
        })
    }

    fn read_json(&mut self) -> Result<String, ConnectionError> {
        let mut buffer = Vec::<u8>::new();
        loop {
            let _read_bytes = self
                .read_stream
                .read_until(b'}', &mut buffer)
                .map_err(|e| ConnectionError::UdsError(e.to_string()))?;
            let utf8_slice =
                str::from_utf8(&buffer).map_err(|e| ConnectionError::UdsError(e.to_string()))?;
            if utf8_slice.matches('{').count() == utf8_slice.matches('}').count() {
                break Ok(utf8_slice.to_string());
            }
        }
    }

    fn write(&mut self, message: String) -> Result<(), ConnectionError> {
        let _write = self
            .write_stream
            .write_all(message.as_bytes())
            .map_err(|e| ConnectionError::UdsError(e.to_string()))?;
        let _flush = self
            .write_stream
            .flush()
            .map_err(|e| ConnectionError::UdsError(e.to_string()))?;
        Ok(())
    }
}

impl Request for Uds {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError> {
        let _write = self.write(cmd)?;
        self.read_json()
    }
}

impl Subscribe for Uds {
    fn read_next(&mut self) -> Result<String, ConnectionError> {
        self.read_json()
    }

    fn fork(&self) -> Result<Self, ConnectionError>
    where
        Self: Sized,
    {
        Self::new(&self.path)
    }
}

impl Drop for Uds {
    fn drop(&mut self) {
        if self.write_stream.shutdown(Shutdown::Both).is_err() {
            println!("Error while closing UDS");
        }
    }
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
                    Err(err) => panic!("{}", err),
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
