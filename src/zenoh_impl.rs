use crate::header_field::*;
use crate::tree::*;
use crate::utils::nul_terminated_str;
use anyhow::{bail, Result};
use zenoh_protocol::core::{wire_expr::WireExpr, Encoding, ZenohId};
use zenoh_protocol::network::{push::Push, NetworkBody, NetworkMessage};
use zenoh_protocol::transport::fragment::Fragment;
use zenoh_protocol::transport::{frame::Frame, init::InitSyn, TransportBody, TransportMessage};
use zenoh_protocol::zenoh::{del::Del, put::Put, PushBody};

trait Sample {
    fn sample() -> Self;
}

#[derive(Default)]
pub struct ZenohProtocol {}

mod impl_for_zenoh_protocol {
    use crate::zenoh_impl::*;

    impl IntoHFMap for ZenohProtocol {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();
            hf_map.insert(
                prefix.to_string(),
                HeaderField {
                    name: "ZenohProtocol".into(),
                    kind: FieldKind::Branch,
                },
            );
            hf_map.extend(TransportMessage::generate_hf_map(prefix));
            hf_map
        }
    }

    impl GenerateHFMap for ZenohProtocol {
        fn span() -> Span<Self> {
            Span::Struct(ZenohProtocol::default())
        }
    }
}

mod impl_for_init_syn {
    use crate::zenoh_impl::*;

    impl IntoHFMap for InitSyn {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();

            // version
            hf_map.insert(
                format!("{prefix}.version"),
                HeaderField {
                    name: "Version".into(),
                    kind: FieldKind::Number,
                },
            );

            // whatmai
            hf_map.insert(
                format!("{prefix}.whatami"),
                HeaderField {
                    name: "WhatAmI".into(),
                    kind: FieldKind::Text,
                },
            );

            // zid
            hf_map.insert(
                format!("{prefix}.zid"),
                HeaderField {
                    name: "ZenohId".into(),
                    kind: FieldKind::Text,
                },
            );

            // resolution
            hf_map.insert(
                format!("{prefix}.resolution"),
                HeaderField {
                    name: "Resolution".into(),
                    kind: FieldKind::Number,
                },
            );

            // batch_size
            hf_map.insert(
                format!("{prefix}.batch_size"),
                HeaderField {
                    name: "BatchSize".into(),
                    kind: FieldKind::Number,
                },
            );

            hf_map
        }
    }

    impl Sample for InitSyn {
        fn sample() -> Self {
            Self {
                version: 0,
                whatami: zenoh_protocol::core::WhatAmI::Peer,
                zid: ZenohId::rand(),
                resolution: 0.into(),
                batch_size: 0,
                ext_qos: None,
                ext_shm: None,
                ext_auth: None,
                ext_mlink: None,
            }
        }
    }

    impl GenerateHFMap for InitSyn {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for InitSyn {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            let hf_index = args.get_hf(&format!("{prefix}.version"))?;
            unsafe {
                epan_sys::proto_tree_add_uint(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    self.version.into(),
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.whatami"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(self.whatami.to_str())?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.zid"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&self.zid.to_string())?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.resolution"))?;
            unsafe {
                epan_sys::proto_tree_add_uint(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    self.resolution.as_u8().into(),
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.batch_size"))?;
            unsafe {
                epan_sys::proto_tree_add_uint(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    self.batch_size.into(),
                );
            }

            Ok(())
        }
    }
}

mod impl_for_init_fragment {
    use crate::zenoh_impl::*;

    impl IntoHFMap for Fragment {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();

            hf_map.insert(
                format!("{prefix}.reliability"),
                HeaderField {
                    name: "Reliability".into(),
                    kind: FieldKind::Text,
                },
            );

            hf_map.insert(
                format!("{prefix}.more"),
                HeaderField {
                    name: "More".into(),
                    kind: FieldKind::Text,
                },
            );

            hf_map.insert(
                format!("{prefix}.sn"),
                HeaderField {
                    name: "TransportSn".into(),
                    kind: FieldKind::Text,
                },
            );

            hf_map.insert(
                format!("{prefix}.payload"),
                HeaderField {
                    name: "Payload".into(),
                    kind: FieldKind::Text,
                },
            );

            hf_map.insert(
                format!("{prefix}.ext_qos"),
                HeaderField {
                    name: "ExtQoS".into(),
                    kind: FieldKind::Text,
                },
            );

            hf_map
        }
    }

    impl Sample for Fragment {
        fn sample() -> Self {
            Fragment::rand()
        }
    }

    impl GenerateHFMap for Fragment {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for Fragment {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            let hf_index = args.get_hf(&format!("{prefix}.reliability"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.reliability))?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.more"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.more))?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.sn"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.sn))?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.payload"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.payload))?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.ext_qos"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.ext_qos))?,
                );
            }

            Ok(())
        }
    }
}

