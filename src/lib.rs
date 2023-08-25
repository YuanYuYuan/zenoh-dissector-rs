#![allow(dead_code)]
#![allow(unused)]

use std::borrow::BorrowMut;
use std::fmt::format;

use std::ffi::{c_char, c_int, c_uint, c_void, CString};

use wsdf::tap::{Offset, Packet};
use wsdf::{
    protocol,
    tap::{Field, Fields, FieldsLocal},
    version, Dissect, Proto,
};

version!("0.0.2", 4, 0);
protocol!(MyZenoh);

// use zenoh_protocol::transport::frame::Frame;
use zenoh::value::Value;
use zenoh_buffers::ZSlice;
use zenoh_buffers::{
    reader::HasReader,
    writer::{HasWriter, Writer},
};
// use zenoh_codec::{RCodec, WCodec, Zenoh080, Zenoh080Header};
use zenoh_protocol::common::imsg;
use zenoh_protocol::transport::{FrameHeader, TransportMessage};
use zenoh_protocol::{core::Reliability, transport::Frame};

#[derive(Dissect, Proto)]
#[wsdf(decode_from = [("tcp.port", 7447)])]
struct MyZenoh {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN", rename = "len", decode_with = "decode_with")]
    len2: u16,

    // #[wsdf(bytes, consume_with = "consume_bytes")]
    // some_data: Vec<u8>,
    msg: MyMessage,
}

fn decode_with(Field(len2): Field<u16>) -> &'static str {
    "Hey"
}

fn consume_bytes(Offset(offset): Offset, Packet(pkt): Packet) -> (usize, String) {
    unimplemented!()
}

#[derive(Default, Debug)]
struct MyMessage {
    body: MyBody,
}

#[derive(Debug)]
enum MyBody {
    InitSyn(MyInitSyn),
    Frame(MyFrame),
}

impl Default for MyBody {
    fn default() -> Self {
        MyBody::InitSyn(MyInitSyn::default())
    }
}

#[derive(Default, Debug)]
struct MyInitSyn {
    version: u8,
    whatami: u8,
}

#[derive(Default, Debug)]
struct MyFrame {
    payload: Vec<u8>,
}

// struct MyHeaderFieldInfo(epan_sys::header_field_info) {
//
// }

use std::sync::atomic::{AtomicUsize, Ordering};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

impl<'tvb> wsdf::Dissect<'tvb, ()> for MyMessage {
    type Emit = ();

    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        // dbg!("================== add_to_tree");


        // let parent = args.add_subtree();
        // let prefix_next = "my_zenoh.msg.body.init_syn";

        // let args = wsdf::DissectorArgs {
        //     hf_indices: args.hf_indices,
        //     etts: args.etts,
        //     dtables: args.dtables,
        //     tvb: args.tvb,
        //     pinfo: args.pinfo,
        //     proto_root: args.proto_root,
        //     data: args.data,
        //     prefix: "my_zenoh.msg.body",
        //     prefix_local: "body",
        //     offset: args.offset,
        //     parent: args.parent,
        //     variant: std::option::Option::None,
        //     list_len: std::option::Option::None,
        //     ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        // };


        // unsafe {
        //     epan_sys::proto_tree_add_uint_format_value(
        //         args.parent,
        //         hf_index,
        //         args.tvb,
        //         args.offset as _,
        //         3,
        //         1234,
        //         nul_terminated_str("hello"),
        //     );
        // }

        // let parent = unsafe {
        //     epan_sys::proto_tree_add_item(
        //         args.parent,
        //         hf_index,
        //         args.tvb,
        //         args.offset as _,
        //         3,
        //         epan_sys::ENC_NA,
        //     )
        // };

        let is_even = CALL_COUNT.fetch_add(1, Ordering::SeqCst) % 2 == 0;

        let mut etts = args.etts.0.clone();

        let prefix = "my_zenoh.msg.body";

        let hf_index = args
            .hf_indices
            .get(prefix)
            .expect(&format!("{:?}", &args.hf_indices));
        let ett = if let Some(ett) = etts.get(prefix) {
            *ett
        } else {
            let ett_index_ptr = Box::leak(Box::new(-1)) as *mut _;
            unsafe {
                epan_sys::proto_register_subtree_array([ett_index_ptr].as_mut_ptr(), 1);
            }
            let ett_index = unsafe { *ett_index_ptr };
            debug_assert_ne!(ett_index, -1);
            etts.insert(prefix.to_string(), ett_index);
            ett_index
        };

        let subtree = unsafe {
            let ti = epan_sys::proto_tree_add_none_format(
                args.parent,
                hf_index,
                args.tvb,
                args.offset as _,
                3,
                if is_even {
                    nul_terminated_str("Body (InitSyn)")
                } else {
                    nul_terminated_str("Body (Frame)")
                },
            );
            let subtree = epan_sys::proto_item_add_subtree(
                ti,
                ett,
            );
            // let subtree = epan_sys::proto_item_add_subtree(
            //     args.parent,
            //     ett,
            // );
            subtree
        };

