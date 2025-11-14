use crate::components::ProgressBar;
use crate::{server_fns::SlideStatistics, slides::SlidesPage};
use leptos::prelude::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_ws::provide_websocket;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    provide_websocket();
    let main_ref = NodeRef::<leptos::html::Main>::new();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rustbcn_leptos.css" />

        // sets the document title
        <Title text="Full stack with Leptos - 🦀 Barcelona" />

        // content for this welcome page
        <Router>
            <main node_ref=main_ref>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("slides") view=SlidesPage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePageOld() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let slide_stats =
        leptos_ws::ReadOnlySignal::new("slidestats", SlideStatistics::default()).unwrap();

    let cloned = slide_stats.clone();
    view! {
        <div class="min-h-screen max-h-screen flex flex-col">
            <header>
                <ProgressBar stats=move || slide_stats.get() />
            </header>
            <h1 class="text-2xl">"Welcome to Leptos!"</h1>
            <p>Our slidedeck is in page {move || cloned.get().current + 1}</p>

        </div>
    }
}
