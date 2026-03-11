// ============================================
// 04 — Error Handling (C++)
// ============================================
// Kompilieren: g++ -std=c++17 main.cpp -o main && ./main

#include <iostream>
#include <fstream>
#include <string>
#include <optional>
#include <variant>
#include <stdexcept>
#include <cerrno>
#include <cstring>

// --- Beispiel 1: C-Style errno — leicht zu vergessen ---
void example_errno() {
    FILE* f = fopen("data.txt", "r");
    if (!f) {
        // Fehler leicht ignorierbar — kein Compiler-Zwang
        std::cerr << "Fehler: " << strerror(errno) << std::endl;
        return;
    }
    fclose(f);
}

// --- Beispiel 2: Exceptions — invisible control flow ---
std::string read_file_exceptions(const std::string& path) {
    std::ifstream f(path);
    if (!f.is_open()) {
        throw std::runtime_error("Datei nicht gefunden: " + path);
        // Aufrufer muss wissen, dass eine Exception geworfen werden kann
        // — steht nicht im Funktionssignatur!
    }
    return std::string((std::istreambuf_iterator<char>(f)),
                        std::istreambuf_iterator<char>());
}

// --- Beispiel 3: std::optional (C++17) — wie Rust's Option<T> ---
std::optional<int> parse_int(const std::string& s) {
    try {
        return std::stoi(s);
    } catch (...) {
        return std::nullopt; // kein Wert
    }
}

// --- Beispiel 4: std::variant als Result<T,E> — C++17 ---
using Result = std::variant<double, std::string>; // Ok oder Err

Result divide(double a, double b) {
    if (b == 0.0) return std::string("Division durch Null");
    return a / b;
}

int main() {
    // errno
    example_errno();

    // Exceptions — muss man wissen, dass es wirft!
    try {
        auto content = read_file_exceptions("user.txt");
        std::cout << content << std::endl;
    } catch (const std::runtime_error& e) {
        std::cerr << "Fehler: " << e.what() << std::endl;
    }

    // optional
    auto n = parse_int("42");
    if (n.has_value()) {
        std::cout << "Geparst: " << n.value() << std::endl;
    }

    auto bad = parse_int("abc");
    std::cout << "Hat Wert: " << bad.has_value() << std::endl;

    // variant als Result
    auto r1 = divide(10.0, 2.0);
    auto r2 = divide(10.0, 0.0);

    if (std::holds_alternative<double>(r1))
        std::cout << "Ergebnis: " << std::get<double>(r1) << std::endl;

    if (std::holds_alternative<std::string>(r2))
        std::cout << "Fehler: " << std::get<std::string>(r2) << std::endl;

    // In Rust: Result<T,E> ist direkt in der Sprache — kein Variant-Hack.
    // Compiler ZWINGT zur Behandlung. ? Operator für elegante Propagation.
    return 0;
}
