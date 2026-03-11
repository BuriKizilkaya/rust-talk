// ============================================
// DEMO 6: Web UI — Leptos
// ============================================
// Zeigt: reaktive Komponenten, kein Virtual DOM, WASM im Browser
// Echte Projekte: Zed Editor (eigenes GPU-UI-Framework in Rust),
//                 Leptos (wachsende Community), Dioxus
//
// Setup:
//   cargo install trunk
//   rustup target add wasm32-unknown-unknown
//   trunk serve   ← Hot-Reload wie Vite!
//
// Cargo.toml:
// [dependencies]
// leptos = { version = "0.6", features = ["csr"] }

use leptos::*;

// Reaktive Komponente — kompiliert direkt zu WASM, kein JS-Framework
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let doubled     = move || count() * 2;
    let is_positive = move || count() > 0;

    view! {
        <div class="counter">
            <h2>"Rust Counter 🦀"</h2>
            <p class:positive=is_positive>
                "Count: " {count} " (doubled: " {doubled} ")"
            </p>
            <button on:click=move |_| set_count.update(|n| *n -= 1)>"-"</button>
            <button on:click=move |_| set_count.set(0)>"Reset"</button>
            <button on:click=move |_| set_count.update(|n| *n += 1)>"+"</button>
        </div>
    }
}

#[component]
fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec!["Rust lernen 🦀".to_string()]);
    let (input, set_input) = create_signal(String::new());

    let add = move |_| {
        let val = input();
        if !val.is_empty() {
            set_todos.update(|t| t.push(val));
            set_input.set(String::new());
        }
    };

    view! {
        <div>
            <h2>"Todos"</h2>
            <input
                prop:value=input
                on:input=move |e| set_input.set(event_target_value(&e))
                placeholder="Neue Aufgabe..."
            />
            <button on:click=add>"+"</button>
            <ul>
                <For each=todos key=|t| t.clone() children=|t| view! { <li>{t}</li> }/>
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <Counter /> <TodoApp /> })
}
