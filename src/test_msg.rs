use crate::header_field::*;
use crate::tree::*;
use crate::utils::nul_terminated_str;
use anyhow::Result;


#[derive(Debug, Default)]
pub struct Message {
    pub body: Body,
}

#[derive(Debug)]
pub enum Body {
    InitSyn(InitSyn),
    InitAck(InitAck),
}

impl Default for Body {
    fn default() -> Self {
        Self::InitSyn(InitSyn::default())
    }
}

#[derive(Debug, Default)]
pub struct InitSyn {
    pub version: u8,
    pub whatami: WhatAmI,
}

#[derive(Debug, Default)]
pub struct InitAck {
    pub version: u8,
    pub whatami: WhatAmI,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WhatAmI {
    #[default]
    Router = 0b001,
    Peer = 0b010,
    Client = 0b100,
}

impl WhatAmI {
    const STR_R: &str = "router";
    const STR_P: &str = "peer";
    const STR_C: &str = "client";

    const U8_R: u8 = Self::Router as u8;
    const U8_P: u8 = Self::Peer as u8;
    const U8_C: u8 = Self::Client as u8;

    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Router => Self::STR_R,
            Self::Peer => Self::STR_P,
            Self::Client => Self::STR_C,
        }
    }
}

impl std::fmt::Display for WhatAmI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
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


impl AddToTree for Message {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()>{
        self.body.add_to_tree(&format!("{prefix}.body"), args)
    }
}

impl AddToTree for Body {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
        match self {
            Self::InitSyn(body) => {
                body.add_to_tree(&format!("{prefix}.init_syn"), args)
            }
            Self::InitAck(body) => {
                todo!()
            }
        }
    }
}

impl AddToTree for InitSyn {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
        let hf_index = *args
            .hf_map
            .get(&format!("{prefix}.version"))
            .expect(&format!("{prefix}.version not found in hf_map"));
        unsafe {
            epan_sys::proto_tree_add_uint(
                args.tree,
                hf_index,
                args.tvb,
                2 as _,
                3,
                self.version.into(),
            );
        }

        let hf_index = *args
            .hf_map
            .get(&format!("{prefix}.whatami"))
            .expect(&format!("{prefix}.whatami not found in hf_map"));
        unsafe {
            epan_sys::proto_tree_add_string(
                args.tree,
                hf_index,
                args.tvb,
                2 as _,
                3,
                nul_terminated_str(self.whatami.to_str()).unwrap(),
            );
        }

        Ok(())
    }
}
