// ============================================
// 03 — Lifetimes & Dangling Pointers (C++)
// ============================================
// Kompilieren: g++ -std=c++17 main.cpp -o main && ./main
//
// C++ hat KEIN Lifetime-System.
// Der Entwickler ist selbst verantwortlich — der Compiler hilft kaum.

#include <iostream>
#include <string>
#include <string_view>

// --- Beispiel 1: Welche Lebensdauer hat der Rückgabewert? ---
// C++: Compiler weiss es nicht — du musst es wissen.
const std::string& longer(const std::string& a, const std::string& b) {
    return a.size() >= b.size() ? a : b;
    // Lebensdauer hängt davon ab, welcher Parameter zurückgegeben wird.
    // Kein Compile-Fehler, wenn Aufrufer es falsch verwendet!
}

// --- Beispiel 2: Dangling Reference — klassischer Bug ---
const std::string& get_longer_broken() {
    std::string temp = "short";
    std::string s1   = "very long string";
    return longer(s1, temp);
    // s1 und temp werden zerstört — Dangling Reference!
    // Kein Compile-Fehler.
}

// --- Beispiel 3: string_view — ähnlich wie Rust &str ---
// string_view hat dasselbe Problem: Lifetime manuell verwalten
std::string_view first_word(const std::string& s) {
    size_t pos = s.find(' ');
    return std::string_view(s).substr(0, pos);
}

void string_view_dangling() {
    std::string_view word;
    {
        std::string sentence = "hello world";
        word = first_word(sentence);
        std::cout << "Innerhalb: " << word << std::endl;
    } // sentence wird zerstört
    // std::cout << word; // 💥 Dangling string_view — UB!
    // In Rust: ❌ COMPILE ERROR — Lifetime-Checker verhindert das
}

// --- Beispiel 4: Struct mit Pointer — Lifetime-Verantwortung beim Dev ---
struct Excerpt {
    const char* part; // roher Pointer — wer besitzt das?
                      // In Rust: struct Excerpt<'a> { part: &'a str }
                      // — Compiler erzwingt korrekte Lifetime
};

int main() {
    std::string s1 = "long string is long";
    std::string s2 = "xyz";

    const std::string& result = longer(s1, s2);
    std::cout << "Länger: " << result << std::endl;

    // Gefährlicher Aufruf — kein Compile-Fehler:
    // const std::string& dangling = get_longer_broken();
    // std::cout << dangling; // 💥

    string_view_dangling();
    return 0;
}