mod imp_for_transport_body {
    use crate::zenoh_impl::*;

    impl Sample for TransportBody {
        fn sample() -> Self {
            Self::InitSyn(InitSyn::sample())
        }
    }

    impl IntoHFMap for TransportBody {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();
            hf_map.insert(
                format!("{prefix}"),
                HeaderField {
                    name: "TransportBody".into(),
                    kind: FieldKind::Branch,
                },
            );

            let branches = match self {
                Self::InitSyn(body) => body.into_hf_map(&format!("{prefix}.init_syn")),
                Self::Frame(body) => body.into_hf_map(&format!("{prefix}.frame")),
                Self::Fragment(body) => body.into_hf_map(&format!("{prefix}.fragment")),
                _ => {
                    todo!()
                }
            };
            hf_map.extend(branches);
            hf_map
        }
    }

    impl GenerateHFMap for TransportBody {
        fn span() -> Span<Self> {
            Span::Enum(vec![
                Self::InitSyn(InitSyn::rand()),
                Self::Frame(Frame::rand()),
                Self::Fragment(Fragment::rand()),
            ])
        }
    }

    impl AddToTree for TransportBody {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            match self {
                Self::InitSyn(body) => {
                    body.add_to_tree(
                        &format!("{prefix}.init_syn"),
                        &args.make_subtree(prefix, &format!("TransportBody (InitSyn)"))?
                    )?;
                }
                Self::Frame(body) => {
                    body.add_to_tree(
                        &format!("{prefix}.frame"),
                        &args.make_subtree(prefix, &format!("TransportBody (Frame)"))?
                    )?;
                }
                Self::Fragment(body) => {
                    body.add_to_tree(
                        &format!("{prefix}.fragment"),
                        &args.make_subtree(prefix, &format!("TransportBody (Fragment)"))?
                    )?;
                }
                _ => {
                    bail!("Not implemented yet.");
                }
            }
            Ok(())
        }
    }
}

mod impl_for_transport_message {
    use crate::zenoh_impl::*;

    impl Sample for TransportMessage {
        fn sample() -> Self {
            Self::rand()
        }
    }

    impl IntoHFMap for TransportMessage {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            TransportBody::generate_hf_map(&format!("{prefix}.body"))
        }
    }

    impl GenerateHFMap for TransportMessage {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for TransportMessage {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            self.body.add_to_tree(&format!("{prefix}.body"), args)
        }
    }
}

mod impl_for_put {
    use crate::zenoh_impl::*;

    impl Sample for Put {
        fn sample() -> Self {
            Put {
                timestamp: None,
                encoding: Encoding::TEXT_PLAIN,
                ext_sinfo: None,
                ext_unknown: vec![],
                payload: vec![].into(),
            }
        }
    }

    impl IntoHFMap for Put {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();

            // timestamp
            hf_map.insert(
                format!("{prefix}.timestamp"),
                HeaderField {
                    name: "Timestamp".into(),
                    kind: FieldKind::Text,
                },
            );

            // encoding
            hf_map.insert(
                format!("{prefix}.encoding"),
                HeaderField {
                    name: "Encoding".into(),
                    kind: FieldKind::Text,
                },
            );

            // payload
            hf_map.insert(
                format!("{prefix}.payload"),
                HeaderField {
                    name: "Payload".into(),
                    kind: FieldKind::Text,
                },
            );

            hf_map
        }
    }

    impl GenerateHFMap for Put {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for Put {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            if let Some(timestamp) = self.timestamp {
                let hf_index = args.get_hf(&format!("{prefix}.timestamp"))?;
                unsafe {
                    epan_sys::proto_tree_add_string(
                        args.tree,
                        hf_index,
                        args.tvb,
                    args.start as _,
                    args.length as _,
                        nul_terminated_str(&format!("{:?}", self.timestamp))?,
                    );
                }
            }

            let hf_index = args.get_hf(&format!("{prefix}.encoding"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&self.encoding.to_string())?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.payload"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.payload))?,
                );
            }

            Ok(())
        }
    }
}

