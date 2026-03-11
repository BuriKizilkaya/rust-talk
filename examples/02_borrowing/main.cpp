// ============================================
// 02 — Borrowing & References (C++)
// ============================================
// Kompilieren: g++ -std=c++17 main.cpp -o main && ./main

#include <iostream>
#include <string>
#include <vector>

// --- Beispiel 1: const Referenz (immutable borrow) ---
size_t calculate_length(const std::string& s) {
    return s.length(); // s wird nicht verändert, nicht kopiert
}

// --- Beispiel 2: Referenz (mutable borrow) ---
void change(std::string& s) {
    s += ", world";
}

// --- Beispiel 3: Das Problem — C++ verhindert NICHTS ---
void aliasing_problem() {
    std::string s = "hello";

    std::string& r1 = s; // mutable Referenz
    std::string& r2 = s; // zweite mutable Referenz — erlaubt in C++!

    r1 += " from r1";
    r2 += " from r2"; // gleichzeitiger Zugriff — kein Fehler

    std::cout << s << std::endl;
    // In Rust: ❌ COMPILE ERROR — nur eine &mut Referenz erlaubt
}

// --- Beispiel 4: Dangling Reference — Compiler verhindert es nicht ---
const std::string& get_dangling() {
    std::string local = "ich werde zerstört";
    return local; // ⚠️ Dangling Reference — UB!
    // Compiler warnt, verhindert aber nicht
}

// --- Beispiel 5: Iterator Invalidation ---
void iterator_invalidation() {
    std::vector<int> v = {1, 2, 3};
    auto& first = v[0]; // Referenz auf erstes Element

    v.push_back(4); // Reallokation! first ist jetzt DANGLING
    // std::cout << first; // 💥 Undefined Behavior
    // In Rust: ❌ COMPILE ERROR — kann nicht passieren
}

int main() {
    std::string s = "hello";

    std::cout << "Länge: " << calculate_length(s) << std::endl;
    std::cout << "Vorher: " << s << std::endl;

    change(s);
    std::cout << "Nachher: " << s << std::endl;

    std::cout << "\n=== Aliasing (in Rust verboten) ===" << std::endl;
    aliasing_problem();

    return 0;
}
