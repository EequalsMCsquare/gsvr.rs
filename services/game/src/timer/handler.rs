use gsfw::util::timer::Wheel;

use crate::hub::ModuleName;

use super::{desc::Description, kind::TimerKind, TMProtoAck, TimerComponet};

impl TimerComponet {
    pub(super) async fn ctl_new_timeout(
        wheel: &mut Wheel<Description>,
        from: ModuleName,
        duration: std::time::Duration,
        data: TimerKind,
    ) -> crate::error::Result<TMProtoAck> {
        let desc = Description {
            from,
            data,
            repeat: false,
        };
        match wheel.dispatch(duration, desc).await {
            Ok(snapshot) => Ok(TMProtoAck::TimeoutSnapshot {
                id: snapshot.id,
                start: snapshot.start,
                duration,
                repeat: false,
            }),
            Err(err) => Err(crate::Error::TimeWheel(format!(
                "dispatch timeout error: {}",
                err
            ))),
        }
    }

    pub(super) async fn ctl_new_interval(
        wheel: &mut Wheel<Description>,
        from: ModuleName,
        duration: std::time::Duration,
        data: TimerKind,
    ) -> crate::error::Result<TMProtoAck> {
        let desc = Description {
            from,
            data,
            repeat: true,
        };
        match wheel.dispatch(duration, desc).await {
            Ok(snapshot) => Ok(TMProtoAck::TimeoutSnapshot {
                id: snapshot.id,
                start: snapshot.start,
                duration,
                repeat: true,
            }),
            Err(err) => Err(crate::Error::TimeWheel(format!(
                "dispatch interval error: {}",
                err
            ))),
        }
    }

    pub(super) async fn ctl_new_deadline(
        wheel: &mut Wheel<Description>,
        from: ModuleName,
        deadline: std::time::Instant,
        data: TimerKind,
    ) -> crate::error::Result<TMProtoAck> {
        let desc = Description {
            from,
            data,
            repeat: false,
        };
        match wheel.dispatch_until(deadline, desc).await {
            Ok(snapshot) => Ok(TMProtoAck::DeadlineSnapshot {
                id: snapshot.id,
                start: snapshot.start,
                deadline,
            }),
            Err(err) => Err(crate::Error::TimeWheel(format!(
                "dispatch deadline error: {}",
                err
            ))),
        }
    }
}
