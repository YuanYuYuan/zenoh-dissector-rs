#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{cell::RefCell, borrow::BorrowMut};
use derives::MyProto;
use std::collections::HashMap;
use zenoh_dissector::{
    utils::nul_terminated_str,
    // test_msg::Message,
    header_field::GenerateHFMap,
    wireshark::register_header_field,
    tree::{TreeArgs, AddToTree},
};
use zenoh_protocol::transport::TransportMessage;
use zenoh_codec::{RCodec, Zenoh080};
use zenoh_buffers::reader::HasReader;
use zenoh_buffers::ZSlice;
use zenoh_buffers::reader::Reader;
use anyhow::Result;

#[no_mangle]
#[used]
static plugin_version: [std::ffi::c_char; 6usize] = [48i8, 46i8, 48i8, 46i8, 49i8, 0i8];
#[no_mangle]
#[used]
static plugin_want_major: std::ffi::c_int = 4;
#[no_mangle]
#[used]
static plugin_want_minor: std::ffi::c_int = 0;

#[derive(Default, Debug)]
struct ProtocolData {
    id: i32,
    hf_map: HashMap<String, std::ffi::c_int>,
    ett_map: HashMap<String, std::ffi::c_int>,
}

thread_local! {
    static PROTOCOL_DATA: RefCell<ProtocolData> = ProtocolData::default().into();
}

#[no_mangle]
extern "C" fn plugin_register() {
    static mut PLUG: epan_sys::proto_plugin = epan_sys::proto_plugin {
        register_protoinfo: None,
        register_handoff: None,
    };
    unsafe {
        PLUG.register_protoinfo = Some(register_protoinfo);
        PLUG.register_handoff = Some(register_handoff);
        wsdf::epan_sys::proto_register_plugin(&PLUG);
    }
}

fn register_zenoh_protocol() -> Result<()> {
    let proto_id = unsafe {
        epan_sys::proto_register_protocol(
            nul_terminated_str("NameIsZenoh")?,
            nul_terminated_str("ShortNameIsZenoh")?,
            nul_terminated_str("FilterNameIszenoh")?,
        )
    };

    let hf_map = TransportMessage::generate_hf_map("zenoh");

    PROTOCOL_DATA.with(|data| {
        data.borrow_mut().id = proto_id;

        // Header Field
        for (key, hf) in &hf_map {
            data.borrow_mut().hf_map.insert(
                key.to_string(),
                register_header_field(
                    proto_id,
                    &hf.name,
                    key,
                    hf.kind,
                )?
            );
        }

        let ett_ptr = Box::leak(Box::new(-1)) as *mut _;
        unsafe {
            epan_sys::proto_register_subtree_array([ett_ptr].as_mut_ptr(), 1);
        }
        let ett = unsafe { *ett_ptr };
        debug_assert_ne!(ett, -1);
        anyhow::Ok(())
    })?;
    Ok(())
}

unsafe extern "C" fn register_protoinfo() {
    if let Err(err) = register_zenoh_protocol() {
        eprint!("{err}");
    }
}

unsafe extern "C" fn register_handoff() {
    PROTOCOL_DATA.with(|data| {
        let proto_id = data.borrow().id;
        unsafe {
            let handle = epan_sys::create_dissector_handle(
                Some(dissect_main),
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

use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn dissect_main(
    tvb: *mut epan_sys::tvbuff,
    pinfo: *mut epan_sys::_packet_info,
    tree: *mut epan_sys::_proto_node,
    data: *mut std::ffi::c_void,
) -> std::ffi::c_int {

    epan_sys::col_set_str(
        (*pinfo).cinfo,
        epan_sys::COL_PROTOCOL as std::ffi::c_int,
        nul_terminated_str("Zenoh").unwrap(),
    );
    epan_sys::col_clear(
        (*pinfo).cinfo,
        epan_sys::COL_INFO as std::ffi::c_int,
    );
    let tvb_len = unsafe { epan_sys::tvb_reported_length(tvb) as usize };
    let mut tvb_buf = Vec::<u8>::new();
    // dbg!(&tvb_len);
    tvb_buf.resize(tvb_len, 0);
    unsafe {
        epan_sys::tvb_memcpy(
            tvb,
            tvb_buf.as_mut_ptr() as *mut std::ffi::c_void,
            0,
            tvb_len,
        );
    }

    // let mut reader = tvb_buf[2..].reader();
    // let codec = Zenoh080::new();
    //
    // let msg: TransportMessage = codec
    //     .read(&mut reader)
    //     .expect("Failed to read!!!!!!!!!");
    // // dbg!(&msg);

    let mut zslice = ZSlice::from(tvb_buf);
    let codec = Zenoh080::new();
    let mut reader = zslice.reader();
    while reader.can_read() {
        match <Zenoh080 as RCodec<TransportMessage, _>>::read(codec, &mut reader) {
            Ok(msg) => {
                dbg!(&msg);
                PROTOCOL_DATA.with(|data| {
                    let tree_args = TreeArgs {
                        tree,
                        tvb,
                        hf_map: &data.borrow().hf_map
                    };
                    if let Err(err) = msg.add_to_tree("zenoh", &tree_args) {
                        dbg!(err);
                    }
                });
            },
            Err(err) => {
                dbg!(err);
            }
        }
    }

    // PROTOCOL_DATA.with(|data| {
    //     let tree_args = TreeArgs {
    //         tree,
    //         tvb,
    //         hf_map: &data.borrow().hf_map
    //     };
    //     if let Err(err) = msg.add_to_tree("zenoh", &tree_args) {
    //         dbg!(err);
    //     }
    // });

    32
}
