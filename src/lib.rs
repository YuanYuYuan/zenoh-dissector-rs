#![allow(dead_code)]
#![allow(unused)]

use std::fmt::format;

use wsdf::{protocol, version, Dissect, Proto, tap::{FieldsLocal, Fields, Field}};

version!("0.0.1", 4, 0);
protocol!(MyZenoh);

// use zenoh_protocol::transport::frame::Frame;
use zenoh::value::Value;
use zenoh_buffers::ZSlice;
use zenoh_buffers::{
    reader::HasReader,
    writer::{HasWriter, Writer},
};
use zenoh_codec::{RCodec, WCodec, Zenoh080, Zenoh080Header};
use zenoh_protocol::common::imsg;
use zenoh_protocol::transport::{FrameHeader, TransportMessage};
use zenoh_protocol::{core::Reliability, transport::Frame};


#[derive(Dissect, Proto)]
#[wsdf(
    // decode_from = [("ip.proto", 6)],
    // decode_from = ["tcp.port"],
    decode_from = [("tcp.port", 7447)],
)]
struct MyZenoh {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    len: u16,

    transport_message: MyTransportMessage,
}

#[derive(Dissect)]
struct MyTransportMessage {

    #[wsdf(
        enc = "ENC_LITTLE_ENDIAN",
        decode_with = "decode_my_transport_message_header",
        save,
    )]
    header: u8,

    #[wsdf(
        enc = "ENC_LITTLE_ENDIAN",
        get_variant = "get_variant_header"
    )]
    body: MyTransportBody,
}

fn decode_my_transport_message_header(Field(header): Field<u8>) -> String {
    // format!("{}\n{}", header, header).to_string()
    format!("
TransportMessage
body: Frame(
Frame
reliability: Reliable,
sn: 106702511,
payload: [
NetworkMessage
body: Push(
Push
wire_expr: demo/example/zenoh-rs-put,
ext_qos: QoS
priority: Data,
congestion: Drop,
express: false,
,
ext_tstamp: None,
ext_nodeid: NodeIdType
node_id: 0,
,
payload: Put(
Put
timestamp: None,
encoding: Exact(
TextPlain,
),
ext_sinfo: None,
ext_unknown: [],
payload: ZBuf
slices: [[50, 75, 74, 20, 66, 72, 6f, 6d, 20, 52, 75, 73, 74, 21]],
,
,
),
,
),
,
],
ext_qos: QoSType
inner: 5,
,
,
),
    ")
}

fn get_variant_header(FieldsLocal(fields): FieldsLocal) -> &'static str {
    let header = *fields.get_u8("header").unwrap();
    use zenoh_protocol::transport::id;
    match imsg::mid(header) {
        id::FRAME => "Frame",
        id::FRAGMENT => "Fragment",
        id::KEEP_ALIVE => "KeepAlive",
        id::INIT => {
            if !imsg::has_flag(header, zenoh_protocol::transport::init::flag::A) {
                "InitSyn"
            } else {
                "InitAck"
            }
        }
        id::OPEN => {
            if !imsg::has_flag(header, zenoh_protocol::transport::open::flag::A) {
                "OpenSyn"
            } else {
                "OpenAck"
            }
        }
        id::CLOSE => "Close",
        id::OAM => "OAM",
        id::JOIN => "Join",
        _ => "Error",
    }
}

#[derive(Dissect)]
enum MyTransportBody {
    Frame,
    Fragment,
    KeepAlive,
    InitSyn,
    InitAck,
    OpenSyn,
    OpenAck,
    Close,
    OAM,
    Join,
    Error,
}

// #[derive(Dissect)]
// struct MyFrame {
//     #[wsdf(
//         get_variant = "get_variant_reliability"
//     )]
//     reliability: MyReliability,
// }

// #[derive(Dissect)]
// enum MyReliability {
//     BestEffort,
//     Reliable
// }

// fn get_variant_reliability(FieldsLocal(fields): FieldsLocal) -> &'static str {
//     let header = *fields.get_u8("my_zenoh.transport_message.header").unwrap();
//     let codec = Zenoh080Header::new(header);
// }

