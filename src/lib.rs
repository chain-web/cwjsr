#![cfg_attr(not(test), forbid(clippy::unwrap_used))]
#![warn(
    clippy::perf,
    clippy::single_match_else,
    clippy::dbg_macro,
    clippy::doc_markdown,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::doc_markdown,
    clippy::semicolon_if_nothing_returned,
    clippy::pedantic
)]
#![deny(
    clippy::all,
    clippy::cast_lossless,
    clippy::redundant_closure_for_method_calls,
    clippy::use_self,
    clippy::unnested_or_patterns,
    clippy::trivially_copy_pass_by_ref,
    clippy::needless_pass_by_value,
    clippy::match_wildcard_for_single_variants,
    clippy::map_unwrap_or,
    unused_qualifications,
    unused_import_braces,
    unused_lifetimes,
    unreachable_pub,
    trivial_numeric_casts,
    // rustdoc,
    missing_debug_implementations,
    missing_copy_implementations,
    deprecated_in_future,
    meta_variable_misuse,
    non_ascii_idents,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_ptr_alignment,
    clippy::missing_panics_doc,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::missing_inline_in_public_items,
    clippy::cognitive_complexity,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::as_conversions,
    clippy::let_unit_value,
    rustdoc::missing_doc_code_examples
)]

use boa_engine::Context;
use wasm_bindgen::prelude::*;
// use std::collections::HashMap;
// use wasm_bindgen_test::__rt::js_console_log;
mod sk;

use sk::skglobal::SkGlobal;


// async fn get_account (_arg1: &Bjsv, arg2: &[Bjsv], _arg3 :&mut Context) -> Result<Bjsv, Bjsv> {
//     let ag2 = arg2[0].display().to_string();
//     js_console_log("&ag2");
//     js_console_log(&ag2);
//     let resu = __sk__ipld__getAccount(&ag2).await
//         .map_err(|e| {
//             js_console_log("&e");
//             let s = e.as_string().unwrap();
//             js_console_log(&s);
//             s
//         })
//         .map(|r| {
//             js_console_log("&r");
//             let s = r.as_string().unwrap();
//             js_console_log(&s);
//             s
//         });
//     js_console_log(&ag2);
//     Ok(Bjsv::new(resu.unwrap()))
// }

// fn get_accounts (_arg1: &Bjsv, arg2: &[Bjsv], _arg3 :&mut Context) -> Result<Bjsv, Bjsv> {
//     let ag2 = arg2[0].display().to_string();
//     js_console_log("&ag2");
//     js_console_log(&ag2);
//     let val = JsValue::from_str(&ag2);
//     let resu = __sk__ipld__getAccounts(&val).as_string();

//     // use std::{thread, time};

//     // let ten_millis = time::Duration::from_millis(10);
//     // let now = time::Instant::now();

//     // thread::sleep(ten_millis);
//     match resu
//     {
//         Some(r) => {
//             js_console_log("&r");
//             js_console_log(&r);
//             Ok(Bjsv::new(r))
//         },
//         None => {
//             js_console_log("&r");
//             js_console_log("None");
//             Ok(Bjsv::new("None"))
//         }
//     }
// }

// #[wasm_bindgen]
// extern "C" {
// //    async fn __sk__ipld__getAccount(s: &str) -> Result<JsValue, JsValue>;
//    fn __sk__ipld__getAccounts(s: &JsValue) -> JsValue;
// }


#[wasm_bindgen]
pub fn evaluate(src: &str, cu_limit: u64, param: &JsValue) -> Result<String, JsValue> {
    // 接收kv参数
    // let ep: HashMap<String, Vec<String>> = serde_wasm_bindgen::from_value(param)?;
    // #[allow(clippy::single_match_else)]
    // let mothed = match ep.get("mothed") {
    //     Some(val) => &val[0],
    //     None => {
    //         js_console_log("no mothed");
    //         return Err(JsValue::from_str("no mothed param"))
    //     }
    // };
    // js_console_log(mothed);
    let mut context = Context::new_sk_context(cu_limit);
    // let js_function = FunctionBuilder::closure(
    //     &mut context,  get_accounts
    // )
    // .name("__sk__ipld__getAccounts")
    // .build();

    // // bind the function as a global property in Javascript.
    // context.register_global_property(
    //     // set the key to access the function the same as its name for
    //     "__sk__ipld__getAccounts",
    //     // pass `js_function` as a property value.
    //     js_function,
    //     // assign to the "__sk__ipld__getAccount" property the desired attributes.
    //     Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE,
    // );

    let _res = context.register_global_class::<SkGlobal>();

    context.eval(src)
        .map_err(|e| JsValue::from(format!("Uncaught {}", e.display())))
        .map(|v| v.display().to_string())
}
