mod utils;

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use serde_wasm_bindgen::{to_value, from_value};
use js_sys;

#[derive(Serialize, Deserialize)]
struct Person {
    age: u32,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct People {
    people: Vec<Person>,
}

impl People {
    fn double(&self) -> Self {
        People {
            people: self.people.iter().map(|p| Person {
                age: p.age * 2,
                name: p.name.clone() + &p.name,
            }).collect() // ここでコレクション型に変換
        }
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct MyStruct {
    callback: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        MyStruct { callback: None }
    }

    pub fn init(&mut self, callbacks: &JsValue) {
        let callbacks_obj: js_sys::Object = callbacks.clone().into();
        let callback1_val: JsValue = js_sys::Reflect::get(&callbacks_obj, &"callback1".into()).unwrap();
        let callback1: js_sys::Function = callback1_val.into();
        self.callback = Some(callback1);
    }

    pub fn call_callback(&self, arg: JsValue) {
        if let Some(ref callback) = self.callback {
            let this = JsValue::null();
            let _ = callback.call1(&this, &arg);
        }
    }
}

#[wasm_bindgen]
pub fn double_people(js_people: JsValue) -> JsValue {
    if let Ok(people) = from_value::<People>(js_people) {
        let doubled_people = people.double();
        return to_value(&doubled_people).unwrap();
    }

    JsValue::undefined()
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello, {}!", name).as_str());
}

#[wasm_bindgen]
pub fn flatten() -> JsValue {
    return JsValue::from_str("aaaaa");
}

#[wasm_bindgen]
pub fn get_person() -> JsValue {
    let p = Person { age: 20, name: "Taro".into() };
    to_value(&p).unwrap()
}

#[wasm_bindgen]
pub fn get_people() -> JsValue {
    let p = Person { age: 20, name: "Taro".into() };
    let p1 = Person { age: 21, name: "Jiro".into() };
    let p2 = Person { age: 22, name: "Alice".into() };
    let p3 = Person { age: 23, name: "Mercurius".into() };
    let people = People {
        people: vec![p, p1, p2, p3]
    };

    to_value(&people).unwrap()
}

// #[wasm_bindgen]
// pub fn print_people(js_people: JsValue) -> JsValue {
//     if let Ok(people) = from_value::<People>(js_people) {
//         to_value(&people).unwrap()
//     } else {
//         println!("Failed to deserialize People");
//         None
//     }
// }
//
// #[wasm_bindgen]
// extern "C" {
//     type JsFunction;
//
//     #[wasm_bindgen(method, js_name = call)]
//     fn call(this: &JsFunction, people: JsValue);
// }
//
//
// use wasm_bindgen::JsCast;
//
// #[wasm_bindgen]
// pub fn call_js_callback(callback: js_sys::Function, people: JsValue) {
//     let this = JsValue::null();
//     let _ = callback.call1(&this, &people);
// }