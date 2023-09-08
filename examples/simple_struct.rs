#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{cell::RefCell, borrow::BorrowMut};

use derives::MyProto;
use zenoh_dissector::MyProtoTrait;
use std::collections::HashMap;

#[no_mangle]
#[used]
static plugin_version: [std::ffi::c_char; 6usize] = [48i8, 46i8, 48i8, 46i8, 49i8, 0i8];
#[no_mangle]
#[used]
static plugin_want_major: std::ffi::c_int = 4;
#[no_mangle]
#[used]
static plugin_want_minor: std::ffi::c_int = 0;

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

unsafe extern "C" fn register_protoinfo() {
    let proto_id = unsafe {
        epan_sys::proto_register_protocol(
            nul_terminated_str("NameIsZenoh"),
            nul_terminated_str("ShortNameIsZenoh"),
            nul_terminated_str("FilterNameIszenoh"),
        )
    };

    PROTOCOL_DATA.with(|data| {
        data.borrow_mut().id = proto_id;
        data.borrow_mut().hf_map.insert(
            "my_zenoh.msg".to_string(),
            register_hf_index(
                "MyMessage",
                "my_zenoh.msg",
                proto_id,
                epan_sys::field_display_e_BASE_NONE as _,
                epan_sys::ftenum_FT_NONE,
            )
        );

        data.borrow_mut().hf_map.insert(
            "my_zenoh.msg.body".to_string(),
            register_hf_index(
                "Body",
                "my_zenoh.msg.body",
                proto_id,
                epan_sys::field_display_e_BASE_NONE as _,
                epan_sys::ftenum_FT_NONE,
            )
        );

        data.borrow_mut().hf_map.insert(
            "my_zenoh.msg.body.init_syn.version".to_string(),
            register_hf_index(
                "Version",
                "my_zenoh.msg.body.init_syn.version",
                proto_id,
                epan_sys::field_display_e_BASE_DEC as _,
                epan_sys::ftenum_FT_UINT8,
            )
        );

        data.borrow_mut().hf_map.insert(
            "my_zenoh.msg.body.init_syn.whatami".to_string(),
            register_hf_index(
                "WhatAmI",
                "my_zenoh.msg.body.init_syn.whatami",
                proto_id,
                epan_sys::field_display_e_BASE_DEC as _,
                epan_sys::ftenum_FT_UINT8,
            )
        );

        data.borrow_mut().hf_map.insert(
            "my_zenoh.msg.body.frame.payload".to_string(),
            register_hf_index(
                "Payload",
                "my_zenoh.msg.body.frame.payload",
                proto_id,
                epan_sys::field_display_e_SEP_SPACE as _,
                epan_sys::ftenum_FT_BYTES,
            )
        );

        let ett_ptr = Box::leak(Box::new(-1)) as *mut _;
        unsafe {
            epan_sys::proto_register_subtree_array([ett_ptr].as_mut_ptr(), 1);
        }
        let ett = unsafe { *ett_ptr };
        debug_assert_ne!(ett, -1);
        data.borrow_mut().ett_map.insert("my_zenoh.msg.body".to_string(), ett);
    });
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

    // PROTO_ID.with(|proto_id| {
    //     unsafe {
    //         let handle = epan_sys::create_dissector_handle(
    //             Some(dissect_main),
    //             *proto_id.borrow(),
    //         );
    //         wsdf::epan_sys::dissector_add_uint(
    //             "tcp.port\u{0}".as_ptr() as *const std::ffi::c_char,
    //             7447u32 as std::ffi::c_uint,
    //             handle,
    //         );
    //     }
    // });
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
        nul_terminated_str("Zenoh"),
    );
    epan_sys::col_clear(
        (*pinfo).cinfo,
        epan_sys::COL_INFO as std::ffi::c_int,
    );
    let tvb_len = unsafe { epan_sys::tvb_reported_length(tvb) as usize };
    let mut tvb_buf = Vec::<u8>::new();
    dbg!(&tvb_len);
    tvb_buf.resize(tvb_len, 0);
    unsafe {
        epan_sys::tvb_memcpy(
            tvb,
            tvb_buf.as_mut_ptr() as *mut std::ffi::c_void,
            0,
            tvb_len,
        );
    }
    // dbg!(&tvb_buf);

    PROTOCOL_DATA.with(|data| {
        // let mut data = data.borrow_mut();
        let prefix = "my_zenoh.msg.body";


        let ett = *data.borrow().ett_map.get(prefix).unwrap();
        let hf_map = &data.borrow().hf_map;

        let is_even = CALL_COUNT.fetch_add(1, Ordering::SeqCst) % 2 == 0;

        let subtree = unsafe {
            let ti = epan_sys::proto_tree_add_none_format(
                tree,
                *hf_map.get(prefix).unwrap(),
                tvb,
                0 as _,
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
            let hf_index = *hf_map
                .get(prefix)
                .expect(&format!("{:?}", &data));

            unsafe {
                epan_sys::proto_tree_add_uint_format_value(
                    subtree,
                    hf_index,
                    tvb,
                    2 as _,
                    3,
                    45678,
                    nul_terminated_str("This is version!"),
                );
            }

            let prefix = "my_zenoh.msg.body.init_syn.whatami";
            let hf_index = *hf_map
                .get(prefix)
                .expect(&format!("{:?}", &hf_map));
            unsafe {
                epan_sys::proto_tree_add_uint_format_value(
                    subtree,
                    hf_index,
                    tvb,
                    (2 + 3) as _,
                    3,
                    45678,
                    nul_terminated_str("This is WhatAmI"),
                );
            }
        } else {
            let prefix = "my_zenoh.msg.body.frame.payload";
            let hf_index = *hf_map
                .get(prefix)
                .expect(&format!("{:?}", &hf_map));

            let payload: Vec<_> = (0usize..1000).into_iter().map(|v| (v % 16) as u8).collect();
            unsafe {
                epan_sys::proto_tree_add_bytes_with_length(
                    subtree,
                    hf_index,
                    tvb,
                    0 as _,
                    16,
                    payload.as_ptr(),
                    payload.len() as _,
                );
            }


        }

    });

    32
}

