#![allow(unused)]
pub mod utils;
pub mod ext;
pub mod wireshark;
pub mod tree;
pub mod test_msg;

use zenoh_protocol::transport::{TransportMessage, TransportBody};
use derives::Registrable;
use std::{collections::HashMap, fmt::Debug};
use anyhow::Result;
use crate::ext::DefaultExt;
use test_msg::*;

#[derive(Debug)]
pub struct HeaderField {
    pub name: String,
    pub kind: FieldKind,
}

pub type HeaderFieldMap = HashMap<String, HeaderField>;

#[derive(Debug, Copy, Clone)]
pub enum FieldKind {
    Number,
    Text,
}

pub trait IntoHFMap {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap;
}

pub enum Span<T> {
    Enum(Vec<T>),
    Struct(T)
}

pub trait GenerateHFMap: Sized + IntoHFMap + Debug {

    fn span() -> Span<Self>;

    fn generate_hf_map(prefix: &str) -> HeaderFieldMap {
        match Self::span() {
            Span::Enum(branches) => {
                dbg!(&branches);
                let mut hf_map = HeaderFieldMap::new();
                for item in branches {
                    hf_map.extend(item.into_hf_map(prefix.clone()));
                }
                hf_map
            }
            Span::Struct(st) => {
                st.into_hf_map(prefix)
            }
        }
    }
}

impl GenerateHFMap for InitAck {
    fn span() -> Span<Self> {
        Span::Struct(Self::default())
    }
}

impl GenerateHFMap for InitSyn {
    fn span() -> Span<Self> {
        Span::Struct(Self::default())
    }
}

impl GenerateHFMap for Body {
    fn span() -> Span<Self> {
        Span::Enum(vec![
            Body::InitSyn(InitSyn::default()),
            Body::InitAck(InitAck::default()),
        ])
    }
}

impl GenerateHFMap for Message {
    fn span() -> Span<Self> {
        Span::Struct(Self::default())
    }
}

impl IntoHFMap for Message {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
        Body::generate_hf_map(&format!("{prefix}.body"))
        // self.body.into_hf_map(&format!("{prefix}.body"))
    }
}

impl IntoHFMap for Body {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
        match self {
            Self::InitSyn(body) => {
                body.into_hf_map(&format!("{prefix}.init_syn"))
            },
            Self::InitAck(body) => {
                body.into_hf_map(&format!("{prefix}.init_ack"))
            }
        }
    }
}

impl IntoHFMap for InitSyn {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
        let mut hf_map = HeaderFieldMap::new();
        hf_map.insert(
            format!("{prefix}.version"),
            HeaderField {
                name: "Version".into(),
                kind: FieldKind::Number,
            },
        );
        hf_map.insert(
            format!("{prefix}.whatami"),
            HeaderField {
                name: "WhatAmI".into(),
                kind: FieldKind::Text,
            },
        );
        hf_map
    }
}

impl IntoHFMap for InitAck {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
        let mut hf_map = HeaderFieldMap::new();
        hf_map.insert(
            format!("{prefix}.version"),
            HeaderField {
                name: "Version".into(),
                kind: FieldKind::Number,
            },
        );
        hf_map.insert(
            format!("{prefix}.whatami"),
            HeaderField {
                name: "WhatAmI".into(),
                kind: FieldKind::Text,
            },
        );
        hf_map
    }
}
