use bytes::{Buf, BufMut};

pub struct Decoder {
    buf: bytes::BytesMut,
}
impl Decoder {
    pub fn new(buf_size: usize) -> Self {
        Self {
            buf: bytes::BytesMut::with_capacity(buf_size),
        }
    }
}

impl gsfw::codec::Decoder for Decoder {
    type Item = String;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        while src.has_remaining() {
            let char = src.get_u8();
            if char == '\n' as u8 {
                let freeze = self.buf.copy_to_bytes(self.buf.len());
                return Ok(Some(String::from_utf8(freeze.to_vec())?));
            } else {
                self.buf.put_u8(char);
            }
        }
        Ok(None)
    }
}