#[derive(Default, Debug)]
struct ProtocolData {
    id: i32,
    hf_map: HashMap<String, std::ffi::c_int>,
    ett_map: HashMap<String, std::ffi::c_int>,
}

thread_local! {
    static PROTOCOL_DATA: RefCell<ProtocolData> = ProtocolData::default().into();
}

fn nul_terminated_str(s: &str) -> *const std::ffi::c_char {
    Box::leak(std::ffi::CString::new(s).unwrap().into_boxed_c_str()).as_ptr()
}

pub fn register_hf_index(
    name: &str,
    prefix: &str,
    proto_id: i32,
    display: std::ffi::c_int,
    type_: std::ffi::c_uint,
) -> std::ffi::c_int {
    let hf_index_ptr = Box::leak(Box::new(-1)) as *mut _;
    let abbrev = nul_terminated_str(prefix);
        // Box::leak(std::ffi::CString::new(prefix).unwrap().into_boxed_c_str()).as_ptr() as *const std::ffi::c_char;

    let hf_register_info = epan_sys::hf_register_info {
        p_id: hf_index_ptr,
        hfinfo: epan_sys::header_field_info {
            name: nul_terminated_str(name),
            abbrev,
            type_,
            display,
            strings: std::ptr::null(),
            bitmask: 0,
            blurb: std::ptr::null(),
            id: -1,
            parent: 0,
            ref_type: epan_sys::hf_ref_type_HF_REF_TYPE_NONE,
            same_name_prev_id: -1,
            same_name_next: std::ptr::null_mut(),
        },
    };
    let hfs = Box::leak(Box::new([hf_register_info])) as *mut _;

    unsafe {
        epan_sys::proto_register_field_array(proto_id, hfs, 1);
    }
    debug_assert_ne!(unsafe { *hf_index_ptr }, -1);
    unsafe { *hf_index_ptr }
}


#[derive(MyProto)]
struct ThisShouldFail {}
