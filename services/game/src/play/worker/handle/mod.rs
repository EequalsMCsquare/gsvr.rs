use crate::play::player::Player;

impl super::Worker {

    pub fn handle_pcs(&self, player: &mut Player, req: cspb::CsMsg) {
        match req {
            cspb::CsMsg::CsEcho(req) => self.ctl_echo(player, req),
            _unexpected => tracing::error!("[worker] recv unexpected PCS. {:?}", _unexpected)
        }
    }

    fn ctl_echo(&self, p: &mut Player, req: cspb::CsEcho) {
        self.sendp(
            p.pid,
            cspb::ScMsg::ScEcho(cspb::ScEcho {
                reply: format!("you said: {}", req.content),
            }),
        )
    }
}
