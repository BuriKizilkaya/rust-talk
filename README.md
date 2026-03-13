# 🦀 Rust für C/C++ Entwickler — Vortrag

## Voraussetzungen

Installiere [mise](https://mise.jdx.dev/) um die Tool-Versionen zu verwalten:

```bash
curl https://mise.run | sh
```

Dann installiere die Tools:

```bash
mise install
```

## Verfügbare Tools

- **Rust 1.94.0** — wird automatisch aktiviert
- **marp-cli** — für die Slides-Generierung

## Verfügbare Tasks

### Slides generieren

```bash
mise run slides
```

Generiert HTML und PDF Slides aus `slides.md` und legt sie in `_output/` ab.

### Rust Beispiele ausführen

Jedes Beispiel ist ein separater Cargo-Workspace-Member:

```bash
# Alle Beispiele bauen
cargo build

# Einzelnes Beispiel ausführen
cargo run -p 01_ownership
cargo run -p 02_borrowing
cargo run -p 03_lifetimes
cargo run -p 04_error_handling
cargo run -p 05_traits
```

Oder mit mise (für direktes Ausführen aus dem Beispiel-Ordner):

```bash
mise run rust examples/01_ownership
mise run rust examples/02_borrowing
mise run rust examples/03_lifetimes
mise run rust examples/04_error_handling
mise run rust examples/05_traits
```

### C++ Beispiele ausführen (Vergleich)

```bash
mise run cpp examples/01_ownership
mise run cpp examples/02_borrowing
mise run cpp examples/03_lifetimes
mise run cpp examples/04_error_handling
mise run cpp examples/05_traits
```

## Beispiele

| Ordner                       | Thema                  |
| ---------------------------- | ---------------------- |
| `examples/01_ownership`      | Ownership              |
| `examples/02_borrowing`      | Borrowing & References |
| `examples/03_lifetimes`      | Lifetimes              |
| `examples/04_error_handling` | Error Handling         |
| `examples/05_traits`         | Traits                 |

