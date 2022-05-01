// NOTE: this example requires the `console` feature to run correctly.
use boa_engine::{
    class::{Class, ClassBuilder},
    property::Attribute,
    Context, JsResult, JsValue as BoaJsValue,
};

use boa_gc::{Finalize, Trace};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::__rt::js_console_log;

#[wasm_bindgen]
extern "C" {
//    async fn __sk__ipld__getAccount(s: &str) -> Result<JsValue, JsValue>;
//    fn __sk__ipld__getAccounts(s: &JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = __sk__)]
    fn getAccount(s: &str) -> JsValue;
}

// We derive `Debug`, `Trace` and `Finalize`, it automatically implements `NativeObject`
// so we can pass it as an object in Javascript.
//
// The fields of the struct are not accessible by Javascript unless we create accessors for them.
#[derive(Debug, Trace, Finalize)]
pub(crate) struct SkGlobal {
    /// The name of the person.
    name: String,
    /// The age of the preson.
    age: u32,
}

// Here we implement a static method for Person that matches the `NativeFunction` signature.
//
// NOTE: The function does not have to be implemented inside Person, it can be a free function,
// or any function that matches the required signature.
impl SkGlobal {
    /// Says hello if `this` is a `Person`
    fn say_hello(this: &BoaJsValue, _: &[BoaJsValue], context: &mut Context) -> JsResult<BoaJsValue> {
        // We check if this is an object.
        if let Some(object) = this.as_object() {
            // If it is we downcast the type to type `Person`.
            if let Some(sk_global) = object.downcast_ref::<Self>() {
                log(&format!("Hello my name is {}, I'm {} years old", sk_global.name, sk_global.age));
                return Ok(BoaJsValue::from(format!("Hello my name is {}, I'm {} years old", sk_global.name, sk_global.age)));
            }
        }
        // If `this` was not an object or the type of `this` was not a native object `Person`,
        // we throw a `TypeError`.
        context.throw_type_error("'this' is not a skGlobal object")
    }
}

impl Class for SkGlobal {
    // We set the binding name of this function to `"Person"`.
    // It does not have to be `"Person"`, it can be any string.
    const NAME: &'static str = "SkGlobal";
    // We set the length to `2` since we accept 2 arguments in the constructor.
    //
    // This is the same as `Object.length`.
    // NOTE: The default value of `LENGTH` is `0`.
    const LENGTH: usize = 2;

    // This is what is called when we construct a `Person` with the expression `new Person()`.
    fn constructor(_this: &BoaJsValue, args: &[BoaJsValue], context: &mut Context) -> JsResult<Self> {
        // We get the first argument. If it is unavailable we default to `undefined`,
        // and then we call `to_string()`.
        //
        // This is equivalent to `String(arg)`.
        let name = args
            .get(0)
            .cloned()
            .unwrap_or_default()
            .to_string(context)?;
        // We get the second argument. If it is unavailable we default to `undefined`,
        // and then we call `to_u32`.
        //
        // This is equivalent to `arg | 0`.
        let age = args.get(1).cloned().unwrap_or_default().to_u32(context)?;

        // We construct a new native struct `Person`
        let person = Self {
            name: name.to_string(),
            age,
        };

        Ok(person) // and we return it.
    }

    /// Here is where the class is initialized.
    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        // We add a inheritable method `sayHello` with `0` arguments of length.
        //
        // This function is added to the `Person` prototype.
        class.method("sayHello", 0, Self::say_hello);
        // We add a static method `is` using a closure, but it must be convertible
        // to a NativeFunction.
        // This means it must not contain state, or the code won't compile.
        //
        // This function is added to the `Person` class.
        class.static_method("is", 1, |_this, args, _ctx| {
            if let Some(arg) = args.get(0) {
                if let Some(object) = arg.as_object() {
                    // We check if the type of `args[0]` is `Person`
                    if object.is::<Self>() {
                        return Ok(true.into()); // and return `true` if it is.
                    }
                }
            }
            Ok(false.into()) // Otherwise we return `false`.
        });

        // We add an `"inheritedProperty"` property to the prototype of `Person` with
        // a value of `10` and default attribute flags `READONLY`, `NON_ENUMERABLE` and `PERMANENT`.
        class.property("inheritedProperty", 10, Attribute::default());

        // Finally, we add a `"staticProperty"` property to `Person` with a value
        // of `"Im a static property"` and attribute flags `WRITABLE`, `ENUMERABLE` and `PERMANENT`.
        class.static_property(
            "skGlobalVersion",
            "0.0.4",
            Attribute::WRITABLE | Attribute::ENUMERABLE | Attribute::PERMANENT,
        );

        Ok(())
    }
}
