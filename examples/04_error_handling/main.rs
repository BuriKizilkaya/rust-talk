// ============================================
// 04 — Error Handling: Result<T, E> & Option<T>
// ============================================
// Analogie C++:
//   errno           → Result<T, E>   (Fehler-Rückgabe)
//   nullptr checks  → Option<T>      (Nullable Werte)
//   exceptions      → Result<T, E>   (aber explizit!)
//
// Rust: Fehler sind TYPEN — der Compiler zwingt zur Behandlung

use std::fs;
use std::num::ParseIntError;

fn main() {
    // --- Beispiel 1: match auf Result ---
    let result = divide(10.0, 0.0);
    match result {
        Ok(val)  => println!("Ergebnis: {}", val),
        Err(e)   => println!("Fehler: {}", e),
    }

    // --- Beispiel 2: Option<T> statt nullptr ---
    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.get(10) {   // gibt Option<&i32>
        Some(n) => println!("Gefunden: {}", n),
        None    => println!("Index out of bounds — kein Crash!"),
    }

    // --- Beispiel 3: ? Operator für Fehler-Propagation ---
    match read_and_parse() {
        Ok(n)  => println!("Geparste Zahl: {}", n),
        Err(e) => println!("Fehler beim Lesen/Parsen: {}", e),
    }

    // --- Beispiel 4: Kurzformen ---
    let val = divide(10.0, 2.0).unwrap_or(0.0);  // Default bei Fehler
    println!("unwrap_or: {}", val);

    let val2 = divide(10.0, 2.0).expect("Division fehlgeschlagen"); // panic bei Fehler
    println!("expect: {}", val2);
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division durch Null"))
    } else {
        Ok(a / b)
    }
}

// ? Operator: bei Err → early return, wie Go's "if err != nil { return err }"
fn read_and_parse() -> Result<i32, Box<dyn std::error::Error>> {
    // Datei lesen (in echt: fs::read_to_string("zahl.txt")?)
    let content = "42"; // simuliert
    let n: i32 = content.trim().parse()?; // ParseIntError wird mit ? propagiert
    Ok(n * 2)
}
