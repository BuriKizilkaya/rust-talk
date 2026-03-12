// ============================================
// 01 — Ownership
// ============================================
//
// Kernregel: Jeder Wert hat genau EINEN Owner.
// Endet der Scope des Owners → Wert wird automatisch gedroppt.
//
// C++:  int* p = new int(42);  delete p;  // du bist verantwortlich
// Rust: Der Compiler übernimmt das.

fn main() {
    // --------------------------------------------------
    // 1. Scope & automatisches Drop
    // --------------------------------------------------
    {
        let s = String::from("hello"); // s wird allokiert
        println!("s = {}", s);
    } // ← s wird hier automatisch gedroppt (kein delete nötig)

    // --------------------------------------------------
    // 2. Ownership in Funktionen übergeben (Move)
    // --------------------------------------------------
    let name = String::from("Alice");
    begruesse(name); // Ownership geht an Funktion
    // println!("{}", name);       // ❌ COMPILE ERROR: name wurde moved

    // --------------------------------------------------
    // 3. Ownership zurückbekommen
    // --------------------------------------------------
    let name2 = String::from("Bob");
    let name2 = gib_zurueck(name2); // Ownership kommt zurück
    println!("Zurück: {}", name2); // ✅

    // --------------------------------------------------
    // 4. Copy-Typen — primitives werden kopiert, nicht gemoved
    // --------------------------------------------------
    let x: i32 = 42;
    let y = x; // Kopie, kein Move
    println!("x = {}, y = {}", x, y); // ✅ beide gültig
}

fn begruesse(s: String) {
    println!("Hallo, {}!", s);
} // s wird hier gedroppt

fn gib_zurueck(s: String) -> String {
    s // Ownership an den Aufrufer zurückgeben
}
