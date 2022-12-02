use crate::client::{
    gclient::GClientBuilder,
    misc::ClientInfo,
};
use tokio::{sync::broadcast, task::JoinHandle};

pub struct Cmder {
    tx: broadcast::Sender<Option<cspb::CsMsg>>,
    gate: String,
    iter: usize,
    script: Vec<cspb::CsMsg>,
    clients: Vec<JoinHandle<Metrics>>,
}

#[derive(Default, Debug)]
struct Metrics {
    send_num: usize,
    recv_num: usize,
    duration: std::time::Duration,
    errs: Vec<Box<dyn std::error::Error + Send + 'static>>,
}

impl Cmder {
    pub fn new(gate: String, iter: usize, count: usize) -> Self {
        let (tx, _rx) = broadcast::channel(4096);
        let clients = (1..=count as i64)
            .map(|player_id| -> JoinHandle<Metrics> {
                let mut rx2 = tx.subscribe();
                let builder = GClientBuilder::new()
                    .gate(gate.clone())
                    .info(ClientInfo::FastLogin { player_id })
                    .build();
                tokio::spawn(async move {
                    let mut metrics = Metrics::default();
                    let mut client = builder.await.unwrap();
                    client.authenticate().await.unwrap();
                    let start = std::time::Instant::now();
                    loop {
                        tokio::select! {
                            csmsg = rx2.recv() => {
                                match csmsg {
                                    Ok(msg) => match msg {
                                        Some(msg) => {
                                            metrics.send_num += 1;
                                            if let Err(err) = client.send(msg).await {
                                                metrics.errs.push(err.into());
                                            }
                                        },
                                        None => break
                                    },
                                    Err(err) => metrics.errs.push(Box::new(err)),
                                }
                            },
                            scmsg = client.recv() => {
                                match scmsg {
                                    Ok(_) => {
                                        metrics.recv_num += 1;
                                    },
                                    Err(err) => {
                                        metrics.errs.push(err.into())
                                    },
                                }
                            }
                        }
                    }
                    metrics.duration = std::time::Instant::now() - start;
                    metrics
                })
            })
            .collect();
        Self {
            tx,
            gate,
            script: Vec::with_capacity(1024),
            iter,
            clients,
        }
    }

    pub fn add_script(&mut self, cs: cspb::CsMsg, count: usize) {
        self.script.append(&mut vec![cs; count])
    }

    async fn run(self) -> Vec<Metrics> {
        self.script.iter().for_each(|cs| {
            self.tx.send(Some(cs.clone())).unwrap();
        });
        self.tx.send(None).unwrap();
        let mut metrics = Vec::with_capacity(self.clients.len());
        for join in self.clients {
            metrics.push(join.await.unwrap())
        }
        metrics
    }
}

pub async fn run_bench(gate: String, iter: usize, client_count: usize) -> anyhow::Result<()> {
    let mut cmder = Cmder::new(gate, iter, client_count);
    cmder.add_script(
        cspb::CsMsg::CsEcho(cspb::CsEcho {
            content: "hello".to_string(),
        }),
        20,
    );
    let metrics = cmder.run().await;
    println!("{:#?}", metrics);
    Ok(())
}
