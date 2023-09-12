use crate::{
    test_msg::{Body, InitSyn, Message},
    utils::nul_terminated_str,
};
use anyhow::{bail, Result};
use std::collections::HashMap;

type HFPointerMap = HashMap<String, std::ffi::c_int>;

pub struct TreeArgs<'a> {
    pub tree: *mut epan_sys::proto_tree,
    pub tvb: *mut epan_sys::tvbuff,
    pub hf_map: &'a HFPointerMap,
}

impl TreeArgs<'_> {
    pub fn get_hf(&self, key: &str) -> Result<std::ffi::c_int> {
        if let Some(hf) = self.hf_map.get(key) {
            Ok(*hf)
        } else {
            bail!("{key} not found in {:?}", &self.hf_map)
        }
    }
}

pub trait AddToTree {
    fn add_to_tree(&self, prefix: &str, args: &TreeArgs) -> Result<()>;
}
