#![allow(dead_code)]
#![allow(unused)]

// use zenoh_protocol::transport::frame::Frame;
use zenoh_protocol::{core::Reliability, transport::Frame};
use zenoh_protocol::common::imsg;
use zenoh_codec::{RCodec, WCodec, Zenoh080};
use zenoh_buffers::{
    reader::HasReader,
    writer::{HasWriter, Writer},
};
use zenoh_buffers::ZSlice;
use zenoh_protocol::transport::TransportMessage;
use zenoh::value::Value;

use wsdf::tap::{Field, Offset, Packet};
use wsdf::{version, Dispatch, Protocol, ProtocolField};

wsdf::version!("0.0.1", 4, 0);


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
    frame: MyFrame,

    // // frame: Vec<u8>
    // header: u8,
    //
    // #[wsdf(enc = "ENC_LITTLE_ENDIAN", dispatch_field = "header")]
    // body: Body,

    // #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    // frame: MyFrame,
}

#[derive(ProtocolField, Dispatch)]
enum Body {
    Frame(MyFrame),
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
struct MyFrame(#[wsdf(
    consume_with = "decode_frame",
    // enc = "ENC_LITTLE_ENDIAN"
)] Vec<u8>);

fn decode_frame(Packet(packet): Packet, Offset(offset): Offset) -> (usize, String) {
    // dbg!(offset);
    let buf = packet[offset..].to_vec();
    // dbg!(&buf);
    let mut reader = buf.reader();
    let codec = Zenoh080::new();
    let msg: TransportMessage = codec
        .read(&mut reader)
        .expect("Failed to read!!!!!!!!!");

    dbg!(&msg);

    if let zenoh_protocol::transport::TransportBody::Frame(frame) = &msg.body {
        for nm in &frame.payload {
            if let zenoh_protocol::network::NetworkBody::Push(push) = &nm.body {
                if let zenoh_protocol::zenoh_new::PushBody::Put(put) = &push.payload {
                    let value = Value::from(put.payload.clone());
                    return (0, format!("{value}"))
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
