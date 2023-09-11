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
