use bytes::{Buf, BufMut, Bytes};
use gsfw::codec::{Decoder, Encoder};

/// EncoderImpl will encode a message received from
/// natsmq to bytes and send to the agent
#[derive(Default, Clone)]
pub struct EncoderImpl;

impl Encoder<Bytes> for EncoderImpl {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: Bytes,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        dst.put_u32(item.len() as u32);
        dst.put_slice(item.chunk());
        Ok(())
    }
}

/// DecoderImpl will try to decode message from raw bytes and frame them
#[derive(Default, Clone)]
pub struct DecoderImpl {
    ctx_payload_len: usize,
}

impl Decoder for DecoderImpl {
    type Item = Bytes;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.remaining() < 4 {
            return Ok(None);
        }
        self.ctx_payload_len = src.get_u32() as usize;
        if src.remaining() < self.ctx_payload_len {
            return Ok(None);
        }
        Ok(Some(src.copy_to_bytes(self.ctx_payload_len)))
    }
}
