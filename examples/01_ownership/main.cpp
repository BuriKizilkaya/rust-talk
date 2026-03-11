// ============================================
// 01 — Ownership & Move Semantics (C++)
// ============================================
// Kompilieren: g++ -std=c++17 main.cpp -o main && ./main

#include <iostream>
#include <string>
#include <vector>

// --- Beispiel 1: Manuelles Speichermanagement ---
void example_manual_memory() {
    int* ptr = new int(42);
    std::cout << "Wert: " << *ptr << std::endl;
    delete ptr;
    // use(ptr); // Use-after-free — undefined behavior, kein Compiler-Fehler!
}

// --- Beispiel 2: Move Semantics (C++11) — opt-in ---
void example_move() {
    std::vector<int> a = {1, 2, 3};
    std::vector<int> b = std::move(a); // expliziter Move

    // a ist jetzt in "valid but unspecified state"
    // Kein Compiler-Fehler, aber gefährlich:
    a.push_back(4); // funktioniert zufällig, aber UB möglich
    std::cout << "a size nach move: " << a.size() << std::endl;
    std::cout << "b size: " << b.size() << std::endl;
}

// --- Beispiel 3: RAII — Rust's Ownership-Inspiration ---
class Resource {
public:
    Resource(std::string name) : name_(name) {
        std::cout << "Erstellt: " << name_ << std::endl;
    }
    ~Resource() {
        std::cout << "Zerstört: " << name_ << std::endl;
        // In Rust: automatisch, kein Destruktor nötig
    }
private:
    std::string name_;
};

void example_raii() {
    Resource r("meine-ressource"); // automatisch zerstört am Scope-Ende
    // Aber: kein Schutz gegen Copies, mehrfaches delete, etc.
}

// --- Beispiel 4: Dangling Pointer — kein Compiler-Schutz ---
int* make_dangling() {
    int x = 42;
    return &x; // ⚠️ x wird zerstört — Dangling Pointer!
               // Compiler warnt vielleicht, verhindert es NICHT
}

int main() {
    std::cout << "=== Manual Memory ===" << std::endl;
    example_manual_memory();

    std::cout << "\n=== Move Semantics ===" << std::endl;
    example_move();

    std::cout << "\n=== RAII ===" << std::endl;
    example_raii();

    // Dangling Pointer — UB, kein Compile Error:
    // int* p = make_dangling();
    // std::cout << *p; // 💥 undefined behavior

    return 0;
}
