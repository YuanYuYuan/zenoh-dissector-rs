#![allow(dead_code)]
#![allow(unused)]

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
#[wsdf(decode_from = [("tcp.port", 7447)])]
struct MyZenoh {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    len: u16,
    frame: MyFrame,
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

#[derive(ProtocolField)]
struct MyFrame(#[wsdf(
    consume_with = "decode_frame",
    // enc = "ENC_LITTLE_ENDIAN"
)] Vec<u8>);

fn decode_frame(Packet(packet): Packet, Offset(offset): Offset) -> (usize, String) {
    let buf = packet[offset..].to_vec();
    let mut reader = buf.reader();
    let codec = Zenoh080::new();
    let msg: TransportMessage = codec
        .read(&mut reader)
        .expect("Failed to read!!!!!!!!!");

    dbg!(&msg);

    // An example for filtering out desired messages
    if let zenoh_protocol::transport::TransportBody::Frame(frame) = &msg.body {
        for nm in &frame.payload {
            if let zenoh_protocol::network::NetworkBody::Push(push) = &nm.body {
                if let zenoh_protocol::zenoh_new::PushBody::Put(put) = &push.payload {
                    let value = Value::from(put.payload.clone());
                    return (0, format!("{value}"))
                }
            }
        }
    }

    let res = format!("{msg:?}");
    (0, res)
}
