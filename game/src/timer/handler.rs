use gsfw::chanrpc::broker::Broker;

use crate::hub::{ChanProto, ModuleName, TimerArgs};
use super::{TimerComponent, TimerMeta};

#[allow(non_snake_case)]
impl TimerComponent {
    pub(super) async fn on_TimerTrigger(&mut self) {
        tracing::debug!("{:?} timer trigger", self.curr);
        let cast_tx;
        let ntf;
        {
            let mut curr = self.curr.write();
            let curr_ref = curr.as_ref().unwrap();
            cast_tx = Broker::cast_tx(&self.hub, curr_ref.from);
            ntf = ChanProto::TimerTriggerNtf {
                typ: curr_ref.typ,
                args: curr_ref.args.clone(),
            };
            // pop
            let mut timers = self.timers.write();
            if let Some(new_curr) = timers.pop_front() {
                let new_deadline = new_curr.deadline.clone();
                // timer meta updated
                tracing::debug!("current updated to {:?}", new_curr);
                curr.replace(new_curr);
                let tx = self.timer_tx.clone();
                self.curr_handle = tokio::spawn(async move {
                    tokio::time::sleep_until(tokio::time::Instant::from_std(new_deadline)).await;
                    if let Err(err) = tx.send(()).await {
                        tracing::error!("mpsc send error: {:?}", err);
                    }
                });
            }
        } // curr & timers RwLock drop
        cast_tx.cast(ntf).await;
    }

    pub(super) fn on_NewTimerReq(
        &mut self,
        from: ModuleName,
        typ: u16,
        deadline: std::time::Instant,
        args: TimerArgs,
    ) {
        let need_refresh;
        let mut curr_timer = self.curr.write();
        if curr_timer.is_none() {
            curr_timer.replace(TimerMeta {
                typ,
                save: false,
                from,
                deadline,
                args,
            });
            need_refresh = true;
        } else {
            let mut timers = self.timers.write();
            // if new timer trigger first
            if deadline < curr_timer.as_ref().unwrap().deadline {
                let old_current = curr_timer.take().unwrap();
                curr_timer.replace(TimerMeta {
                    typ,
                    save: false,
                    from,
                    deadline,
                    args,
                });
                // put the old one into the wait queue
                //  - find out the insert index
                if let Some((idx, _)) = timers
                    .iter()
                    .enumerate()
                    .find(|(_, item)| item.deadline > old_current.deadline)
                {
                    //  - insert the old one
                    timers.insert(idx - 1, old_current);
                } else {
                    timers.push_back(old_current);
                }
                need_refresh = true;
            } else {
                let new_timer = TimerMeta {
                    typ,
                    save: false,
                    from,
                    deadline,
                    args,
                };
                if let Some((idx, _)) = timers
                    .iter()
                    .enumerate()
                    .find(|(_, item)| item.deadline > deadline)
                {
                    //  - insert the old one
                    timers.insert(idx - 1, new_timer);
                } else {
                    timers.push_back(new_timer);
                }
                need_refresh = false;
            }
        }
        if need_refresh {
            let tx = self.timer_tx.clone();
            self.curr_handle.abort();
            self.curr_handle = tokio::spawn(async move {
                tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)).await;
                if let Err(err) = tx.send(()).await {
                    tracing::error!("mpsc send error: {:?}", err);
                }
            });
        }
    }
}
