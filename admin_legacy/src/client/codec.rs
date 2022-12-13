use bytes::{Buf, BufMut};
use gsfw::codec;
use gsfw::RegistryExt;

#[derive(Clone)]
pub struct Encoder;

impl codec::Encoder<cspb::Registry> for Encoder {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: cspb::Registry,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let len = item.encoded_len();
        dst.put_u32(len as u32);
        item.encode_to(dst)?;
        return Ok(());
    }
}

#[derive(Default, Clone)]
pub struct Decoder {
    ctx_payload_len: usize,
}

impl codec::Decoder for Decoder {
    type Item = cspb::Registry;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.remaining() < 4 {
            return Ok(None);
        }
        self.ctx_payload_len = src.get_u32() as usize;
        if src.remaining() < self.ctx_payload_len {
            return Ok(None);
        }
        let payload_buf = src.copy_to_bytes(self.ctx_payload_len);
        Ok(Some(cspb::Registry::decode_frame(payload_buf)?))
    }
}
