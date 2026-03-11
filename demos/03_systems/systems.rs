// ============================================
// DEMO 3: Systems / Performance — CSV Parser
// ============================================
// Zeigt: Zero-Copy Parsing, Iteratoren, FFI, Benchmarking
// Echte Projekte: ripgrep, rust-analyzer, SWC (JS Compiler), Turbopack
//
// Ausführen: rustc -O systems.rs && ./systems

use std::time::Instant;

// Zero-Copy: Slices zeigen in den Originaldaten — kein Kopieren
#[derive(Debug)]
struct CsvRecord<'a> {
    name:   &'a str,
    age:    u32,
    city:   &'a str,
    salary: f64,
}

fn parse_record(line: &str) -> Option<CsvRecord<'_>> {
    let mut f = line.splitn(4, ',');
    Some(CsvRecord {
        name:   f.next()?.trim(),
        age:    f.next()?.trim().parse().ok()?,
        city:   f.next()?.trim(),
        salary: f.next()?.trim().parse().ok()?,
    })
}

fn main() {
    let csv_data = generate_csv(1_000);

    let start = Instant::now();

    let records: Vec<CsvRecord> = csv_data
        .lines()
        .skip(1)
        .filter_map(parse_record)
        .collect();

    println!("✅ {} Records in {:.2?} geparst", records.len(), start.elapsed());

    let avg_salary = records.iter().map(|r| r.salary).sum::<f64>() / records.len() as f64;

    let top = records.iter()
        .max_by(|a, b| a.salary.partial_cmp(&b.salary).unwrap())
        .unwrap();

    let berlin = records.iter().filter(|r| r.city == "Berlin").count();

    println!("📊 Ø Gehalt:      {:.0}€", avg_salary);
    println!("🏆 Höchstes:      {} ({:.0}€)", top.name, top.salary);
    println!("🏙️  Berlin:        {} Personen", berlin);

    // Direkt C-Funktionen aufrufen — zero overhead
    println!("\n--- FFI: C stdlib direkt aufrufen ---");
    let result = unsafe { abs(-42) };
    println!("abs(-42) = {}", result);
}

fn generate_csv(n: usize) -> String {
    let mut out = String::from("name,age,city,salary\n");
    let cities = ["Berlin", "Munich", "Hamburg", "Frankfurt"];
    let names  = ["Alice", "Bob", "Carol", "Dave", "Eve"];
    for i in 0..n {
        out.push_str(&format!(
            "{},{},{},{:.2}\n",
            names[i % names.len()],
            25 + (i % 40),
            cities[i % cities.len()],
            40_000.0 + (i * 137 % 60_000) as f64,
        ));
    }
    out
}

extern "C" { fn abs(x: i32) -> i32; }
