# gsvr.rs
Game server written in rust, example of [gsfw](https://crates.io/crates/gsfw)

## Architecute
![architure](./pic/ARCHITECTURE.png)

## Services

### pf
**platform service** is used for account register and authorization. In addition, when a account is authorized, a token is return to the user, which can be used for access other HTTP APIs like retriving all the players of the account and create, modify, delete players. In order to connect to the gate service, both token and player_id are required when sending the first protocol [cspb::CsLogin](cspb/proto/01_player.proto).

### gate
**gate service** serve as a broker for redirect users' protocol to the game service and send reply from the game service to the users. When a user connect to the gate, before it begin to proxy user's protocol, authorization is required, users are expected to send [cspb::CsLogin](cspb/proto/01_player.proto) as the very first protocol, if the token is valid, [cspb::ScLogin](cspb/proto/01_player.proto) with ErrCode::Success will be send back. Each GateAgent runs on their own cooroutine.

### game
**game service** is used to handle logined players' simple protocols like Gacha, accept missions, buy items, edit their equipments. It is not suitable for execute battle logics like synchronize players' coordinates, checking if bullets hit the enemies. If you need those functions, consider creating a new service for battle logics.