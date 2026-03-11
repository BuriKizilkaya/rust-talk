# 06 — Cargo & Tooling (C++ Vergleich)

C++ hat kein einheitliches Build- und Tooling-System.
Jedes Projekt setzt andere Tools ein — hier die typische Kombination:

## Typischer C++ Workflow

```bash
# Build-System: CMake
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build

# Paketmanager (einer von mehreren)
vcpkg install nlohmann-json
conan install . --build=missing

# Tests: Google Test oder Catch2 (extern!)
# muss separat installiert und konfiguriert werden

# Linter
clang-tidy src/main.cpp -- -std=c++17

# Formatter
clang-format -i src/main.cpp

# Memory-Check
valgrind --leak-check=full ./build/my_app

# Debugger
gdb ./build/my_app
```

## Äquivalente Rust/Cargo Befehle

| C++ | Rust/Cargo |
|---|---|
| `cmake -B build && cmake --build build` | `cargo build` |
| `./build/my_app` | `cargo run` |
| `cmake --build build --config Release` | `cargo build --release` |
| Google Test / Catch2 + cmake setup | `cargo test` (built-in!) |
| vcpkg / conan install | `cargo add <crate>` |
| clang-tidy | `cargo clippy` |
| clang-format | `cargo fmt` |
| valgrind | nicht nötig (Borrow Checker) |
| Doxygen | `cargo doc --open` |

## CMakeLists.txt vs Cargo.toml

```cmake
# CMakeLists.txt — C++
cmake_minimum_required(VERSION 3.20)
project(my_project VERSION 0.1.0)
set(CMAKE_CXX_STANDARD 17)

find_package(nlohmann_json REQUIRED)
add_executable(my_app src/main.cpp)
target_link_libraries(my_app nlohmann_json::nlohmann_json)
```

```toml
# Cargo.toml — Rust
[package]
name    = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1"
```
