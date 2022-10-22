use crossbeam::queue::ArrayQueue;

#[derive(Debug)]
pub struct Histroy {
    pub cs: ArrayQueue<pb::CsMsg>,
    pub sc: ArrayQueue<pb::ScMsg>,
}

impl Histroy {
    pub fn new() -> Self {
        Self {
            cs: ArrayQueue::new(512),
            sc: ArrayQueue::new(512),
        }
    }
}