mod impl_for_del {
    use crate::zenoh_impl::*;

    impl Sample for Del {
        fn sample() -> Self {
            Del {
                timestamp: None,
                ext_sinfo: None,
                ext_unknown: vec![],
            }
        }
    }

    impl IntoHFMap for Del {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();

            // timestamp
            hf_map.insert(
                format!("{prefix}.timestamp"),
                HeaderField {
                    name: "Timestamp".into(),
                    kind: FieldKind::Text,
                },
            );
            hf_map
        }
    }

    impl GenerateHFMap for Del {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for Del {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            if let Some(timestamp) = self.timestamp {
                let hf_index = args.get_hf(&format!("{prefix}.timestamp"))?;
                unsafe {
                    epan_sys::proto_tree_add_string(
                        args.tree,
                        hf_index,
                        args.tvb,
                        args.start as _,
                        args.length as _,
                        nul_terminated_str(&format!("{:?}", self.timestamp))?,
                    );
                }
            }

            Ok(())
        }
    }
}

mod impl_for_push_body {
    use crate::zenoh_impl::*;

    impl Sample for PushBody {
        fn sample() -> Self {
            PushBody::Put(Put::sample())
        }
    }

    impl IntoHFMap for PushBody {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();
            hf_map.insert(
                format!("{prefix}"),
                HeaderField {
                    name: "PushBody".into(),
                    kind: FieldKind::Branch,
                },
            );
            let branches = {
                match self {
                    Self::Put(body) => body.into_hf_map(&format!("{prefix}.put")),
                    Self::Del(body) => body.into_hf_map(&format!("{prefix}.del")),
                }
            };
            hf_map.extend(branches);
            hf_map
        }
    }

    impl GenerateHFMap for PushBody {
        fn span() -> Span<Self> {
            Span::Enum(vec![Self::Put(Put::sample()), Self::Del(Del::sample())])
        }
    }

    impl AddToTree for PushBody {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            match self {
                PushBody::Put(body) => body.add_to_tree(
                    &format!("{prefix}.put"),
                    &args.make_subtree(prefix, &format!("PushBody (Put)"))?
                ),
                PushBody::Del(body) => body.add_to_tree(
                    &format!("{prefix}.del"),
                    &args.make_subtree(prefix, &format!("PushBody (Del)"))?
                ),
            }
        }
    }
}

mod impl_for_push {
    use crate::zenoh_impl::*;

    impl Sample for Push {
        fn sample() -> Self {
            Push::rand()
        }
    }

    impl IntoHFMap for Push {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();

            // wire_expr
            hf_map.insert(
                format!("{prefix}.wire_expr"),
                HeaderField {
                    name: "WireExpr".into(),
                    kind: FieldKind::Text,
                },
            );

            // ext_qos
            hf_map.insert(
                format!("{prefix}.ext_qos"),
                HeaderField {
                    name: "ExtQoS".into(),
                    kind: FieldKind::Text,
                },
            );

            // ext_tstamp
            hf_map.insert(
                format!("{prefix}.ext_tstamp"),
                HeaderField {
                    name: "ExtTimestampType".into(),
                    kind: FieldKind::Text,
                },
            );

            // ext_nodeid
            hf_map.insert(
                format!("{prefix}.ext_nodeid"),
                HeaderField {
                    name: "ExtNodeId".into(),
                    kind: FieldKind::Text,
                },
            );

            // payload
            hf_map.extend(PushBody::generate_hf_map(&format!("{prefix}.payload")));

            hf_map
        }
    }

    impl GenerateHFMap for Push {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for Push {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            let hf_index = args.get_hf(&format!("{prefix}.wire_expr"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(self.wire_expr.as_str())?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.ext_qos"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.ext_qos))?,
                );
            }

            if let Some(ext_tstamp) = self.ext_tstamp {
                let hf_index = args.get_hf(&format!("{prefix}.ext_tstamp"))?;
                unsafe {
                    epan_sys::proto_tree_add_string(
                        args.tree,
                        hf_index,
                        args.tvb,
                        args.start as _,
                        args.length as _,
                        nul_terminated_str(&format!("{:?}", ext_tstamp))?,
                    );
                }
            }

            let hf_index = args.get_hf(&format!("{prefix}.ext_nodeid"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.ext_nodeid))?,
                );
            }

            self.payload
                .add_to_tree(&format!("{prefix}.payload"), args)?;

            Ok(())
        }
    }
}

