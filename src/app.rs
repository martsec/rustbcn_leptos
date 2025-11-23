use crate::home::HomePage;
use crate::slides::SlidesPage;
use leptos::prelude::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_ws::provide_websocket;
use singlestage::ThemeProvider;

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
                      <link href="prism.css" rel="stylesheet" />
                <script type="module">
      import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
      mermaid.initialize({ startOnLoad: true });
    </script>
                  </head>
                  <body>
                      <App />
                      <script defer src="prism.js"></script>
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
        <Title text="🦀 Full stack: Leptos - 🦀 Barcelona" />

        // content for this welcome page
        <ThemeProvider>
            <Router>
                <main node_ref=main_ref>
                    <Routes fallback=|| view! { <h1>"Page not found"</h1> }>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("slides") view=SlidesPage />
                    </Routes>
                </main>
            </Router>
        </ThemeProvider>
    }
}
