#![feature(prelude_import)]
#![allow(dead_code)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::format;
use wsdf::{protocol, version, Dissect, Proto, tap::{FieldsLocal, Fields, Field}};
use wsdf::tap::{Offset, Packet};
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
use zenoh_protocol::common::imsg;
use zenoh_protocol::transport::{FrameHeader, TransportMessage};
use zenoh_protocol::{core::Reliability, transport::Frame};
#[wsdf(decode_from = [("tcp.port", 7447)])]
struct MyZenoh {
    #[wsdf(enc = "ENC_LITTLE_ENDIAN", rename = "WTF", decode_with = "decode_with")]
    len2: u16,
    #[wsdf(bytes, consume_with = "consume_bytes")]
    some_data: Vec<u8>,
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
        let prefix_next = args.prefix.to_owned() + "." + "len2";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "len2",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let __len2 = <u16 as wsdf::Dissect<'tvb, ()>>::emit(&args_next);
        let ctx = wsdf::tap::Context {
            field: __len2,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let s = wsdf::tap::handle_decode_with(&ctx, decode_with);
        let n = <u16 as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        <u16 as wsdf::Primitive<'tvb, ()>>::add_to_tree_format_value(&args_next, &s, n);
        let offset = offset + n;
        let prefix_next = args.prefix.to_owned() + "." + "some_data";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "some_data",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let __some_data = <Vec<u8> as wsdf::Dissect<'tvb, [u8]>>::emit(&args_next);
        let ctx = wsdf::tap::Context {
            field: __some_data,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let (n, s) = wsdf::tap::handle_consume_with(&ctx, consume_bytes);
        <Vec<
            u8,
        > as wsdf::Primitive<'tvb, [u8]>>::add_to_tree_format_value(&args_next, &s, n);
        let offset = offset + n;
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
        let prefix_next = args.prefix.to_owned() + "." + "len2";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "len2",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::Some(wsdf::epan_sys::ENC_LITTLE_ENDIAN),
        };
        let __len2 = <u16 as wsdf::Dissect<'tvb, ()>>::emit(&args_next);
        let ctx = wsdf::tap::Context {
            field: __len2,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let offset = offset + <u16 as wsdf::Dissect<'tvb, ()>>::size(&args_next, fields);
        let prefix_next = args.prefix.to_owned() + "." + "some_data";
        let args_next = wsdf::DissectorArgs {
            hf_indices: args.hf_indices,
            etts: args.etts,
            dtables: args.dtables,
            tvb: args.tvb,
            pinfo: args.pinfo,
            proto_root: args.proto_root,
            data: args.data,
            prefix: &prefix_next,
            prefix_local: "some_data",
            offset,
            parent,
            variant: std::option::Option::None,
            list_len: std::option::Option::None,
            ws_enc: std::option::Option::None,
        };
        let __some_data = <Vec<u8> as wsdf::Dissect<'tvb, [u8]>>::emit(&args_next);
        let ctx = wsdf::tap::Context {
            field: __some_data,
            fields,
            fields_local: &fields_local,
            pinfo: args.pinfo,
            packet: args.data,
            offset,
        };
        let (n, _) = wsdf::tap::handle_consume_with(&ctx, consume_bytes);
        let offset = offset + n;
        offset - args.offset
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        let _ = ws_indices.ett.get_or_create_ett(args);
        let _ = ws_indices.hf.get_or_create_text_node(args);
        let prefix_next = args.prefix.to_owned() + "." + "len2";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "WTF\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <u16 as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
        let prefix_next = args.prefix.to_owned() + "." + "some_data";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "Some Data\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <Vec<u8> as wsdf::Dissect<'tvb, [u8]>>::register(&args_next, ws_indices);
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
fn decode_with(Field(len2): Field<u16>) -> &'static str {
    "Hey"
}
fn consume_bytes(Offset(offset): Offset, Packet(pkt): Packet) -> (usize, String) {
    ::core::panicking::panic("not implemented")
}
struct MyTransportMessage {}
impl<'tvb> wsdf::Dissect<'tvb, ()> for MyTransportMessage {
    type Emit = ();
    fn add_to_tree(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        2
    }
    fn size(
        args: &wsdf::DissectorArgs<'_, 'tvb>,
        fields: &mut wsdf::FieldsStore<'tvb>,
    ) -> usize {
        2
    }
    fn register(args: &wsdf::RegisterArgs, ws_indices: &mut wsdf::WsIndices) {
        match "============ register ==============" {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}] {2} = {3:#?}\n",
                            "src/lib.rs",
                            65u32,
                            "\"============ register ==============\"",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
        match args {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}] {2} = {3:#?}\n",
                            "src/lib.rs",
                            66u32,
                            "args",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
        match &ws_indices {
            tmp => {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "[{0}:{1}] {2} = {3:#?}\n",
                            "src/lib.rs",
                            67u32,
                            "&ws_indices",
                            &tmp,
                        ),
                    );
                };
                tmp
            }
        };
        let prefix_next = args.prefix.to_owned() + "." + "wtfffffffffffff";
        let args_next = wsdf::RegisterArgs {
            proto_id: args.proto_id,
            name: "WTFF\u{0}".as_ptr() as *const std::ffi::c_char,
            prefix: &prefix_next,
            blurb: std::ptr::null(),
            ws_type: std::option::Option::None,
            ws_display: std::option::Option::None,
        };
        <u16 as wsdf::Dissect<'tvb, ()>>::register(&args_next, ws_indices);
    }
    fn emit(_args: &wsdf::DissectorArgs) {}
}
