mod utils;

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
struct Person {
    age: u32,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct People {
    people: Vec<Person>,
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

