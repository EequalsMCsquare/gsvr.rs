use super::{player, PlayPlugin};
use anyhow::anyhow;

impl PlayPlugin {
    pub fn handle_pmsg(&self, player_id: u64, msg: pb::CsMsg) {
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
            pb::CsMsg::CsEcho(msg) => self.on_CsEcho(player_ref, msg),

            _um => Err(anyhow!(
                "[play] unhandled player message from player-{}. {:?}",
                player_id,
                _um
            )),
        } {
            tracing::error!("handle message error happend. {}", err);
        }
    }

    fn on_CsEcho(&self, player: &player::Model, msg: pb::CsEcho) -> anyhow::Result<()> {
        let resp = pb::ScMsg::ScEcho(pb::ScEcho {
            reply: format!("you said: {}", msg.content),
        });
        self.sendp(player.pid, resp);
        Ok(())
    }
}
