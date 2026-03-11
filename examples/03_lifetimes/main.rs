// ============================================
// 03 — Lifetimes
// ============================================
// Analogie C++:
//   const T& foo(const T& a, const T& b) — welche Lebensdauer hat der Return?
//   Der Compiler in C++ weiß es nicht. Du musst es wissen.
//
// Rust: Lifetime-Annotationen machen es explizit.
// WICHTIG: Lifetimes sind nur Annotationen — kein Runtime-Overhead!

fn main() {
    // --- Beispiel 1: longer() mit Lifetimes ---
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longer(string1.as_str(), string2.as_str());
        println!("Längerer String: '{}'", result); // ✅ hier noch OK
    }
    // println!("{}", result); // ❌ COMPILE ERROR: string2 lebt nicht mehr lang genug

    // --- Beispiel 2: Struct mit Lifetime ---
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = FirstSentence { part: &novel[..i] };
        println!("Erster Satz: '{}'", first_sentence.part);
    }
}

// 'a sagt: Rückgabewert lebt so lange wie das KÜRZERE der beiden Parameter
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct der eine Referenz hält — Lifetime nötig
struct FirstSentence<'a> {
    part: &'a str,  // 'a: dieser struct darf nicht länger leben als der referenzierte String
}
