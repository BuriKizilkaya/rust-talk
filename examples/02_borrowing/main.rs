// ============================================
// 02 — Borrowing & Referenzen
// ============================================
//
// Borrowing = Ausleihen ohne Ownership zu übertragen.
// Der Owner bleibt Eigentümer — die Referenz ist nur temporär.
//
// Regeln (Compile-Zeit geprüft, kein Runtime-Overhead):
//   &T     → immutable borrow  — beliebig viele gleichzeitig
//   &mut T → mutable borrow    — genau eine, exklusiv

fn main() {
    // --------------------------------------------------
    // 1. Immutable Borrow — Owner bleibt Eigentümer
    // --------------------------------------------------
    let s = String::from("hello");

    let laenge = berechne_laenge(&s); // & = borrow, kein Move
    println!("'{}' hat {} Zeichen", s, laenge); // ✅ s gehört noch main

    // --------------------------------------------------
    // 2. Mehrere Leser gleichzeitig — kein Problem
    // --------------------------------------------------
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{} {} {}", r1, r2, r3); // ✅ alle lesen gleichzeitig

    // --------------------------------------------------
    // 3. Mutable Borrow — exklusiver Schreibzugriff
    // --------------------------------------------------
    let mut text = String::from("hello");

    aendern(&mut text);
    println!("Geändert: {}", text); // ✅

    // --------------------------------------------------
    // 4. Nicht gleichzeitig lesen & schreiben
    // --------------------------------------------------
    let mut data = String::from("hello");

    let leser = &data; // ✅ immutable borrow
    // aendern(&mut data);      // ❌ COMPILE ERROR: cannot borrow as mutable
                                //    because it is also borrowed as immutable
    println!("{}", leser); // leser wird hier nicht mehr genutzt

    aendern(&mut data); // ✅ jetzt OK — leser ist nicht mehr aktiv
    println!("{}", data);
}

fn berechne_laenge(s: &String) -> usize {
    s.len() // s wird nicht gedroppt — nur geliehen
}

fn aendern(s: &mut String) {
    s.push_str(", world");
}

// Diese Funktion kompiliert NICHT — Dangling Reference:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s   // ❌ s wird am Ende des Scopes gedroppt
// }        //    die Referenz würde auf ungültigen Speicher zeigen