        if is_even {
            let prefix = "my_zenoh.msg.body.init_syn.version";
            let hf_index = args
                .hf_indices
                .get(prefix)
                .expect(&format!("{:?}", &args.hf_indices));

            unsafe {
                epan_sys::proto_tree_add_uint_format_value(
                    subtree,
                    hf_index,
                    args.tvb,
                    args.offset as _,
                    3,
                    45678,
                    nul_terminated_str("This is version!"),
                );
            }

            let prefix = "my_zenoh.msg.body.init_syn.whatami";
            let hf_index = args
                .hf_indices
                .get(prefix)
                .expect(&format!("{:?}", &args.hf_indices));
            unsafe {
                epan_sys::proto_tree_add_uint_format_value(
                    subtree,
                    hf_index,
                    args.tvb,
                    (args.offset + 3) as _,
                    3,
                    45678,
                    nul_terminated_str("This is WhatAmI"),
                );
            }
        } else {
            let prefix = "my_zenoh.msg.body.frame.payload";
            let hf_index = args
                .hf_indices
                .get(prefix)
                .expect(&format!("{:?}", &args.hf_indices));

            unsafe {
                epan_sys::proto_tree_add_uint_format_value(
                    subtree,
                    hf_index,
                    args.tvb,
                    args.offset as _,
                    3,
                    45678,
                    nul_terminated_str("This is payload!"),
                );
            }
        }

        2
    }

    fn size(args: &wsdf::DissectorArgs<'_, 'tvb>, fields: &mut wsdf::FieldsStore<'tvb>) -> usize {
        3
    }

    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        // dbg!("============ register ==============");
        // dbg!(args);
        // dbg!(&ws_indices);

        // let _ = ws_indices.ett.get_or_create_ett(args);

        // register_helper(
        //     ws_indices,
        //     args.proto_id,
        //     "my_zenoh.msg.body",
        //     "Body",
        //     // epan_sys::field_display_e_BASE_NONE as _,
        //     // epan_sys::ftenum_FT_UINT_STRING,
        //     epan_sys::field_display_e_BASE_DEC as _,
        //     epan_sys::ftenum_FT_UINT8,
        // );

        register_helper(
            ws_indices,
            args.proto_id,
            "my_zenoh.msg",
            "MyMessage",
            epan_sys::field_display_e_BASE_NONE as _,
            epan_sys::ftenum_FT_NONE,
        );

        register_helper(
            ws_indices,
            args.proto_id,
            "my_zenoh.msg.body",
            "Body",
            epan_sys::field_display_e_BASE_NONE as _,
            epan_sys::ftenum_FT_NONE,
        );

        register_helper(
            ws_indices,
            args.proto_id,
            "my_zenoh.msg.body.init_syn.version",
            "Version",
            epan_sys::field_display_e_BASE_DEC as _,
            epan_sys::ftenum_FT_UINT8,
        );

        register_helper(
            ws_indices,
            args.proto_id,
            "my_zenoh.msg.body.init_syn.whatami",
            "WhatAmI",
            epan_sys::field_display_e_BASE_DEC as _,
            epan_sys::ftenum_FT_UINT8,
        );

        register_helper(
            ws_indices,
            args.proto_id,
            "my_zenoh.msg.body.frame.payload",
            "Payload",
            epan_sys::field_display_e_BASE_DEC as _,
            epan_sys::ftenum_FT_UINT8,
        );

        // let args_next = wsdf::RegisterArgs {
        //     proto_id: args.proto_id,
        //     name: "my_zenoh.msg.body.init_syn".as_ptr() as _,
        //     prefix: "my_zenoh.msg.body.init_syn",
        //     blurb: std::ptr::null(),
        //     ws_type: std::option::Option::None,
        //     ws_display: std::option::Option::None,
        // };
        // let hf_index = wsdf::register_hf_index(
        //     &args_next,
        //     epan_sys::field_display_e_BASE_DEC as _,
        //     epan_sys::ftenum_FT_UINT8,
        // );
        //
        // ws_indices.hf.insert(args_next.prefix, hf_index);
    }

    fn emit(_args: &wsdf::DissectorArgs) {}
}

fn nul_terminated_str(s: &str) -> *const i8 {
    Box::leak(CString::new(s).unwrap().into_boxed_c_str()).as_ptr()
}

fn register_helper<'a>(
    ws_indices: &mut wsdf::WsIndices,
    proto_id: i32,
    field_key: &'a str,
    field_name: &'a str,
    field_enc: c_int,
    field_typ: c_uint,
) {
    // let name = Box::leak(CString::new(field_name).unwrap().into_boxed_c_str()).as_ptr();
    let args_next = wsdf::RegisterArgs {
        proto_id,
        name: nul_terminated_str(field_name),
        // name: "WTFFFFFFF\u{0}".as_ptr() as _,
        prefix: field_key,
        blurb: std::ptr::null(),
        ws_type: std::option::Option::None,
        ws_display: std::option::Option::None,
    };
    // let _ = ws_indices.ett.get_or_create_ett(&args_next);
    // dbg!(&args_next);
    let hf_index = wsdf::register_hf_index(&args_next, field_enc, field_typ);

    ws_indices.hf.insert(args_next.prefix, hf_index);
}