// impl MyTransportBody {
//     fn dispatch_header(header: &u8) -> MyTransportBodyDispatch {
//         use zenoh_protocol::transport::id;
//         let codec = Zenoh080::new();
//         let buf = vec![*header];
//         let mut reader = buf.reader();
//         let header: u8 = codec
//             .read(&mut reader)
//             .expect("Failed to read my frame header");
//         let codec = Zenoh080Header::new(header);
//         match imsg::mid(codec.header) {
//             id::FRAME => MyTransportBodyDispatch::Frame,
//             id::FRAGMENT => MyTransportBodyDispatch::Fragment,
//             id::KEEP_ALIVE => MyTransportBodyDispatch::KeepAlive,
//             id::INIT => {
//                 if !imsg::has_flag(codec.header, zenoh_protocol::transport::init::flag::A) {
//                     MyTransportBodyDispatch::InitSyn
//                 } else {
//                     MyTransportBodyDispatch::InitAck
//                 }
//             }
//             id::OPEN => {
//                 if !imsg::has_flag(codec.header, zenoh_protocol::transport::open::flag::A) {
//                     MyTransportBodyDispatch::OpenSyn
//                 } else {
//                     MyTransportBodyDispatch::OpenAck
//                 }
//             }
//             id::CLOSE => MyTransportBodyDispatch::Close,
//             id::OAM => MyTransportBodyDispatch::OAM,
//             id::JOIN => MyTransportBodyDispatch::Join,
//             _ => MyTransportBodyDispatch::Error,
//         }
//     }
// }
//
// #[derive(ProtocolField)]
// struct MyFrame {
//     #[wsdf(enc = "ENC_LITTLE_ENDIAN", decode_with = "decode_my_frame_header")]
//     header: u8,
// }
//
//
//
// #[derive(ProtocolField, Dispatch)]
// enum Body {
//     Frame(FooFrame),
//     Other,
// }
//
// impl Body {
//     fn dispatch_header(header: &u8) -> BodyDispatch {
//         match imsg::mid(*header) {
//             zenoh_protocol::transport::id::FRAME => BodyDispatch::Frame,
//             _ => BodyDispatch::Other,
//         }
//     }
// }
//
// // // #[repr(u8)]
// // #[derive(ProtocolField)]
// // enum Reliability {
// //     BestEffort(u8),
// //     Reliable(u8)
// // }
//
// #[derive(ProtocolField)]
// struct FooFrame(
//     #[wsdf(
//     consume_with = "decode_frame",
//     // enc = "ENC_LITTLE_ENDIAN"
// )]
//     Vec<u8>,
// );
//
// fn decode_frame(Packet(packet): Packet, Offset(offset): Offset) -> (usize, String) {
//     // dbg!(offset);
//     let buf = packet[offset..].to_vec();
//     // dbg!(&buf);
//     let mut reader = buf.reader();
//     let codec = Zenoh080::new();
//     let msg: TransportMessage = codec.read(&mut reader).expect("Failed to read!!!!!!!!!");
//
//     dbg!(&msg);
//
//     if let zenoh_protocol::transport::TransportBody::Frame(frame) = &msg.body {
//         for nm in &frame.payload {
//             if let zenoh_protocol::network::NetworkBody::Push(push) = &nm.body {
//                 if let zenoh_protocol::zenoh_new::PushBody::Put(put) = &push.payload {
//                     let value = Value::from(put.payload.clone());
//                     return (0, format!("{value}"));
//                     // let payload = &put.payload;
//                 }
//             }
//         }
//     }
//     let res = format!("{msg:?}");
//     (0, res)
// }
//
// // #[derive(wsdf::ProtocolField)]
// // pub enum Reliability {
// //     BestEffort(u8),
// //     Reliable(u8),
// // }
// //
// // #[derive(wsdf::ProtocolField)]
// // struct MyFrame {
// //     reliability: Reliability
// // }
