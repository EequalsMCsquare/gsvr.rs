use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use prost::Message;
use tokio_util::codec;

#[derive(Default, Clone, Copy, Debug, )]
pub struct RawDecoder {}
impl codec::Decoder for RawDecoder {
    type Item = Bytes;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut buf = Bytes::from(src.to_vec());

        if buf.len() < 4 {
            return Ok(None);
        }
        if buf.get_u16() != 0xFFFF {
            return Err(anyhow!(
                "identifier 0xFFFF should be at the first two bytes"
            ));
        }
        let payload_len = buf.get_u16() as usize;
        if buf.remaining() < payload_len {
            if src.len() < 4 + payload_len {
                src.reserve(4 + payload_len - src.len());
            }
            return Ok(None);
        }
        src.advance(4 + payload_len);
        Ok(Some(buf.take(payload_len).into_inner()))
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct RawEncoder {}
impl RawDecoder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl codec::Encoder<Bytes> for RawEncoder {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = item.len() + std::mem::size_of::<u16>() * 2;
        let left = dst.remaining_mut();
        if left < len {
            dst.reserve(len - left);
        }
        dst.put_u16(0xFFFF);
        dst.put_u16(item.len() as u16);
        dst.put_slice(item.chunk());
        Ok(())
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct CsProtoEncoder {}
impl CsProtoEncoder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl codec::Encoder<super::CsMsg> for CsProtoEncoder {
    type Error = anyhow::Error;

    fn encode(&mut self, item: super::CsMsg, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = super::CsProto {
            payload: Some(item),
        };
        dst.put_u16(0xFFFF);
        dst.put_u16(msg.encoded_len() as u16);
        msg.encode(dst).map_err(Into::into)
    }
}

pub struct CsProtoDecoder {
    raw_decoder: RawDecoder,
}
impl codec::Decoder for CsProtoDecoder {
    type Item = crate::CsMsg;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.raw_decoder.decode(src) {
            Ok(Some(buf)) => match crate::CsProto::decode(buf) {
                Ok(pb) => match pb.payload {
                    Some(payload) => return Ok(Some(payload)),
                    None => return Err(anyhow!("empty payload")),
                },
                Err(err) => Err(err.into()),
            },
            Ok(None) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct ScProtoDecoder {
    raw_decoder: RawDecoder,
}

impl ScProtoDecoder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl codec::Decoder for ScProtoDecoder {
    type Item = crate::ScMsg;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.raw_decoder.decode(src) {
            Ok(Some(buf)) => match crate::ScProto::decode(buf) {
                Ok(pb) => match pb.payload {
                    Some(payload) => return Ok(Some(payload)),
                    None => return Err(anyhow!("empty payload")),
                },
                Err(err) => Err(err.into()),
            },
            Ok(None) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
