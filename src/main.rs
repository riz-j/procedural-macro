#![allow(dead_code)]
pub use reflective_derive::Reflective;

pub trait Reflective {
    fn name(&self) -> &'static str;
}

/**
* There might be warning saying:
* ```
* proc macro `Reflective` not expanded:
* proc macro library returned no proc macrosrust-analyzerunresolved-proc-macro
* ```
* To fix this, go to Vs Code settings (Ctrl + Shift + P), and select:
* ```
* rust-analyzer: Rebuild proc macros and build scripts
* ```
*/

#[derive(Reflective)]
struct Bar {
    a: i32,
    b: i32,
}

fn main() {
    let bar = Bar { a: 50, b: 25 };

    println!("Hello, world: {}", bar.name());
}