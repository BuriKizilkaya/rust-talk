// ============================================
// 03 — Lifetimes
// ============================================
//
// Jede Referenz hat eine Lifetime — wie lange sie gültig ist.
// Meistens inferiert der Compiler sie automatisch (Lifetime Elision).
// Explizite Annotationen nötig wenn der Compiler es nicht selbst herleiten kann.
//
// WICHTIG: Lifetimes sind reine Compiler-Annotationen — kein Runtime-Overhead!

fn main() {
    // --------------------------------------------------
    // 1. Dangling Reference — Compiler fängt es ab
    // --------------------------------------------------
    let r;
    {
        let x = String::from("hello"); // x lebt nur im Block
        r = &x;
        println!("r = {}", r); // ✅ hier noch OK
    } // x wird hier gedroppt
    // println!("{}", r);              // ❌ COMPILE ERROR: x lebt nicht lang genug

    // --------------------------------------------------
    // 2. longer() — explizite Lifetime nötig
    // --------------------------------------------------
    // Der Compiler muss wissen: wie lange lebt der Rückgabewert?
    let s1 = String::from("langer string");
    let ergebnis;
    {
        let s2 = String::from("kurz");
        ergebnis = longer(s1.as_str(), s2.as_str());
        println!("Längerer: '{}'", ergebnis); // ✅
    }
    // println!("{}", ergebnis); // ❌ COMPILE ERROR: s2 lebt nicht mehr

    // --------------------------------------------------
    // 3. Lifetime Elision — Compiler inferiert automatisch
    // --------------------------------------------------
    let s = String::from("hello world");
    let wort = erstes_wort(&s); // Lifetime wird automatisch inferiert
    println!("Erstes Wort: '{}'", wort); // ✅
}

// 'a bedeutet: Rückgabewert lebt so lange wie das KÜRZERE der beiden Parameter
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Hier inferiert der Compiler die Lifetime automatisch:
// fn erstes_wort<'a>(s: &'a str) -> &'a str { ... }
fn erstes_wort(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
