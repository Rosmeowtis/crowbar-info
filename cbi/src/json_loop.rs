use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;

use ::core::types::{A2SInfo, A2SPlayer};

pub async fn main_loop(a2s: Arc<a2s::A2SClient>) {
    let mut read_buf = String::with_capacity(1024);
    let mut stdin_buf = BufReader::new(std::io::stdin());
    loop {
        stdin_buf.read_line(&mut read_buf).unwrap();
        let line = read_buf.trim();
        if line.len() == 0 {
            continue;
        }
        let input = serde_json::from_str::<JsonInput>(line);
        match input {
            Err(e) => {
                let output = JsonOutput {
                    cmd: None,
                    payloads: vec![JsonPayload::Err {
                        what: JsonResult::Err(format!("Failed to parse input: {}(P{})", &line, &e)),
                    }],
                };
                let output = serde_json::to_string(&output).unwrap();
                std::io::stdout().write_all(&output.as_bytes()).unwrap();
                std::io::stdout().write(b"\n").unwrap();
                std::io::stdout().flush().unwrap();
            }
            Ok(input) => {
                let output = match input.cmd {
                    JsonCommand::Info => {
                        let mut jobs = vec![];
                        let mut results = vec![];
                        for host in input.hosts {
                            let a2s = a2s.clone();
                            let handle =
                                tokio::spawn(async move { (host.clone(), a2s.info(&host).await) });
                            jobs.push(handle);
                        }
                        for job in jobs {
                            let res = job.await.unwrap();
                            results.push(res);
                        }
                        let results = results;
                        let payloads = results
                            .into_iter()
                            .map(|(host, r)| match r {
                                Ok(info) => JsonPayload::Info {
                                    host,
                                    info: JsonResult::Ok(A2SInfo::from(info)),
                                },
                                Err(e) => JsonPayload::Info {
                                    host,
                                    info: JsonResult::Err(format!("Failed to get info: {e}")),
                                },
                            })
                            .collect();
                        JsonOutput {
                            cmd: Some(JsonCommand::Info),
                            payloads,
                        }
                    }
                    JsonCommand::Players => {
                        let mut jobs = vec![];
                        let mut results = vec![];
                        for host in input.hosts {
                            let a2s = a2s.clone();
                            let handle =
                                tokio::spawn(
                                    async move { (host.clone(), a2s.players(&host).await) },
                                );
                            jobs.push(handle);
                        }
                        for job in jobs {
                            let res = job.await.unwrap();
                            results.push(res);
                        }
                        let results = results;
                        let payloads = results
                            .into_iter()
                            .map(|(host, r)| match r {
                                Ok(players) => JsonPayload::Players {
                                    host,
                                    players: JsonResult::Ok(
                                        players.into_iter().map(A2SPlayer::from).collect(),
                                    ),
                                },
                                Err(e) => JsonPayload::Players {
                                    host,
                                    players: JsonResult::Err(format!("Failed to get info: {e}")),
                                },
                            })
                            .collect();
                        JsonOutput {
                            cmd: Some(JsonCommand::Players),
                            payloads,
                        }
                    }
                    JsonCommand::Full => {
                        let mut jobs = vec![];
                        let mut results = vec![];
                        for host in input.hosts {
                            let a2s = a2s.clone();
                            let handle = tokio::spawn(async move {
                                (
                                    host.clone(),
                                    a2s.info(&host).await,
                                    a2s.players(&host).await,
                                )
                            });
                            jobs.push(handle);
                        }
                        for job in jobs {
                            let res = job.await.unwrap();
                            results.push(res);
                        }
                        let results = results;
                        let payloads = results
                            .into_iter()
                            .map(|(host, i, p)| match (i, p) {
                                (Ok(info), Ok(players)) => JsonPayload::Full {
                                    host,
                                    info: JsonResult::Ok(info.into()),
                                    players: JsonResult::Ok(
                                        players.into_iter().map(A2SPlayer::from).collect(),
                                    ),
                                },
                                (Ok(info), Err(e)) => JsonPayload::Full {
                                    host,
                                    info: JsonResult::Ok(info.into()),
                                    players: JsonResult::Err(format!("Failed to get players: {e}")),
                                },
                                (Err(e), Ok(players)) => JsonPayload::Full {
                                    host,
                                    info: JsonResult::Err(format!("Failed to get info: {e}")),
                                    players: JsonResult::Ok(
                                        players.into_iter().map(A2SPlayer::from).collect(),
                                    ),
                                },
                                (Err(e1), Err(e2)) => JsonPayload::Full {
                                    host,
                                    info: JsonResult::Err(format!("Failed to get info: {e1}")),
                                    players: JsonResult::Err(format!(
                                        "Failed to get players: {e2}"
                                    )),
                                },
                            })
                            .collect();
                        JsonOutput {
                            cmd: Some(JsonCommand::Full),
                            payloads,
                        }
                    }
                };
                let output = serde_json::to_string(&output).unwrap();
                std::io::stdout().write_all(&output.as_bytes()).unwrap();
                std::io::stdout().write(b"\n").unwrap();
                std::io::stdout().flush().unwrap();
            }
        }
        read_buf.clear();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct JsonInput {
    cmd: JsonCommand,
    hosts: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum JsonCommand {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "players")]
    Players,
    #[serde(rename = "full")]
    Full,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct JsonOutput {
    cmd: Option<JsonCommand>,
    payloads: Vec<JsonPayload>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum JsonPayload {
    Info {
        host: String,
        info: JsonResult<A2SInfo>,
    },
    Players {
        host: String,
        players: JsonResult<Vec<A2SPlayer>>,
    },
    Full {
        host: String,
        info: JsonResult<A2SInfo>,
        players: JsonResult<Vec<A2SPlayer>>,
    },
    /// 未知错误
    Err { what: JsonResult<()> },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum JsonResult<T> {
    Ok(T),
    Err(String),
}
