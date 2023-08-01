mod utils;

use std::string::ToString;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue};
use serde_wasm_bindgen::{to_value, from_value};
use js_sys;

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
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

#[wasm_bindgen]
pub struct Point2D {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}

use lazy_static::lazy_static;
lazy_static! {
    // 10000個のPersonを保持する静的配列
    static ref PEOPLE: [Person; 8] = [
        Person {name: String::from("taro"), age: 20},
        Person {name: "jiro".to_string(), age: 21},
        Person {name: "saburo".to_string(), age: 22},
        Person {name: "hanako".to_string(), age: 23},
        Person {name: "jane".to_string(), age: 24},
        Person {name: "tom".to_string(), age: 25},
        Person {name: "jack".to_string(), age: 26},
        Person {name: "milro".to_string(), age: 27},
];
}

// キャッシュの定義
static mut CACHE: Option<(String, Vec<Person>)> = None;

#[wasm_bindgen]
pub fn search_people(name_part: &str) -> JsValue {
    // キャッシュのチェック
    unsafe {
        if let Some((ref query, ref result)) = CACHE {
            if query == name_part {
                // return result.clone();
                return to_value(&result.clone()).unwrap();
            }
        }
    }

    // 絞り込み条件に一致する人々を集める
    let result: Vec<Person> = PEOPLE
        .iter()
        .filter(|p| p.name.contains(name_part))
        .cloned()
        .collect();

    // 結果をキャッシュに格納
    unsafe {
        CACHE = Some((name_part.to_string(), result.clone()));
    }

    to_value(&result).unwrap()
}


impl Person {
    pub fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

#[wasm_bindgen]
pub struct PeopleFinder {
    part_name: Option<String>,
    age_lte: Option<u32>,
    age_gte: Option<u32>,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[wasm_bindgen]
impl PeopleFinder {
    pub fn new() -> PeopleFinder {
        PeopleFinder {
            part_name: None,
            age_lte: None,
            age_gte: None,
        }
    }

    pub fn set_name_part(&mut self, name_part: String) {
        self.part_name = Some(name_part);
    }

    pub fn set_age_lte(&mut self, age: u32) {
        self.age_lte = Some(age);
    }

    pub fn set_age_gte(&mut self, age: u32) {
        self.age_gte = Some(age);
    }

    pub fn clear_name_part(&mut self) {
        self.part_name = None;
    }

    pub fn clear_age_lte(&mut self) {
        self.age_lte = None;
    }

    pub fn clear_age_gte(&mut self) {
        self.age_gte = None;
    }

    pub fn apply(&self) -> JsValue {
        // let people = vec![ // ここに人々のデータを記述
        //                    Person::new(String::from("taro"), 20),
        //                    Person { name: "jiro".to_string(), age: 21 },
        //                    Person { name: "saburo".to_string(), age: 22 },
        //                    Person { name: "hanako".to_string(), age: 23 },
        //                    Person { name: "jane".to_string(), age: 24 },
        //                    Person { name: "tom".to_string(), age: 25 },
        //                    Person { name: "jack".to_string(), age: 26 },
        //                    Person { name: "milro".to_string(), age: 27 },
        // ];
        let people = PEOPLE_ALL.iter();

        let result: Vec<&Person> = people
            // .iter()
            .filter(|p| {
                self.part_name.as_ref().map_or(true, |name_part| p.name.contains(name_part))
                    && self.age_lte.map_or(true, |age| p.age <= age)
                    && self.age_gte.map_or(true, |age| p.age >= age)
            })
            .collect();

        // 結果をJsValueに変換する
        to_value(&result).unwrap()
    }
}