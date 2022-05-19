// 向boa runtime注入sk function
use boa_engine::{
  builtins::JsArgs,
  object::{FunctionBuilder, JsObject, ObjectInitializer},
  property::{Attribute, PropertyDescriptor},
  Context, JsResult, JsString, JsValue,
};
use boa_gc::{Finalize, Trace};
use cid::multihash::{Code, MultihashDigest};
// https://github.com/multiformats/rust-cid
use cid::Cid;
use tap::{Conv, Pipe};
use wasm_bindgen_test::__rt::js_console_log;
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_name = jsOftenUsesCamelCase)]
//     fn js_often_uses_camel_case() -> u32;
// }

const RAW: u64 = 0x55;
const JSON: u64 = 0x0200;

// 生成cid hash
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn gen_hash_raw(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
  let text = args
    .get_or_undefined(0)
    .to_string(context)
    .expect("gen_hash: get arg error");
  let h = Code::Sha2_256.digest(text.as_bytes());

  let cid = Cid::new_v1(RAW, h);
  let cid_string = cid.to_string();
  Ok(JsValue::from(cid_string))
}

// 生成dag cid hash
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn gen_hash_dag(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
  let text = args
    .get_or_undefined(0)
    .to_string(context)
    .expect("gen_hash: get arg error");
  let h = Code::Sha2_256.digest(text.as_bytes());

  let cid = Cid::new_v1(JSON, h);
  let cid_string = cid.to_string();
  Ok(JsValue::from(cid_string))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn log(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
  let text = args
    .get_or_undefined(0)
    .to_string(context)
    .expect("log: get arg error");
  js_console_log(&text);
  Ok(JsValue::new(""))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn init_sk(context: &mut Context) -> Result<(), JsValue> {
  let attribute = Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT;
  let obj = ObjectInitializer::new(context)
    .property("name", "cwjsr_sk", attribute)
    .function(gen_hash_raw, "genRawHash", 1)
    .function(gen_hash_dag, "genDagHash", 1)
    .function(log, "log", 1)
    .build()
    .conv::<JsValue>()
    .pipe(Some)
    .expect("init_sk gen sk obj error");

  let init_function = FunctionBuilder::closure_with_captures(
    context,
    |_, _, captures, _context| Ok(captures.clone()),
    obj,
  )
  .name("__init__sk__")
  .build();

  context.register_global_property("__init__sk__", init_function, attribute);
  Ok(())
}
