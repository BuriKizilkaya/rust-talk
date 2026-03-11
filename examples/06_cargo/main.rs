// ============================================
// 06 — Cargo & Tooling
// ============================================
// Vergleich C++ Toolchain vs Rust/Cargo:
//
//  C++                          Rust
//  ─────────────────────────    ─────────────────────
//  cmake / make                 cargo build
//  vcpkg / conan                cargo add <crate>
//  Makefile run target          cargo run
//  Google Test / Catch2         cargo test  (built-in!)
//  clang-tidy                   cargo clippy
//  clang-format                 cargo fmt
//  valgrind                     nicht nötig (Borrow Checker)
//  gdb / lldb                   rust-gdb / CodeLLDB (VSCode)

// Cargo.toml (equivalent zu CMakeLists.txt):
//
// [package]
// name = "my_project"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// serde = { version = "1", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }

// Tests sind direkt im Code — kein separates Framework nötig!
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
    println!();
    println!("Nützliche Cargo-Befehle:");
    println!("  cargo new <name>     — Projekt erstellen");
    println!("  cargo run            — Kompilieren und ausführen");
    println!("  cargo test           — Tests ausführen");
    println!("  cargo clippy         — Linter (sehr empfehlenswert!)");
    println!("  cargo doc --open     — Dokumentation generieren & öffnen");
    println!("  cargo add serde      — Dependency hinzufügen");
}

// Unit Tests — einfach ins selbe File schreiben
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }
}
