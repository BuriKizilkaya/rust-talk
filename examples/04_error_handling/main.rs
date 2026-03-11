// ============================================
// 04 — Error Handling: Result<T, E> & Option<T>
// ============================================
//
// In C++ gibt es kein eingebautes Result — Fehler per:
//   errno, Rückgabecodes, Exceptions — alle ignorierbar!
//
// In Rust sind Fehler Teil des Typsystems:
//   Result<T, E> → Ok(wert) oder Err(fehler)
//   Option<T>    → Some(wert) oder None  (für nullable Werte)
//
// Unbehandelte Fehler → Compile Warning/Error

fn main() {
    // --------------------------------------------------
    // 1. Result<T, E> mit match behandeln
    // --------------------------------------------------
    match dividiere(10.0, 0.0) {
        Ok(n) => println!("Ergebnis: {}", n),
        Err(e) => println!("Fehler: {}", e), // "Division durch Null"
    }

    match dividiere(10.0, 2.0) {
        Ok(n) => println!("Ergebnis: {}", n), // 5.0
        Err(e) => println!("Fehler: {}", e),
    }

    // --------------------------------------------------
    // 2. Option<T> — kein nullptr, kein Crash
    // --------------------------------------------------
    let zahlen = vec![1, 2, 3];

    match zahlen.get(1) {
        Some(n) => println!("Index 1: {}", n), // ✅ 2
        None => println!("Nicht gefunden"),
    }

    match zahlen.get(99) {
        Some(n) => println!("Index 99: {}", n),
        None => println!("Nicht gefunden"), // ✅ kein Crash
    }

    // --------------------------------------------------
    // 3. ? Operator — Fehler propagieren ohne Boilerplate
    // --------------------------------------------------
    match parse_und_verdopple("21") {
        Ok(n) => println!("Ergebnis: {}", n), // 42
        Err(e) => println!("Fehler: {}", e),
    }

    match parse_und_verdopple("abc") {
        Ok(n) => println!("Ergebnis: {}", n),
        Err(e) => println!("Fehler: {}", e), // Parse-Fehler
    }

    // --------------------------------------------------
    // 4. Kurzformen
    // --------------------------------------------------
    let a = dividiere(10.0, 0.0).unwrap_or(0.0); // Default bei Fehler
    let b = dividiere(10.0, 2.0).unwrap_or(0.0); // 5.0
    println!("unwrap_or: {} / {}", a, b);
}

fn dividiere(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division durch Null"))
    } else {
        Ok(a / b)
    }
}

// ? Operator: bei Err → sofortiger early return mit dem Fehler
// Entspricht in C++ etwa: if (err) return err;
fn parse_und_verdopple(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let n: i32 = s.trim().parse()?; // ParseIntError wird automatisch propagiert
    Ok(n * 2)
}
