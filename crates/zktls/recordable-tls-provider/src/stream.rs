use std::io::{Read, Result, Write};

use zktls_program_core::TypedPacket;

pub struct RecordableStream<T: Read + Write> {
    inner: T,
    data: Vec<TypedPacket>,
}

impl<T: Read + Write> RecordableStream<T> {
    pub fn new(inner: T) -> Self
    where
        Self: Sized,
    {
        RecordableStream {
            inner,
            data: Vec::new(),
        }
    }

    pub fn stream_data(self) -> Vec<TypedPacket> {
        self.data
    }
}

impl<T: Read + Write> Read for RecordableStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        if bytes_read > 0 {
            self.data
                .push(TypedPacket::new_incoming(buf[..bytes_read].to_vec()));
        }
        Ok(bytes_read)
    }
}

impl<T: Read + Write> Write for RecordableStream<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        if bytes_written > 0 {
            self.data
                .push(TypedPacket::new_outgoing(buf[..bytes_written].to_vec()));
        }
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
