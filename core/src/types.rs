use serde::{Deserialize, Serialize};

/// 摘抄部分感兴趣的 A2S::info::Info 字段
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2SInfo {
    name: String,
    map: String,
    players: u8,
    max_players: u8,
    game: String,
    app_id: u16,
    bots: u8,
    visibility: bool,
    vac: bool,
    version: String,
}

/// 摘抄部分感兴趣的 A2S::info::Player 字段
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2SPlayer {
    name: String,
    score: i32,
    duration: f32,
}

impl From<a2s::info::Info> for A2SInfo {
    fn from(info: a2s::info::Info) -> Self {
        A2SInfo {
            name: info.name,
            map: info.map,
            players: info.players,
            max_players: info.max_players,
            game: info.game,
            app_id: info.app_id,
            bots: info.bots,
            visibility: info.visibility,
            vac: info.vac,
            version: info.version,
        }
    }
}

impl From<a2s::players::Player> for A2SPlayer {
    fn from(player: a2s::players::Player) -> Self {
        A2SPlayer {
            name: player.name,
            score: player.score,
            duration: player.duration,
        }
    }
}
