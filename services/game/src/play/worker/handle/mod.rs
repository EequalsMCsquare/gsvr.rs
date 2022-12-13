use crate::play::player::Player;

impl super::Worker {
    pub fn handle_pcs(&self, player: &mut Player, req: cspb::Registry) {
        match req {
            cspb::Registry::CsEcho(req) => self.ctl_echo(player, req),
            _unexpected => tracing::error!("[worker] recv unexpected PCS. {:?}", _unexpected),
        }
    }

    fn ctl_echo(&self, p: &mut Player, req: cspb::CsEcho) {
        self.sendp(
            p.pid,
            cspb::Registry::ScEcho(cspb::ScEcho {
                reply: format!("you said: {}", req.content),
            }),
        )
    }
}
