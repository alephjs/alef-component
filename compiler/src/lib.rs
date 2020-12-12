// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

mod ast_walker;
mod error;
mod module;
mod resolve;

use module::{AlefComponentModule, EmitOptions};
use resolve::{CSSTemplate, DependencyDescriptor, Resolver};
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOutput {
    pub code: String,
    pub deps: Vec<DependencyDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<CSSTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
}

#[wasm_bindgen(js_name = "transformSync")]
pub fn transform_sync(specifier: &str, source: &str, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let opts: EmitOptions = opts
        .into_serde()
        .map_err(|err| format!("failed to parse options: {}", err))
        .unwrap();
    let resolver = Rc::new(RefCell::new(Resolver::new()));
    let module = AlefComponentModule::parse(specifier, source).expect("could not parse module");
    let (code, map) = module
        .transpile(resolver.clone(), &opts)
        .expect("could not transpile module");
    let r = resolver.borrow_mut();
    Ok(JsValue::from_serde(&TransformOutput {
        code,
        map,
        deps: r.dep_graph.clone(),
        css: r.css.clone(),
    })
    .unwrap())
}
