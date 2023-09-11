use crate::{
    test_msg::{Body, InitSyn, Message},
    utils::nul_terminated_str,
};
use std::collections::HashMap;

type HFPointerMap = HashMap<String, std::ffi::c_int>;

pub struct TreeArgs<'a> {
    pub tree: *mut epan_sys::proto_tree,
    pub tvb: *mut epan_sys::tvbuff,
    pub hf_map: &'a HFPointerMap,
}

pub trait AddToTree {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs);
}

impl AddToTree for Message {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) {
        self.body.add_to_tree(&format!("{prefix}.body"), args);
    }
}

impl AddToTree for Body {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) {
        match self {
            Self::InitSyn(body) => {
                body.add_to_tree(&format!("{prefix}.init_syn"), args);
            }
            Self::InitAck(body) => {
                todo!()
            }
        }
    }
}

impl AddToTree for InitSyn {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) {
        let hf_index = *args
            .hf_map
            .get(&format!("{prefix}.version"))
            .expect(&format!("{prefix}.version not found in hf_map"));
        unsafe {
            epan_sys::proto_tree_add_uint(
                args.tree,
                hf_index,
                args.tvb,
                2 as _,
                3,
                self.version.into(),
            );
        }

        let hf_index = *args
            .hf_map
            .get(&format!("{prefix}.whatami"))
            .expect(&format!("{prefix}.whatami not found in hf_map"));
        unsafe {
            epan_sys::proto_tree_add_string(
                args.tree,
                hf_index,
                args.tvb,
                2 as _,
                3,
                nul_terminated_str(self.whatami.to_str()).unwrap(),
            );
        }
    }
}
