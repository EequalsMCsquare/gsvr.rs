use crate::client::{gclient::GClient, nclient::NClient};
use anyhow::bail;
use fst::{Automaton, IntoStreamer};
use once_cell::sync::Lazy;
use requestty::{question::Completions, Answers};
use std::{collections::HashMap, str::FromStr};
use strum::{IntoEnumIterator, VariantNames};

// execute fast login client
pub async fn run_game_client(mut client: GClient, prompt: String) -> anyhow::Result<()> {
    client.authenticate().await?;
    loop {
        let input = requestty::prompt_one(
            requestty::Question::input("cmd")
                .message(&prompt)
                .validate(|cmd, _| {
                    if let Err(err) = cmd.parse::<GameCmd>() {
                        Err(format!("invalid input: {}", err))
                    } else {
                        Ok(())
                    }
                })
                .auto_complete(GameCmd::auto_complete),
        )?;
        let cmd: GameCmd = input.as_string().unwrap().parse().unwrap();
        cmd.handle(&mut client).await?;
    }
}

// execute normal login client
pub async fn run_api_client(mut client: NClient, prompt: String) -> anyhow::Result<()> {
    // ApiCmd Loop
    loop {
        let input = requestty::prompt_one(
            requestty::Question::input("api_cmd")
                .message(&prompt)
                .validate(|cmd, _| {
                    if let Err(err) = cmd.parse::<ApiCmd>() {
                        Err(format!("invalid input: {}", err))
                    } else {
                        Ok(())
                    }
                })
                .auto_complete(ApiCmd::auto_complete),
        )?;
        let cmd: ApiCmd = input.as_string().unwrap().parse().unwrap();
        cmd.handle(&mut client).await?;
    }
}

#[derive(strum::EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
enum ApiCmd {
    List,
    Create(String),
    Use(i64),
    Help,
}

impl FromStr for ApiCmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = shell_words::split(s)?;
        if words.len() == 0 {
            bail!("invalid input")
        }
        if words[0] == "list" {
            Ok(ApiCmd::List)
        } else if words[0] == "create" {
            if words.len() != 2 {
                bail!("create <name>")
            }
            Ok(ApiCmd::Create(words[1].clone()))
        } else if words[0] == "use" {
            if words.len() != 2 {
                bail!("create <player_id>")
            }
            Ok(ApiCmd::Use(words[1].parse()?))
        } else if words[0] == "help" {
            Ok(ApiCmd::Help)
        } else {
            bail!("unknown command, try 'help'")
        }
    }
}

impl ApiCmd {
    const CMD_HINT: Lazy<fst::Set<Vec<u8>>> = Lazy::new(|| {
        let mut cmds: Vec<&'static str> = Self::VARIANTS.iter().map(|&s| s).collect();
        cmds.sort();
        fst::Set::from_iter(cmds).unwrap()
    });

    const HELP_MSG: &'static str = r#"
admin client api:
    help: display this help message
    list: list all players of current account
    create: create a new player
        format: create <name>
        example: create Reco
    use: login to gate with given player id
        format: use <player id>
        example: use 43534
"#;

    pub fn auto_complete(p: String, _: &Answers) -> Completions<String> {
        Self::CMD_HINT
            .search(fst::automaton::Str::new(&p).starts_with())
            .into_stream()
            .into_strs()
            .unwrap_or(vec![p])
            .into()
    }

    pub async fn handle(&self, client: &mut NClient) -> anyhow::Result<()> {
        match self {
            ApiCmd::List => match client.list_players().await {
                Ok(ack) => {
                    println!("PLAYERS:");
                    ack.iter().for_each(|p| println!("\t{}", p));
                    Ok(())
                }
                Err(err) => {
                    eprintln!("fail to list player. {}", err);
                    Ok(())
                }
            },
            ApiCmd::Create(name) => match client.create_player(name).await {
                Ok(ack) => {
                    println!("new player: {}", ack);
                    Ok(())
                }
                Err(err) => {
                    eprintln!("fail to create player. {}", err);
                    Ok(())
                }
            },
            ApiCmd::Use(id) => match client.use_player(*id).await {
                Ok(_) => {
                    println!("logout from gate client");
                    Ok(())
                }
                Err(err) => {
                    eprintln!("error occur when run_game_client. {}", err);
                    Ok(())
                }
            },
            ApiCmd::Help => {
                print!("{}", Self::HELP_MSG);
                Ok(())
            }
        }
    }
}

#[derive(strum::EnumVariantNames, PartialEq, Debug)]
#[strum(serialize_all = "snake_case")]
enum GameCmd {
    Send(cspb::CsMsg),
    Logout,
    Help,
}

