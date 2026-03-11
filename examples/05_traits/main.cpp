// ============================================
// 05 — Traits & Generics (C++)
// ============================================
// Kompilieren: g++ -std=c++17 main.cpp -o main && ./main

#include <iostream>
#include <vector>
#include <memory>
#include <cmath>

// --- Beispiel 1: Pure virtual class = Rust trait ---
class Area {
public:
    virtual double area() const = 0; // pure virtual — wie Rust trait
    virtual std::string describe() const {
        return "Form mit Fläche " + std::to_string(area());
    }
    virtual ~Area() = default;
};

struct Circle : public Area {
    double radius;
    Circle(double r) : radius(r) {}
    double area() const override { return M_PI * radius * radius; }
};

struct Rectangle : public Area {
    double width, height;
    Rectangle(double w, double h) : width(w), height(h) {}
    double area() const override { return width * height; }
};

// --- Beispiel 2: Dynamischer Dispatch (vtable) ---
// Entspricht Rust: fn print_area(shape: &dyn Area)
void print_area_dynamic(const Area& shape) {
    std::cout << "[dynamic] " << shape.describe() << std::endl;
}

// --- Beispiel 3: Templates = statischer Dispatch ---
// Entspricht Rust: fn print_area(shape: &impl Area)
// Problem: Kein explizites Interface — Fehler erst bei Instanziierung
template<typename T>
void print_area_static(const T& shape) {
    std::cout << "[static]  " << shape.describe() << std::endl;
    // Was T sein muss? Unklar aus der Signatur!
    // In Rust: fn print_area<T: Area>(shape: &T) — explizit
}

// --- Beispiel 4: C++20 Concepts — expliziter wie Rust Traits ---
// (Rust hatte das schon immer durch Trait Bounds)
#if __cplusplus >= 202002L
#include <concepts>
template<typename T>
concept HasArea = requires(T t) {
    { t.area() } -> std::convertible_to<double>;
};

template<HasArea T>
void print_area_concept(const T& shape) {
    std::cout << "[concept] " << shape.area() << std::endl;
}
#endif

int main() {
    Circle    c{3.0};
    Rectangle r{4.0, 5.0};

    print_area_static(c);
    print_area_static(r);

    // Heterogene Collection — braucht Pointer (unique_ptr)
    // Entspricht Rust: Vec<Box<dyn Area>>
    std::vector<std::unique_ptr<Area>> shapes;
    shapes.push_back(std::make_unique<Circle>(1.0));
    shapes.push_back(std::make_unique<Rectangle>(2.0, 3.0));
    shapes.push_back(std::make_unique<Circle>(5.0));

    for (const auto& s : shapes) {
        print_area_dynamic(*s);
    }

    return 0;
}
