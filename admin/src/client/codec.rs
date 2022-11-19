use anyhow::anyhow;
use bytes::{Buf, BufMut};
use gsfw::codec;
use pb::Message;

#[derive(Clone)]
pub struct Encoder;

impl codec::Encoder<pb::CsMsg> for Encoder {
    type Error = anyhow::Error;

    fn encode(&mut self, item: pb::CsMsg, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let msg = pb::CsProto {
            payload: Some(item),
        };
        dst.put_u32(msg.encoded_len() as u32);
        msg.encode(dst)?;
        return Ok(());
    }
}

#[derive(Default, Clone)]
pub struct Decoder {
    ctx_payload_len: usize,
}

impl codec::Decoder for Decoder {
    type Item = pb::ScMsg;

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
        let msg = pb::ScProto::decode(payload_buf)?;
        match msg.payload {
            Some(payload) => Ok(Some(payload)),
            None => Err(anyhow!("message payload is None")),
        }
    }
}
