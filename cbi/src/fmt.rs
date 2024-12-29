use a2s::{info::Info, players::Player};
use iocraft::prelude::*;
use std::error::Error;

#[derive(Clone)]
struct DisplayedInfo {
    id: usize,
    name: String,
    map: String,
    players: u8,
    max_players: u8,
}

impl DisplayedInfo {
    fn new(id: usize, name: &str, map: &str, players: u8, max_players: u8) -> Self {
        Self {
            id,
            name: name.into(),
            map: map.into(),
            players,
            max_players,
        }
    }
    fn from_a2s(info: Info, id: usize) -> Self {
        Self {
            id,
            name: info.name,
            map: info.map,
            players: info.players,
            max_players: info.max_players,
        }
    }
}

#[derive(Default, Props)]
struct InfosTableProps<'a> {
    infos: Option<&'a Vec<DisplayedInfo>>,
}

#[component]
fn InfosTable<'a>(props: &InfosTableProps<'a>) -> impl Into<AnyElement<'a>> {
    element! {
        Box(
            margin_top: 1,
            margin_bottom: 1,
            flex_direction: FlexDirection::Column,
            width: 60,
            border_style: BorderStyle::Round,
            border_color: Color::Cyan,
        ) {
            Box(border_style: BorderStyle::Single, border_edges: Edges::Bottom, border_color: Color::Grey) {
                Box(width: 10pct, justify_content: JustifyContent::End, padding_right: 2) {
                    Text(content: "Id", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 40pct) {
                    Text(content: "Name", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 30pct) {
                    Text(content: "Map", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 10pct) {
                    Text(content: "P", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 10pct) {
                    Text(content: "/MaxP", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }
            }

            #(props.infos.map(|infos| infos.iter().enumerate().map(|(i, info)| element! {
                Box(background_color: if i % 2 == 0 { None } else { Some(Color::DarkGrey) }) {
                    Box(width: 10pct, justify_content: JustifyContent::End, padding_right: 2) {
                        Text(content: info.id.to_string())
                    }

                    Box(width: 40pct) {
                        Text(content: info.name.clone())
                    }

                    Box(width: 30pct) {
                        Text(content: info.map.clone(), weight: Weight::Bold, decoration: TextDecoration::Underline)
                    }

                    Box(width: 10pct) {
                        Text(content: info.players.to_string(), weight: Weight::Bold, decoration: TextDecoration::Underline)
                    }

                    Box(width: 10pct) {
                        Text(content: info.max_players.to_string(), weight: Weight::Bold, decoration: TextDecoration::Underline)
                    }
                }
            })).into_iter().flatten())
        }
    }
}

pub fn fmt_info(infos: Vec<Result<Info, impl Error>>) {
    let infos: Vec<DisplayedInfo> = infos
        .into_iter()
        .enumerate()
        .map(|(j, i)| match i {
            Ok(i) => DisplayedInfo::from_a2s(i, j),
            Err(e) => DisplayedInfo::new(j, &format!("#<{}>", &e), "", 0, 0),
        })
        .collect();
    element!(InfosTable(infos: &infos)).print();
}

#[derive(Clone)]
struct DisplayedPlayer {
    id: usize,
    name: String,
    score: i32,
    duration: f32,
}

impl DisplayedPlayer {
    fn new(id: usize, name: &str, score: i32, duration: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            score,
            duration,
        }
    }
    fn from_a2s(player: Player, id: usize) -> Self {
        Self {
            id,
            name: player.name,
            score: player.score,
            duration: player.duration,
        }
    }
}

#[derive(Default, Props)]
struct PlayersTableProps<'a> {
    players: Option<&'a Vec<DisplayedPlayer>>,
}

fn display_time(duration: f32) -> String {
    if duration < 60.0 {
        format!("{:.1}s", duration)
    } else if duration < 3600.0 {
        format!("{:.1}m", duration / 60.0)
    } else {
        format!("{:.1}h", duration / 3600.0)
    }
}

#[component]
fn PlayersTable<'a>(props: &PlayersTableProps<'a>) -> impl Into<AnyElement<'a>> {
    element! {
        Box(
            margin_top: 1,
            margin_bottom: 1,
            flex_direction: FlexDirection::Column,
            width: 60,
            border_style: BorderStyle::Round,
            border_color: Color::Cyan,
        ) {
            Box(border_style: BorderStyle::Single, border_edges: Edges::Bottom, border_color: Color::Grey) {
                Box(width: 10pct, justify_content: JustifyContent::End, padding_right: 2) {
                    Text(content: "Id", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 40pct) {
                    Text(content: "Name", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 30pct) {
                    Text(content: "Score", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }

                Box(width: 20pct) {
                    Text(content: "Time", weight: Weight::Bold, decoration: TextDecoration::Underline)
                }
            }

            #(props.players.map(|players| players.iter().enumerate().map(|(i, player)| element! {
                Box(background_color: if i % 2 == 0 { None } else { Some(Color::DarkGrey) }) {
                    Box(width: 10pct, justify_content: JustifyContent::End, padding_right: 2) {
                        Text(content: player.id.to_string())
                    }

                    Box(width: 40pct) {
                        Text(content: player.name.clone())
                    }

                    Box(width: 30pct) {
                        Text(content: player.score.to_string(), weight: Weight::Bold, decoration: TextDecoration::Underline)
                    }

                    Box(width: 20pct) {
                        Text(content: display_time(player.duration), weight: Weight::Bold, decoration: TextDecoration::Underline)
                    }
                }
            })).into_iter().flatten())
        }
    }
}

pub fn fmt_players(players: Vec<Result<Vec<Player>, impl Error>>) {
    let players: Vec<DisplayedPlayer> = players.into_iter().fold(vec![], |mut acc, ps| {
        match ps {
            Ok(mut ps) => {
                ps.sort_by(|l, r| r.duration.partial_cmp(&l.duration).unwrap());
                for p in ps {
                    acc.push(DisplayedPlayer::from_a2s(p, acc.len()));
                }
            }
            Err(e) => acc.push(DisplayedPlayer::new(
                acc.len(),
                &format!("#{{{e}}}"),
                0,
                0.0,
            )),
        };
        acc
    });
    element!(PlayersTable(players: &players)).print();
}
