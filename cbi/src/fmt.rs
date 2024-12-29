use a2s::info::Info;
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