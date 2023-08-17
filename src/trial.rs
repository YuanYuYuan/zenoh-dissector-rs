#![allow(dead_code)]
#![allow(unused)]

use std::fmt::format;

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

use wsdf::tap::{Field, Offset, Packet};
use wsdf::{version, Dispatch, Protocol, ProtocolField};

wsdf::version!("0.0.1", 4, 0);

#[derive(ProtocolField)]
struct MyTransportMessage {

    #[wsdf(
        enc = "ENC_LITTLE_ENDIAN",
        decode_with = "decode_my_transport_message_header",
    )]
    header: u8,

    #[wsdf(enc = "ENC_LITTLE_ENDIAN", dispatch_field = "header")]
    body: MyTransportBody,
}

fn decode_my_transport_message_header(Field(header): Field<u8>) -> String {
    use zenoh_protocol::transport::id;
    let str = match imsg::mid(header) {
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
    };
    str.to_string()
}

#[derive(ProtocolField, Dispatch)]
enum MyTransportBody {
    Frame(MyFrame),
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

impl MyTransportBody {
    fn dispatch_header(header: &u8) -> MyTransportBodyDispatch {
        use zenoh_protocol::transport::id;
        let codec = Zenoh080::new();
        let buf = vec![*header];
        let mut reader = buf.reader();
        let header: u8 = codec
            .read(&mut reader)
            .expect("Failed to read my frame header");
        let codec = Zenoh080Header::new(header);
        match imsg::mid(codec.header) {
            id::FRAME => MyTransportBodyDispatch::Frame,
            id::FRAGMENT => MyTransportBodyDispatch::Fragment,
            id::KEEP_ALIVE => MyTransportBodyDispatch::KeepAlive,
            id::INIT => {
                if !imsg::has_flag(codec.header, zenoh_protocol::transport::init::flag::A) {
                    MyTransportBodyDispatch::InitSyn
                } else {
                    MyTransportBodyDispatch::InitAck
                }
            }
            id::OPEN => {
                if !imsg::has_flag(codec.header, zenoh_protocol::transport::open::flag::A) {
                    MyTransportBodyDispatch::OpenSyn
                } else {
                    MyTransportBodyDispatch::OpenAck
                }
            }
            id::CLOSE => MyTransportBodyDispatch::Close,
            id::OAM => MyTransportBodyDispatch::OAM,
            id::JOIN => MyTransportBodyDispatch::Join,
            _ => MyTransportBodyDispatch::Error,
        }
    }
}

#[derive(ProtocolField)]
struct MyFrame {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN", decode_with = "decode_my_frame_header")]
    header: u8,
}

fn decode_my_frame_header(Fields(x): Fields) -> String {
    format!("{}", header).to_string()
    // let codec = Zenoh080::new();
    // let buf = vec![header];
    // let mut reader = buf.reader();
    // dbg!(&buf);
    // let frame_header: FrameHeader = codec
    //     .read(&mut reader)
    //     .expect("Failed to read my frame header");
    // dbg!(frame_header);
    // // let mut buf = codec.read(&[header]);
    // "".to_string()
}

#[derive(Protocol)]
#[wsdf(
    // decode_from = [("ip.proto", 6)],
    // decode_from = ["tcp.port"],
    decode_from = [("tcp.port", 7447)],
)]
struct MyZenoh {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    len: u16,
    // src_port: u16,
    // dst_port: u16,
    // length: u16,
    // checksum: u16,
    //
    // #[wsdf(subdissector = ("MyZenohWTF", "dst_port", "src_port"))]
    // #[wsdf(len_field = "len")]
    // frame: Vec<u8>,
    // frame: FooFrame,

    transport_message: MyTransportMessage,

    // #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    // frame: FooFrame,
}

#[derive(ProtocolField, Dispatch)]
enum Body {
    Frame(FooFrame),
    Other,
}

impl Body {
    fn dispatch_header(header: &u8) -> BodyDispatch {
        match imsg::mid(*header) {
            zenoh_protocol::transport::id::FRAME => BodyDispatch::Frame,
            _ => BodyDispatch::Other,
        }
    }
}

// // #[repr(u8)]
// #[derive(ProtocolField)]
// enum Reliability {
//     BestEffort(u8),
//     Reliable(u8)
// }

#[derive(ProtocolField)]
struct FooFrame(
    #[wsdf(
    consume_with = "decode_frame",
    // enc = "ENC_LITTLE_ENDIAN"
)]
    Vec<u8>,
);

fn decode_frame(Packet(packet): Packet, Offset(offset): Offset) -> (usize, String) {
    // dbg!(offset);
    let buf = packet[offset..].to_vec();
    // dbg!(&buf);
    let mut reader = buf.reader();
    let codec = Zenoh080::new();
    let msg: TransportMessage = codec.read(&mut reader).expect("Failed to read!!!!!!!!!");

    dbg!(&msg);

    if let zenoh_protocol::transport::TransportBody::Frame(frame) = &msg.body {
        for nm in &frame.payload {
            if let zenoh_protocol::network::NetworkBody::Push(push) = &nm.body {
                if let zenoh_protocol::zenoh_new::PushBody::Put(put) = &push.payload {
                    let value = Value::from(put.payload.clone());
                    return (0, format!("{value}"));
                    // let payload = &put.payload;
                }
            }
        }
    }
    let res = format!("{msg:?}");
    (0, res)
}

// #[derive(wsdf::ProtocolField)]
// pub enum Reliability {
//     BestEffort(u8),
//     Reliable(u8),
// }
//
// #[derive(wsdf::ProtocolField)]
// struct MyFrame {
//     reliability: Reliability
// }
