use std::io::{Read, Result, Write};

use t3zktls_program_core::TypedPacket;

pub struct ReplayStream {
    replay_data: Vec<TypedPacket>,
    offset: usize,
}

impl ReplayStream {
    pub fn new(data: Vec<u8>) -> Self {
        let mut offset = 0;
        let mut replay_data = Vec::new();

        while offset < data.len() {
            let typed_packet = TypedPacket::from_bytes(&data[offset..]).unwrap();

            offset += typed_packet.length();

            replay_data.push(typed_packet);
        }

        ReplayStream {
            replay_data,
            offset: 0,
        }
    }
}

impl Read for ReplayStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data = &self.replay_data[self.offset];

        match data {
            TypedPacket::Incoming(data) => {
                let length = data.len();

                buf[..length].copy_from_slice(data);

                self.offset += 1;
                Ok(length)
            }
            TypedPacket::Outgoing(_data) => {
                panic!("Outgoing data not supported");
            }
        }
    }
}

impl Write for ReplayStream {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        let data = &self.replay_data[self.offset];

        match data {
            TypedPacket::Outgoing(data) => {
                let length = data.len();
                self.offset += 1;
                Ok(length)
            }
            TypedPacket::Incoming(_data) => {
                panic!("Incoming data not supported");
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
