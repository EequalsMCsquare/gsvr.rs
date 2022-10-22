use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub enum PbError {
    TooLessMutRemain { required: usize, actual: usize },
    TooLess,
    UncompleteData,
    ProstEncode(prost::EncodeError),
    ProstDecode(prost::DecodeError),
    Corrupted { bytes_used: usize },
}

impl std::fmt::Display for PbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Unpack {
    pub bytes_used: usize,
    pub payload_len: usize,
    pub payload: Bytes,
}

pub struct Processor {}

impl Processor {
    pub fn pack_frame<T: prost::Message>(msg: T, dst: &mut BytesMut) -> Result<Bytes, PbError> {
        // check dst capacity
        let encoded_len = msg.encoded_len();
        let required_size = std::mem::size_of::<u16>() * 2 + encoded_len;
        let actual_size = dst.remaining_mut();
        if actual_size < required_size {
            return Err(PbError::TooLessMutRemain {
                required: required_size,
                actual: actual_size,
            });
        }
        dst.put_u16(0xFFFF);
        dst.put_u16(encoded_len as u16);
        if let Err(err) = msg.encode(dst) {
            return Err(PbError::ProstEncode(err));
        }
        return Ok(Bytes::from(dst.clone()));
    }

    pub fn unpack_frame(buf: Bytes) -> Result<Unpack, PbError> {
        // firstly try to find start identifier
        let mut bytes_used = 0usize;
        let mut mbuf = buf.clone();
        while mbuf.remaining() >= 6 {
            let ident = mbuf.get_u16();
            bytes_used += std::mem::size_of::<u16>();
            if ident == 0xFFFF {
                // get proto length
                let proto_len = mbuf.get_u16() as usize;
                bytes_used += 2;
                // check remaining >= proto_len
                if mbuf.remaining() < proto_len {
                    return Err(PbError::UncompleteData);
                }
                let mut ret = buf.clone();
                ret.advance(bytes_used);
                ret = ret.take(proto_len).into_inner();
                bytes_used += proto_len;
                return Ok(Unpack {
                    bytes_used,
                    payload_len: proto_len,
                    payload: ret,
                });
            }
        }
        return Err(PbError::Corrupted { bytes_used });
    }
}
