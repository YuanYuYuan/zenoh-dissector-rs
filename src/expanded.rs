#![feature(prelude_import)]
#![allow(dead_code)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::format;
use wsdf::{protocol, version, Dissect, Proto, tap::{FieldsLocal, Fields, Field}};
#[no_mangle]
#[used]
static plugin_version: [std::ffi::c_char; 6usize] = [48i8, 46i8, 48i8, 46i8, 49i8, 0i8];
#[no_mangle]
#[used]
static plugin_want_major: std::ffi::c_int = 4;
#[no_mangle]
#[used]
static plugin_want_minor: std::ffi::c_int = 0;
const __WSDF_HF_INDICES: ::std::thread::LocalKey<std::cell::RefCell<wsdf::HfIndices>> = {
    #[inline]
    fn __init() -> std::cell::RefCell<wsdf::HfIndices> {
        wsdf::HfIndices::default().into()
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<
            &mut ::std::option::Option<std::cell::RefCell<wsdf::HfIndices>>,
        >,
    ) -> ::std::option::Option<&'static std::cell::RefCell<wsdf::HfIndices>> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<
            std::cell::RefCell<wsdf::HfIndices>,
        > = ::std::thread::local_impl::Key::<std::cell::RefCell<wsdf::HfIndices>>::new();
        #[allow(unused_unsafe)]
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if true {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
const __WSDF_ETT_INDICES: ::std::thread::LocalKey<
    std::cell::RefCell<wsdf::EttIndices>,
> = {
    #[inline]
    fn __init() -> std::cell::RefCell<wsdf::EttIndices> {
        wsdf::EttIndices::default().into()
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<
            &mut ::std::option::Option<std::cell::RefCell<wsdf::EttIndices>>,
        >,
    ) -> ::std::option::Option<&'static std::cell::RefCell<wsdf::EttIndices>> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<
            std::cell::RefCell<wsdf::EttIndices>,
        > = ::std::thread::local_impl::Key::<
            std::cell::RefCell<wsdf::EttIndices>,
        >::new();
        #[allow(unused_unsafe)]
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if true {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
const __WSDF_DTABLES: ::std::thread::LocalKey<
    std::cell::RefCell<wsdf::DissectorTables>,
> = {
    #[inline]
    fn __init() -> std::cell::RefCell<wsdf::DissectorTables> {
        wsdf::DissectorTables::default().into()
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<
            &mut ::std::option::Option<std::cell::RefCell<wsdf::DissectorTables>>,
        >,
    ) -> ::std::option::Option<&'static std::cell::RefCell<wsdf::DissectorTables>> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<
            std::cell::RefCell<wsdf::DissectorTables>,
        > = ::std::thread::local_impl::Key::<
            std::cell::RefCell<wsdf::DissectorTables>,
        >::new();
        #[allow(unused_unsafe)]
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if true {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
#[no_mangle]
extern "C" fn plugin_register() {
    static mut PLUG_0: wsdf::epan_sys::proto_plugin = wsdf::epan_sys::proto_plugin {
        register_protoinfo: None,
        register_handoff: None,
    };
    unsafe {
        PLUG_0
            .register_protoinfo = std::option::Option::Some(
            <MyZenoh as wsdf::Proto>::register_protoinfo,
        );
        PLUG_0
            .register_handoff = std::option::Option::Some(
            <MyZenoh as wsdf::Proto>::register_handoff,
        );
        wsdf::epan_sys::proto_register_plugin(&PLUG_0);
    }
}
use zenoh::value::Value;
use zenoh_buffers::ZSlice;
use zenoh_buffers::{reader::HasReader, writer::{HasWriter, Writer}};
use zenoh_codec::{RCodec, WCodec, Zenoh080, Zenoh080Header};
use zenoh_protocol::common::imsg;
use zenoh_protocol::transport::{FrameHeader, TransportMessage};
use zenoh_protocol::{core::Reliability, transport::Frame};
#[wsdf(decode_from = [("tcp.port", 7447)])]
struct MyZenoh {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    len: u16,
    transport_message: MyTransportMessage,
}
impl<'tvb> wsdf::Dissect<'tvb, ()> for MyZenoh {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.add_subtree();
        let prefix_next = args.prefix.to_owned() + "." + "len";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "len",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let offset = offset
            + <u16 as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        let prefix_next = args.prefix.to_owned() + "." + "transport_message";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "transport_message",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <MyTransportMessage as wsdf::Dissect<
                'tvb,
                (),
            >>::add_to_tree(&args_next, fields);
        unsafe {
            wsdf::epan_sys::proto_item_set_len(parent, (offset - args.offset) as _);
        }
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let prefix_next = args.prefix.to_owned() + "." + "len";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "len",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let offset = offset + <u16 as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        let prefix_next = args.prefix.to_owned() + "." + "transport_message";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "transport_message",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <MyTransportMessage as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let _ = ws_indices.ett.get_or_create_ett(args);
        let _ = ws_indices.hf.get_or_create_text_node(args);
        let prefix_next = args.prefix.to_owned() + "." + "len";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Len\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <u16 as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "transport_message";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Transport Message\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <MyTransportMessage as wsdf::Dissect<
            'tvb,
            (),
        >>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
impl wsdf::Proto for MyZenoh {
    #[allow(clippy::missing_safety_doc)]
    unsafe extern "C" fn dissect_main(
        tvb: *mut wsdf::epan_sys::tvbuff,
        pinfo: *mut wsdf::epan_sys::_packet_info,
        tree: *mut wsdf::epan_sys::_proto_node,
        data: *mut std::ffi::c_void,
    ) -> std::ffi::c_int {
        wsdf::epan_sys::col_set_str(
            (*pinfo).cinfo,
            wsdf::epan_sys::COL_PROTOCOL as std::ffi::c_int,
            "MY ZENOH\u{0}".as_ptr() as *const std::ffi::c_char,
        );
        wsdf::epan_sys::col_clear(
            (*pinfo).cinfo,
            wsdf::epan_sys::COL_INFO as std::ffi::c_int,
        );
        let tvb_len = unsafe { wsdf::epan_sys::tvb_reported_length(tvb) as usize };
        let mut tvb_buf = Vec::new();
        tvb_buf.resize(tvb_len, 0);
        unsafe {
            wsdf::epan_sys::tvb_memcpy(
                tvb,
                tvb_buf.as_mut_ptr() as *mut std::ffi::c_void,
                0,
                tvb_len,
            );
        }
        __WSDF_HF_INDICES
            .with(|hf_indices| {
                __WSDF_ETT_INDICES
                    .with(|etts| {
                        __WSDF_DTABLES
                            .with(|dtables| {
                                let mut fields = wsdf::FieldsStore::default();
                                let hf_indices = hf_indices.borrow();
                                let etts = etts.borrow();
                                let dtables = dtables.borrow();
                                let args = wsdf::DissectorArgs {
                                    hf_indices: &hf_indices,
                                    etts: &etts,
                                    dtables: &dtables,
                                    tvb,
                                    pinfo,
                                    proto_root: tree,
                                    data: &tvb_buf,
                                    prefix: "my_zenoh",
                                    prefix_local: "my_zenoh",
                                    offset: 0,
                                    parent: tree,
                                    variant: std::option::Option::None,
                                    list_len: std::option::Option::None,
                                    ws_enc: std::option::Option::None,
                                };
                                <MyZenoh as Dissect<
                                    '_,
                                    (),
                                >>::add_to_tree(&args, &mut fields) as _
                            })
                    })
            })
    }
    unsafe extern "C" fn register_protoinfo() {
        let proto_id = unsafe {
            wsdf::epan_sys::proto_register_protocol(
                "MY ZENOH\u{0}".as_ptr() as *const std::ffi::c_char,
                "MY ZENOH\u{0}".as_ptr() as *const std::ffi::c_char,
                "my_zenoh\u{0}".as_ptr() as *const std::ffi::c_char,
            )
        };
        __WSDF_HF_INDICES
            .with(|hf_indices| {
                __WSDF_ETT_INDICES
                    .with(|etts| {
                        __WSDF_DTABLES
                            .with(|dtables| {
                                let mut hf = hf_indices.borrow_mut();
                                let mut ett = etts.borrow_mut();
                                let mut dtable = dtables.borrow_mut();
                                let mut ws_indices = wsdf::WsIndices {
                                    hf: &mut hf,
                                    ett: &mut ett,
                                    dtable: &mut dtable,
                                };
                                ws_indices.hf.insert("my_zenoh", proto_id);
                                let args = wsdf::RegisterArgs {
                                    proto_id,
                                    name: "MY ZENOH\u{0}".as_ptr() as *const std::ffi::c_char,
                                    prefix: "my_zenoh",
                                    blurb: std::ptr::null(),
                                    ws_type: std::option::Option::None,
                                    ws_display: std::option::Option::None,
                                };
                                <MyZenoh as Dissect<
                                    '_,
                                    (),
                                >>::register(&args, &mut ws_indices);
                            })
                    })
            });
    }
    unsafe extern "C" fn register_handoff() {
        __WSDF_HF_INDICES
            .with(|hf_indices| {
                let hf_indices = hf_indices.borrow();
                let proto_id = hf_indices.get("my_zenoh").unwrap();
                unsafe {
                    let handle = wsdf::epan_sys::create_dissector_handle(
                        std::option::Option::Some(
                            <MyZenoh as wsdf::Proto>::dissect_main,
                        ),
                        proto_id,
                    );
                    wsdf::epan_sys::dissector_add_uint(
                        "tcp.port\u{0}".as_ptr() as *const std::ffi::c_char,
                        7447u32 as std::ffi::c_uint,
                        handle,
                    );
                }
            });
    }
}
struct MyTransportMessage {
    #[wsdf(
        enc = "ENC_LITTLE_ENDIAN",
        decode_with = "decode_my_transport_message_header",
        save,
    )]
    header: u8,
    #[wsdf(enc = "ENC_LITTLE_ENDIAN", get_variant = "get_variant_header")]
    body: MyTransportBody,
}
impl<'tvb> wsdf::Dissect<'tvb, ()> for MyTransportMessage {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.add_subtree();
        let prefix_next = args.prefix.to_owned() + "." + "header";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "header",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let __header = <u8 as wsdf::Dissect<'tvb, ()>>::emit(&args_next);
        <u8 as wsdf::Primitive<'tvb, ()>>::save(&args_next, fields, &mut fields_local);
        let ctx = wsdf::tap::Context {
            field: __header,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let s = wsdf::tap::handle_decode_with(&ctx, decode_my_transport_message_header);
        let n = <u8 as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        <u8 as wsdf::Primitive<'tvb, ()>>::add_to_tree_format_value(&args_next, &s, n);
        let offset = offset + n;
        let prefix_next = args.prefix.to_owned() + "." + "body";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "body",
            offset,
            parent,
            variant: std::option::Option::Some(
                wsdf::tap::handle_get_variant(
                    &wsdf::tap::Context {
                        field: (),
                        fields,
                        fields_local: &fields_local,
                        pinfo: args.pinfo,
                        packet: args.data,
                        offset,
                    },
                    get_variant_header,
                ),
            ),
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let __body = <MyTransportBody as wsdf::Dissect<'tvb, ()>>::emit(&args_next);
        let ctx = wsdf::tap::Context {
            field: __body,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let offset = offset
            + <MyTransportBody as wsdf::Dissect<
                'tvb,
                (),
            >>::add_to_tree(&args_next, fields);
        unsafe {
            wsdf::epan_sys::proto_item_set_len(parent, (offset - args.offset) as _);
        }
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let prefix_next = args.prefix.to_owned() + "." + "header";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "header",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let __header = <u8 as wsdf::Dissect<'tvb, ()>>::emit(&args_next);
        <u8 as wsdf::Primitive<'tvb, ()>>::save(&args_next, fields, &mut fields_local);
        let ctx = wsdf::tap::Context {
            field: __header,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let offset = offset + <u8 as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        let prefix_next = args.prefix.to_owned() + "." + "body";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "body",
            offset,
            parent,
            variant: std::option::Option::Some(
                wsdf::tap::handle_get_variant(
                    &wsdf::tap::Context {
                        field: (),
                        fields,
                        fields_local: &fields_local,
                        pinfo: args.pinfo,
                        packet: args.data,
                        offset,
                    },
                    get_variant_header,
                ),
            ),
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let __body = <MyTransportBody as wsdf::Dissect<'tvb, ()>>::emit(&args_next);
        let ctx = wsdf::tap::Context {
            field: __body,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let offset = offset
            + <MyTransportBody as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let _ = ws_indices.ett.get_or_create_ett(args);
        let _ = ws_indices.hf.get_or_create_text_node(args);
        let prefix_next = args.prefix.to_owned() + "." + "header";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Header\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <u8 as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "body";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Body\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <MyTransportBody as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
fn decode_my_transport_message_header(Field(header): Field<u8>) -> String {
    {
        let res = ::alloc::fmt::format(
            format_args!(
                "\nTransportMessage\nbody: Frame(\nFrame\nreliability: Reliable,\nsn: 106702511,\npayload: [\nNetworkMessage\nbody: Push(\nPush\nwire_expr: demo/example/zenoh-rs-put,\next_qos: QoS\npriority: Data,\ncongestion: Drop,\nexpress: false,\n,\next_tstamp: None,\next_nodeid: NodeIdType\nnode_id: 0,\n,\npayload: Put(\nPut\ntimestamp: None,\nencoding: Exact(\nTextPlain,\n),\next_sinfo: None,\next_unknown: [],\npayload: ZBuf\nslices: [[50, 75, 74, 20, 66, 72, 6f, 6d, 20, 52, 75, 73, 74, 21]],\n,\n,\n),\n,\n),\n,\n],\next_qos: QoSType\ninner: 5,\n,\n,\n),\n    ",
            ),
        );
        res
    }
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
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __Frame;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __Frame {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __Fragment;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __Fragment {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __KeepAlive;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __KeepAlive {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __InitSyn;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __InitSyn {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __InitAck;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __InitAck {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __OpenSyn;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __OpenSyn {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __OpenAck;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __OpenAck {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __Close;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __Close {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __OAM;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __OAM {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __Join;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __Join {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
#[wsdf(pre_dissect = [])]
#[wsdf(post_dissect = [])]
struct __Error;
impl<'tvb> wsdf::Dissect<'tvb, ()> for __Error {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset
            + <() as wsdf::Dissect<'tvb, ()>>::add_to_tree(&args_next, fields);
        offset - args.offset
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        let offset = args.offset;
        let parent = args.parent;
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: args.prefix,
            prefix_local: args.prefix_local,
            offset: args.offset,
            parent: args.parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let offset = offset + <() as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: args.name,
            prefix: args.prefix,
            blurb: if !args.blurb.is_null() { args.blurb } else { std::ptr::null() },
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <() as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
impl<'tvb> wsdf::Dissect<'tvb, ()> for MyTransportBody {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        match args.variant {
            Some("Frame") => {
                let prefix_next = args.prefix.to_owned() + "." + "frame";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "frame",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Frame::add_to_tree(&args_next, fields)
            }
            Some("Fragment") => {
                let prefix_next = args.prefix.to_owned() + "." + "fragment";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "fragment",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Fragment::add_to_tree(&args_next, fields)
            }
            Some("KeepAlive") => {
                let prefix_next = args.prefix.to_owned() + "." + "keep_alive";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "keep_alive",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __KeepAlive::add_to_tree(&args_next, fields)
            }
            Some("InitSyn") => {
                let prefix_next = args.prefix.to_owned() + "." + "init_syn";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "init_syn",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __InitSyn::add_to_tree(&args_next, fields)
            }
            Some("InitAck") => {
                let prefix_next = args.prefix.to_owned() + "." + "init_ack";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "init_ack",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __InitAck::add_to_tree(&args_next, fields)
            }
            Some("OpenSyn") => {
                let prefix_next = args.prefix.to_owned() + "." + "open_syn";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "open_syn",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __OpenSyn::add_to_tree(&args_next, fields)
            }
            Some("OpenAck") => {
                let prefix_next = args.prefix.to_owned() + "." + "open_ack";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "open_ack",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __OpenAck::add_to_tree(&args_next, fields)
            }
            Some("Close") => {
                let prefix_next = args.prefix.to_owned() + "." + "close";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "close",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Close::add_to_tree(&args_next, fields)
            }
            Some("OAM") => {
                let prefix_next = args.prefix.to_owned() + "." + "oam";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "oam",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __OAM::add_to_tree(&args_next, fields)
            }
            Some("Join") => {
                let prefix_next = args.prefix.to_owned() + "." + "join";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "join",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Join::add_to_tree(&args_next, fields)
            }
            Some("Error") => {
                let prefix_next = args.prefix.to_owned() + "." + "error";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "error",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Error::add_to_tree(&args_next, fields)
            }
            Some(v) => {
                ::core::panicking::panic_fmt(
                    format_args!("unexpected variant {0} of {1}", v, "MyTransportBody"),
                );
            }
            None => {
                ::core::panicking::panic_fmt(
                    format_args!("unable to determine variant of {0}", "MyTransportBody"),
                );
            }
        }
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        let mut fields_local = wsdf::FieldsStore::default();
        match args.variant {
            Some("Frame") => {
                let prefix_next = args.prefix.to_owned() + "." + "frame";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "frame",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Frame::size(&args_next, fields)
            }
            Some("Fragment") => {
                let prefix_next = args.prefix.to_owned() + "." + "fragment";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "fragment",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Fragment::size(&args_next, fields)
            }
            Some("KeepAlive") => {
                let prefix_next = args.prefix.to_owned() + "." + "keep_alive";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "keep_alive",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __KeepAlive::size(&args_next, fields)
            }
            Some("InitSyn") => {
                let prefix_next = args.prefix.to_owned() + "." + "init_syn";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "init_syn",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __InitSyn::size(&args_next, fields)
            }
            Some("InitAck") => {
                let prefix_next = args.prefix.to_owned() + "." + "init_ack";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "init_ack",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __InitAck::size(&args_next, fields)
            }
            Some("OpenSyn") => {
                let prefix_next = args.prefix.to_owned() + "." + "open_syn";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "open_syn",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __OpenSyn::size(&args_next, fields)
            }
            Some("OpenAck") => {
                let prefix_next = args.prefix.to_owned() + "." + "open_ack";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "open_ack",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __OpenAck::size(&args_next, fields)
            }
            Some("Close") => {
                let prefix_next = args.prefix.to_owned() + "." + "close";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "close",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Close::size(&args_next, fields)
            }
            Some("OAM") => {
                let prefix_next = args.prefix.to_owned() + "." + "oam";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "oam",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __OAM::size(&args_next, fields)
            }
            Some("Join") => {
                let prefix_next = args.prefix.to_owned() + "." + "join";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "join",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Join::size(&args_next, fields)
            }
            Some("Error") => {
                let prefix_next = args.prefix.to_owned() + "." + "error";
                let args_next = wsdf::DissectorArgs {
                    hf_indices: args.hf_indices,
                    etts: args.etts,
                    dtables: args.dtables,
                    tvb: args.tvb,
                    pinfo: args.pinfo,
                    proto_root: args.proto_root,
                    data: args.data,
                    prefix: &prefix_next,
                    prefix_local: "error",
                    offset: args.offset,
                    parent: args.parent,
                    variant: std::option::Option::None,
                    list_len: std::option::Option::None,
                    ws_enc: std::option::Option::None,
                };
                __Error::size(&args_next, fields)
            }
            Some(v) => {
                ::core::panicking::panic_fmt(
                    format_args!("unexpected variant {0} of {1}", v, "MyTransportBody"),
                );
            }
            None => {
                ::core::panicking::panic_fmt(
                    format_args!("unable to determine variant of {0}", "MyTransportBody"),
                );
            }
        }
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let prefix_next = args.prefix.to_owned() + "." + "frame";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Frame\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __Frame::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "fragment";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Fragment\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __Fragment::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "keep_alive";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Keep Alive\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __KeepAlive::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "init_syn";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Init Syn\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __InitSyn::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "init_ack";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Init Ack\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __InitAck::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "open_syn";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Open Syn\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __OpenSyn::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "open_ack";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Open Ack\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __OpenAck::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "close";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Close\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __Close::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "oam";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Oam\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __OAM::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "join";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Join\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __Join::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "error";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Error\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        __Error::register(&args_next, ws_indices);
    }
    fn emit(args: &wsdf::DissectorArgs<'_, 'tvb>) {}
}
