// 在浏览器里展示trace

// boa/boa_engine/Cargo.toml
// wasm-bindgen = "=0.2.79"
// wasm-bindgen-test = "0.3.29"

// vm/mod.rs
// use wasm_bindgen_test::__rt::js_console_log;


// js_console_log(&self.vm.frame().code.to_interned_string(self.interner()));


// js_console_log(&"stack---");
// if self.vm.stack.is_empty() {
// js_console_log(&"    <empty>");
// } else {
// for (i, value) in self.vm.stack.iter().enumerate() {
// js_console_log(&i.to_string());
// let msgg = (if value.is_callable() {
// "[function]".to_string()
// } else if value.is_object() {
// "[object]".to_string()
// } else {
// value.display().to_string()
// });
// js_console_log(&msgg);
// }
// }
//
// js_console_log(&"\n");