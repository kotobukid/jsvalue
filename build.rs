extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();
    eprintln!("build.rs is running {}", out_dir);
    println!("build.rs is running, {}", out_dir);
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut f = File::create(&dest_path).unwrap();

    // JSONファイルのパス
    let json_file_path = "src/data.json";

    // JSONファイルの読み取り
    let json_file = File::open(json_file_path).unwrap();
    let people: Vec<Person> = serde_json::from_reader(json_file).unwrap();

    // 生成するRustコードの開始
    writeln!(f, "lazy_static! {{\n pub static ref PEOPLE_ALL: [Person; {}] = [", people.len()).unwrap();

    // 各PersonをRustの配列として書き出し
    for person in people {
        writeln!(
            f,
            "    Person {{ name: \"{}\".to_string(), age: {} }},",
            person.name,
            person.age
        )
        .unwrap();
    }

    writeln!(f, "];\n}}").unwrap();
}