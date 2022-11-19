use crate::client::{client::ClientCsMsg, codec, console_adaptor::ConsoleAdaptorBuilder};
use bytes::{Buf, BufMut};
use futures::StreamExt;
use gsfw::network::AgentService;
use tokio::sync::{broadcast, mpsc};
use tower::Service;

pub async fn run(gate: String, player_id: u64) -> anyhow::Result<()> {
    let (br_tx, _br_rx) = broadcast::channel(128);
    let (tx, mut rx) = mpsc::channel(128);
    let mut make_agent = AgentService::<_, _, _, _>::new(
        codec::Encoder,
        codec::Decoder::default(),
        ConsoleAdaptorBuilder::new(br_tx.clone(), tx.clone(), player_id),
    );
    let stream = std::net::TcpStream::connect(gate)?;
    let stream = tokio::net::TcpStream::from_std(stream)?;
    let fut = Service::call(&mut make_agent, stream);
    // 发送第一个消息 CsFlogin
    let _join = tokio::spawn(fut);
    let mut line_reader =
        tokio_util::codec::FramedRead::new(tokio::io::stdin(), Decoder::new(1024));
    loop {
        tokio::select! {
            Some(line) = line_reader.next() => {
                match line {
                    Ok(line) => {
                        // println!("line: {}", line);
                        // if line == "exit" {
                        //     return Ok(())
                        // }
                        br_tx.send(ClientCsMsg { ids: vec![player_id], payload: pb::CsMsg::CsEcho(pb::CsEcho {
                            content: line
                            })
                        }).unwrap();

                    },
                    Err(err) => tracing::error!("error: {:?}", err)
                }
            },
            Some(reply) = rx.recv() => {
                println!("{:?}", reply)
            }
        }
    }
}

struct Decoder {
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