impl FromStr for GameCmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_whitespace().collect();
        if words.len() == 0 {
            bail!("invalid input")
        }
        if words[0] == "help" {
            Ok(GameCmd::Help)
        } else if words[0] == "logout" {
            Ok(GameCmd::Logout)
        } else if words[0] == "send" {
            if words.len() != 3 {
                bail!("send <cspb::CsMsg> <payload>")
            }
            let strpb = format!(
                r#"{{"{pbname}":{payload}}}"#,
                pbname = words[1],
                payload = words[2]
            );
            println!("{}", strpb);
            let ret = serde_json::from_str(&strpb)?;
            Ok(GameCmd::Send(ret))
        } else {
            bail!("unknown command, try 'help'")
        }
    }
}

impl GameCmd {
    const CSMSG_NAME_PAYLOAD: Lazy<HashMap<&'static str, String>> = Lazy::new(|| {
        cspb::CsMsg::iter()
            .enumerate()
            .map(|(idx, payload)| {
                let pstr = serde_json::to_string(&payload).expect("serialize error");
                let (_, payload) = pstr.split_once(':').unwrap();
                (
                    cspb::CsMsg::VARIANTS[idx],
                    payload[..payload.len() - 1].to_string(),
                )
            })
            .collect()
    });

    pub const CSMSG_NAME_HINT: Lazy<fst::Set<Vec<u8>>> = Lazy::new(|| {
        let mut csmsg: Vec<&'static str> = cspb::CsMsg::VARIANTS.iter().map(|&s| s).collect();
        csmsg.sort();
        fst::Set::from_iter(csmsg).unwrap()
    });

    const CMD_HINT: Lazy<fst::Set<Vec<u8>>> = Lazy::new(|| {
        let mut cmds: Vec<&'static str> = Self::VARIANTS.iter().map(|&s| s).collect();
        cmds.sort();
        fst::Set::from_iter(cmds).unwrap()
    });

    const HELP_MSG: &'static str = r#"
admin client:
    help: display this help message
    send: send protocol to the gate
        format: send <name> <payload>
        example: send CsEcho {"content": "hello world"}
    logout: disconnect from gate
"#;

    pub async fn handle(&self, client: &mut GClient) -> anyhow::Result<()> {
        match self {
            GameCmd::Help => {
                print!("{}", Self::HELP_MSG);
                Ok(())
            }
            GameCmd::Send(msg) => {
                if let Err(err) = client.send(msg.clone()).await {
                    eprintln!("AdminClient::send error. {}", err);
                    return Ok(());
                }
                match client.recv().await {
                    Ok(msg) => println!("{:?}", msg),
                    Err(err) => eprintln!("AdminClient::recv error. {}", err),
                }
                Ok(())
            }
            GameCmd::Logout => bail!("logout"),
        }
    }

    pub fn auto_complete(p: String, _: &Answers) -> Completions<String> {
        if p.len() == 0 {
            return GameCmd::VARIANTS.iter().map(|&s| String::from(s)).collect();
        }
        // split into shell_words
        let sw = shell_words::split(&p).unwrap();
        // hint cmds
        if sw.len() == 1 && !p.ends_with(' ') {
            return Self::CMD_HINT
                .search(fst::automaton::Str::new(&sw[0]).starts_with())
                .into_stream()
                .into_strs()
                .unwrap_or(vec![p])
                .into();
        }
        if sw[0] == "send" {
            let hint_iter;
            if sw.len() == 2 {
                hint_iter = Self::CSMSG_NAME_HINT
                    .search(fst::automaton::Str::new(&sw[1]).starts_with())
                    .into_stream()
                    .into_strs()
                    .unwrap_or_default()
                    .into_iter();
            } else {
                hint_iter = Self::CSMSG_NAME_HINT
                    .search(fst::automaton::AlwaysMatch)
                    .into_stream()
                    .into_strs()
                    .unwrap_or_default()
                    .into_iter();
            }
            // return user input directly
            if hint_iter.len() == 0 {
                return Completions::from_vec(vec![p]);
            } else if hint_iter.len() == 1 {
                // auto-complete message payload as well
                return hint_iter
                    .map(|s| {
                        println!("s => {}", s);
                        format!(
                            "send {} {}",
                            s,
                            Self::CSMSG_NAME_PAYLOAD.get(s.as_str()).unwrap()
                        )
                    })
                    .collect();
            } else {
                return hint_iter.map(|s| format!("send {}", s)).collect();
            }
        } else {
            vec![p].into()
        }
    }
}

#[test]
fn foo() {
    let input = "send CsEcho {\"content\":\"hello\"}";
    let cmd: GameCmd = input.parse().unwrap();
    assert_eq!(
        cmd,
        GameCmd::Send(cspb::CsMsg::CsEcho(cspb::CsEcho {
            content: "hello".to_string()
        }))
    );
}