mod impl_for_network_body {
    use crate::zenoh_impl::*;

    impl Sample for NetworkBody {
        fn sample() -> Self {
            NetworkBody::Push(Push::sample())
        }
    }

    impl IntoHFMap for NetworkBody {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();
            hf_map.insert(
                format!("{prefix}"),
                HeaderField {
                    name: "NetworkBody".into(),
                    kind: FieldKind::Branch,
                },
            );
            let branches = {
                match self {
                    Self::Push(body) => body.into_hf_map(&format!("{prefix}.push")),
                    _ => {
                        todo!()
                    }
                }
            };
            hf_map.extend(branches);
            hf_map
        }
    }

    impl GenerateHFMap for NetworkBody {
        fn span() -> Span<Self> {
            Span::Enum(vec![Self::Push(Push::sample())])
        }
    }

    impl AddToTree for NetworkBody {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            match self {
                NetworkBody::Push(body) => body.add_to_tree(
                    &format!("{prefix}.push"),
                    &args.make_subtree(prefix, &format!("NetworkBody (Push)"))?,
                ),
                _ => bail!("Not implemented yet"),
            }
        }
    }
}

mod impl_for_frame {
    use crate::zenoh_impl::*;

    impl Sample for Frame {
        fn sample() -> Self {
            Frame::rand()
        }
    }

    impl IntoHFMap for Frame {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            let mut hf_map = HeaderFieldMap::new();

            // reliability
            hf_map.insert(
                format!("{prefix}.reliability"),
                HeaderField {
                    name: "Reliability".into(),
                    kind: FieldKind::Text,
                },
            );

            // sn
            hf_map.insert(
                format!("{prefix}.sn"),
                HeaderField {
                    name: "TransportSn".into(),
                    kind: FieldKind::Number,
                },
            );

            // ext_qos
            hf_map.insert(
                format!("{prefix}.ext_qos"),
                HeaderField {
                    name: "QoSType".into(),
                    kind: FieldKind::Text,
                },
            );

            // payload
            hf_map.insert(
                format!("{prefix}.payload"),
                HeaderField {
                    name: "Payload".into(),
                    kind: FieldKind::Branch,
                },
            );
            hf_map.extend(NetworkMessage::generate_hf_map(&format!(
                "{prefix}.payload"
            )));

            hf_map
        }
    }

    impl GenerateHFMap for Frame {
        fn span() -> Span<Self> {
            Span::Enum(vec![Self::sample()])
        }
    }

    impl AddToTree for Frame {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            let hf_index = args.get_hf(&format!("{prefix}.reliability"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.reliability))?,
                );
            }

            let hf_index = args.get_hf(&format!("{prefix}.sn"))?;
            unsafe {
                epan_sys::proto_tree_add_uint(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    self.sn.into(),
                );
            }

            // payload
            let payload_args = args.make_subtree(&format!("{prefix}.payload"), &format!("Payload"))?;
            for msg in &self.payload {
                msg.add_to_tree(&format!("{prefix}.payload"), &payload_args)?;
            }

            // ext_qos
            let hf_index = args.get_hf(&format!("{prefix}.ext_qos"))?;
            unsafe {
                epan_sys::proto_tree_add_string(
                    args.tree,
                    hf_index,
                    args.tvb,
                    args.start as _,
                    args.length as _,
                    nul_terminated_str(&format!("{:?}", self.ext_qos))?,
                );
            }

            Ok(())
        }
    }
}

mod impl_for_network_message {
    use crate::zenoh_impl::*;

    impl Sample for NetworkMessage {
        fn sample() -> Self {
            NetworkMessage::rand()
        }
    }

    impl IntoHFMap for NetworkMessage {
        fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
            NetworkBody::generate_hf_map(&format!("{prefix}.body"))
        }
    }

    impl GenerateHFMap for NetworkMessage {
        fn span() -> Span<Self> {
            Span::Struct(Self::sample())
        }
    }

    impl AddToTree for NetworkMessage {
        fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()> {
            self.body.add_to_tree(&format!("{prefix}.body"), args)
        }
    }
}
