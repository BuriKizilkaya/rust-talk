// ============================================
// 02 — Borrowing & References
// ============================================
// Analogie C++:
//   void use(const std::string& s)  → &s (immutable borrow)
//   void modify(std::string& s)     → &mut s (mutable borrow)
//
// Rust-Regel:
//   Entweder N * &T  ODER  1 * &mut T — nie beides gleichzeitig

fn main() {
    // --- Beispiel 1: Immutable Borrow ---
    let s = String::from("hello");
    let len = calculate_length(&s); // leihen, nicht verschieben
    println!("'{}' hat Länge {}", s, len); // s ist noch gültig ✅

    // --- Beispiel 2: Mutable Borrow ---
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Geändert: {}", s2); // ✅

    // --- Beispiel 3: Borrow-Regeln ---
    let mut s3 = String::from("hello");

    let r1 = &s3; // ✅ immutable
    let r2 = &s3; // ✅ zweiter immutable
    println!("{} und {}", r1, r2);
    // Nach diesem println! werden r1, r2 nicht mehr genutzt → Borrows enden

    let r3 = &mut s3; // ✅ jetzt mutable borrow OK
    r3.push_str(", world");
    println!("{}", r3);

    // --- Beispiel 4: Kein Dangling Pointer möglich ---
    // let reference = dangle(); // ❌ würde nicht kompilieren
}

fn calculate_length(s: &String) -> usize {
    s.len() // s wird nicht gedroppt — nur geliehen
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// Diese Funktion kompiliert NICHT — zeigen und erklären:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ❌ s wird hier gedroppt, Referenz wäre dangling
// }
