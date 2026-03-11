// ============================================
// 05 — Traits & Generics
// ============================================
//
// Traits = Rusts Interfaces — aber ohne Vererbungshierarchie.
//
// C++:  class Shape { virtual double area() = 0; }
//       class Circle : public Shape { ... }   ← Circle IST-EIN Shape
//
// Rust: trait Area { fn area(&self) -> f64; }
//       impl Area for Circle { ... }          ← Circle KANN Area berechnen
//
// Kein "extends", kein "implements" — Komposition statt Vererbung.
// Traits können nachträglich für jeden Typ implementiert werden.

// --------------------------------------------------
// Trait Definition
// --------------------------------------------------
trait Area {
    fn area(&self) -> f64;

    // Default-Implementierung — kann überschrieben werden
    fn beschreibung(&self) -> String {
        format!("Fläche: {:.2}", self.area())
    }
}

// --------------------------------------------------
// Typen — keine Basisklasse nötig
// --------------------------------------------------
struct Kreis {
    radius: f64,
}
struct Rechteck {
    breite: f64,
    hoehe: f64,
}
struct Dreieck {
    basis: f64,
    hoehe: f64,
}

impl Area for Kreis {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rechteck {
    fn area(&self) -> f64 {
        self.breite * self.hoehe
    }
}

impl Area for Dreieck {
    fn area(&self) -> f64 {
        0.5 * self.basis * self.hoehe
    }
    // beschreibung() überschreiben
    fn beschreibung(&self) -> String {
        format!("Dreieck mit Fläche {:.2}", self.area())
    }
}

// --------------------------------------------------
// impl Trait — statischer Dispatch (Zero-Cost)
// Compiler erzeugt für jeden Typ eine eigene Funktion (Monomorphisierung)
// --------------------------------------------------
fn zeige_flaeche(form: &impl Area) {
    println!("[static]  {}", form.beschreibung());
}

// --------------------------------------------------
// dyn Trait — dynamischer Dispatch (vtable, wie C++ virtual)
// Typ erst zur Laufzeit bekannt
// --------------------------------------------------
fn zeige_flaeche_dyn(form: &dyn Area) {
    println!("[dynamic] {}", form.beschreibung());
}

// --------------------------------------------------
// Generics mit Trait-Bounds
// C++: template<typename T>  (keine Constraints sichtbar)
// Rust: <T: PartialOrd>      (Constraint direkt im Typ)
// --------------------------------------------------
fn groesster<T: PartialOrd>(liste: &[T]) -> &T {
    let mut max = &liste[0];
    for item in liste {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let k = Kreis { radius: 3.0 };
    let r = Rechteck {
        breite: 4.0,
        hoehe: 5.0,
    };
    let d = Dreieck {
        basis: 6.0,
        hoehe: 4.0,
    };

    // Statischer Dispatch
    zeige_flaeche(&k);
    zeige_flaeche(&r);
    zeige_flaeche(&d);

    println!();

    // Heterogene Liste — nur mit dyn Trait möglich
    // Entspricht: vector<unique_ptr<Shape>> in C++
    let formen: Vec<Box<dyn Area>> = vec![
        Box::new(Kreis { radius: 1.0 }),
        Box::new(Rechteck {
            breite: 2.0,
            hoehe: 3.0,
        }),
        Box::new(Dreieck {
            basis: 4.0,
            hoehe: 2.0,
        }),
    ];

    for form in &formen {
        zeige_flaeche_dyn(form.as_ref());
    }

    println!();

    // Generics
    let zahlen = vec![34, 50, 25, 100, 65];
    println!("Größte Zahl: {}", groesster(&zahlen));
}
