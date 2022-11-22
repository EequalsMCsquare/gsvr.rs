use super::{player, PlayComponent};
use anyhow::anyhow;

impl PlayComponent {
    pub fn handle_pmsg(&self, player_id: u64, msg: cspb::CsMsg) {
        let mut players = self.players.borrow_mut();
        let player_ref;
        if let Some(player_model) = players.get_player(player_id) {
            player_ref = player_model;
        } else {
            // create new player
            let new_player = player::Model::new(player_id);
            player_ref = match players.add_player(new_player) {
                Ok(p) => p,
                Err(_) => todo!(),
            };
        }
        if let Err(err) = match msg {
            cspb::CsMsg::CsEcho(msg) => self.on_CsEcho(player_ref, msg),
            _um => Err(anyhow!(
                "[play] unhandled player message from player-{}. {:?}",
                player_id,
                _um
            )),
        } {
            tracing::error!("error occur while handle player message: {}", err)
        }
    }

    #[allow(non_snake_case)]
    fn on_CsEcho(&self, player: &player::Model, msg: cspb::CsEcho) -> anyhow::Result<()> {
        let resp = cspb::ScMsg::ScEcho(cspb::ScEcho {
            reply: format!("you said: {}", msg.content),
        });
        self.sendp(player.pid, resp);
        Ok(())
    }
}
