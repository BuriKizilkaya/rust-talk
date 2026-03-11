// ============================================
// 01 — Ownership & Move Semantics
// ============================================
// Analogie C++:
//   int* ptr = new int(42);
//   delete ptr;  // du bist verantwortlich
//
// Rust: Compiler übernimmt das automatisch

fn main() {
    // --- Beispiel 1: Basic Ownership ---
    let s1 = String::from("hello");
    let s2 = s1; // Move! s1 ist nicht mehr gültig

    // println!("{}", s1); // ❌ COMPILE ERROR: value borrowed here after move
    println!("s2 = {}", s2); // ✅

    // --- Beispiel 2: Clone (explizite Kopie wie memcpy) ---
    let s3 = String::from("world");
    let s4 = s3.clone(); // explizite deep copy
    println!("s3 = {}, s4 = {}", s3, s4); // ✅ beide gültig

    // --- Beispiel 3: Copy-Types (primitives, wie in C) ---
    let x = 5;
    let y = x; // Copy, kein Move (integers sind Copy)
    println!("x = {}, y = {}", x, y); // ✅ beide gültig

    // --- Beispiel 4: Ownership in Funktionen ---
    let s5 = String::from("ownership");
    takes_ownership(s5); // s5 wird gemoved
    // println!("{}", s5); // ❌ COMPILE ERROR

    let x = makes_copy(42); // i32 wird kopiert
    println!("x zurück: {}", x); // ✅
}

fn takes_ownership(s: String) {
    println!("Ich besitze jetzt: {}", s);
} // s wird hier gedroppt → free()

fn makes_copy(x: i32) -> i32 {
    println!("Kopie von: {}", x);
    x
}
