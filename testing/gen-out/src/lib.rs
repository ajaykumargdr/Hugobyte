#![allow(unused_imports)]

mod common;
mod traits;
mod types;
use derive_enum_from_into::{EnumFrom, EnumTryInto};
use dyn_clone::{clone_trait_object, DynClone};
use openwhisk_macro::OpenWhisk;
use openwhisk_rust::*;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use workflow_macro::Flow;

use common::*;
use paste::*;
use std::convert::TryInto;
use traits::*;
use types::*;
extern crate alloc;
use codec::{Decode, Encode};
use core::alloc::Layout;
use sp_core::H256;
use sp_runtime::AccountId32;
use substrate_macro::Polkadot;
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
pub struct StakingLedger {
    pub stash: AccountId32,
    #[codec(compact)]
    pub total: u128,
    #[codec(compact)]
    pub active: u128,
    pub unlocking: Vec<u32>,
    pub claimed_rewards: Vec<u32>,
}

#[no_mangle]
pub fn _start(ptr: *mut u8, length: i32) {
    let result: Value;
    unsafe {
        let mut vect = Vec::new();
        for i in 1..=length {
            if let Some(val_back) = ptr.as_ref() {
                vect.push(val_back.clone());
            }
            *ptr = *ptr.add(i as usize);
        }
        result = serde_json::from_slice(&vect).unwrap();
    }

    let res = main(result);
    let output = Output {
        result: serde_json::to_value(res).unwrap(),
    };
    let serialized = serde_json::to_vec(&output).unwrap();
    let size = serialized.len() as i32;
    let ptr = serialized.as_ptr();
    unsafe {
        set_output(ptr as i32, size);
    }
}

#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 2;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

    let stakingpayout = Stakingpayout::new(
        input.url,
        input.owner_key,
        input.address,
        input.era,
        String::from("stakingpayout"),
    );

    let pushnotification = PushNotification::new(
        input.token,
        input.message,
        String::from("push_notification"),
    );

    let stakingpayout_index = workflow.add_node(Box::new(stakingpayout));
    let pushnotification_index = workflow.add_node(Box::new(pushnotification));

    workflow.add_edges(&[(stakingpayout_index, pushnotification_index)]);

    let result = workflow.init()?.term(Some(pushnotification_index))?;

    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}

#[no_mangle]
pub unsafe extern "C" fn memory_alloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn free_memory(ptr: *mut u8, size: u32, alignment: u32) {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}

#[link(wasm_import_module = "host")]
extern "C" {
    fn set_output(ptr: i32, size: i32);
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}
