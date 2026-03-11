// ============================================
// DEMO 4: WebAssembly (WASM)
// ============================================
// Zeigt: Rust → WASM, JS-Interop, Structs als JS-Klassen
// Echte Projekte: Figma (Rendering), Google Earth Web,
//                 Cloudflare Workers, Fastly Compute@Edge
//
// Setup:
//   rustup target add wasm32-unknown-unknown
//   cargo install wasm-bindgen-cli
//   cargo build --target wasm32-unknown-unknown --release
//   wasm-bindgen --out-dir ./out --target web target/.../pkg.wasm
//
// Cargo.toml:
// [lib]
// crate-type = ["cdylib"]
// [dependencies]
// wasm-bindgen = "0.2"
// web-sys = { version = "0.3", features = ["console"] }

use wasm_bindgen::prelude::*;

// Direkt aus JavaScript aufrufbar
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                (a, b) = (b, a + b);
            }
            b
        }
    }
}

// Rust-Struct wird zur JS-Klasse
#[wasm_bindgen]
pub struct Counter {
    value: i32,
    step:  i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(step: i32) -> Counter { Counter { value: 0, step } }
    pub fn increment(&mut self) { self.value += self.step; }
    pub fn decrement(&mut self) { self.value -= self.step; }
    pub fn reset(&mut self)     { self.value = 0; }
    pub fn get(&self) -> i32    { self.value }
}

// Browser-API direkt aufrufen
#[wasm_bindgen]
pub fn greet(name: &str) {
    web_sys::console::log_1(&format!("Hello, {}! 🦀", name).into());
}

// -----------------------------------------------
// JavaScript:
//
// import init, { fibonacci, Counter, greet } from './pkg.js';
// await init();
//
// console.log(fibonacci(10));      // → 55
//
// const c = new Counter(5);
// c.increment();
// console.log(c.get());            // → 5
//
// greet("World");                  // → "Hello, World! 🦀"
// -----------------------------------------------

fn main() {
    println!("fibonacci(10) = {}", fibonacci(10));
    println!("fibonacci(20) = {}", fibonacci(20));
    let mut c = Counter { value: 0, step: 3 };
    c.increment(); c.increment();
    println!("Counter: {}", c.get());
}
