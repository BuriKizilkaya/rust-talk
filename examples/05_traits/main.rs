// ============================================
// 05 — Traits & Generics
// ============================================
// Analogie C++:
//   class Shape { virtual double area() = 0; }  → trait Area
//   template<typename T>                         → fn foo<T: Trait>
//
// Rust:
//   impl Trait  → statischer Dispatch (Zero-Cost, wie C++ Templates)
//   dyn Trait   → dynamischer Dispatch (vtable, wie C++ virtual)

use std::fmt;

// --- Trait Definition (wie pure virtual class in C++) ---
trait Area {
    fn area(&self) -> f64;
    fn describe(&self) -> String {  // Default-Implementierung möglich
        format!("Form mit Fläche {:.2}", self.area())
    }
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// --- Statischer Dispatch (wie C++ Templates — Zero-Cost) ---
fn print_area_static(shape: &impl Area) {
    println!("[static]  {}", shape.describe());
}

// --- Dynamischer Dispatch (wie C++ virtual — vtable) ---
fn print_area_dynamic(shape: &dyn Area) {
    println!("[dynamic] {}", shape.describe());
}

// --- Generics mit Bounds (explizit, klare Fehlermeldungen) ---
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

// --- Mehrere Trait Bounds (wie C++20 Concepts) ---
fn print_and_compare<T: fmt::Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} ist größer als {}", a, b);
    } else {
        println!("{} ist kleiner/gleich {}", a, b);
    }
}

fn main() {
    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };

    print_area_static(&c);
    print_area_static(&r);

    // Heterogene Collection mit dyn Trait (wie vector<unique_ptr<Shape>>)
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Circle { radius: 5.0 }),
    ];
    for shape in &shapes {
        print_area_dynamic(shape.as_ref());
    }

    // Generics
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Größte Zahl: {}", largest(&numbers));

    print_and_compare(3.14, 2.71);
}
