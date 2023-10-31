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
use core::alloc::Layout;

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
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

    let cartype = Cartype::new(input.car_type, String::from("cartype"));

    let modelavail = Modelavail::new(input.company_name, String::from("modelavail"));

    let modelsprice = Modelsprice::new(String::from("modelsprice"));

    let purchase = Purchase::new(input.model_name, input.price, String::from("purchase"));

    let cartype_index = workflow.add_node(Box::new(cartype));
    let modelavail_index = workflow.add_node(Box::new(modelavail));
    let modelsprice_index = workflow.add_node(Box::new(modelsprice));
    let purchase_index = workflow.add_node(Box::new(purchase));

    workflow.add_edges(&[
        (cartype_index, modelavail_index),
        (modelavail_index, modelsprice_index),
        (modelsprice_index, purchase_index),
    ]);

    let result = workflow
        .init()?
        .pipe(modelavail_index)?
        .pipe(modelsprice_index)?
        .term(Some(purchase_index))?;

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
