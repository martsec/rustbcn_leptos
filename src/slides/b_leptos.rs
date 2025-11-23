use crate::{
    components::Code,
    slides::{Appear, Replace, Slide},
};
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn UiComponent() -> impl IntoView {
    let title = "Building User Interfaces: Component";
    let notes = r#"Building the client side. Under the hood, snipped of JS that will load leptos WebAssembly compilation to drive the interactivity in the Client-Side rendered web.

    The main part, like in some JS frameworks, is a component

    "#;

    let code = r#"use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = Signal::new(0);

    view! {
        <button
            on:click=move |_| *set_count.write() += 1 
        >
            "Click me: "{count}
        </button>
        <p>
            "Double count: "{move || count.get() * 2}
        </p>
    }
}"#;
    view! {
      <Slide title=title notes=notes>
        <p>Similar to HTML elements, they represent a section of the DOM</p>
        <p> The body is a set-up function. Runs once</p>
        <p>Views have JSX-like format</p>
        <Code code=code />
      </Slide>
    }
}

#[component]
pub fn ViewMacro() -> impl IntoView {
    let notes = r#"The view macro provides a similar html elements but with some extras. 

    We use 

    "#;
    let code = r#"use crate::slides::{Appear, Slide};
use leptos::prelude::*;

#[component]
pub fn ViewMacro() -> impl IntoView {
    let code = "...";
    view! {
        <Slide title="Typecheck your HTML">
            <p>"Let's apply recursion on this page"</p>
            <code>
                {code}
            </code>
        </Slide>
    }
}"#;
    view! {
        <Slide title="Typecheck your HTML">
            <p>"Let's apply recursion on this page"</p>
            <Code code=code/>
        </Slide>
    }
}
#[component]
pub fn Reactivity() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let code = r#"#[component]
pub fn Reactivity() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    view! {
        <Slide title="What are we going to see?">
            <Button size="large" variant="normal" on:click=on_click class:red=move || count.get() % 2 == 1>
                "Click Me: "{count}
            </Button>
        </Slide>
    }
}"#;

    view! {
        <Slide title="Reactivity">

        <p>Special syntax for listeners <code>r#"on:click"#</code> and dynamic attributes <code>r#"class:red=move || count.get() % 2 == 1"#</code></p>
            <Button size="large" variant="normal" on:click=on_click class:red=move || count.get() % 2 == 1>
                "Click Me: "{count}
            </Button>
            <Appear id=1><Code code=code/></Appear>
        </Slide>


    }
}

#[component]
pub fn Contexts() -> impl IntoView {
    let title = "Passing contexts";
    let notes = r#""#;
    view! {
      <Slide title=title notes=notes>
        <p>TODO</p>
      </Slide>
    }
}

#[component]
pub fn Stores() -> impl IntoView {
    let title = "";
    let notes = r#""#;
    view! {
      <Slide title=title notes=notes>
        <p>TODO</p>
      </Slide>
    }
}

#[component]
pub fn Server() -> impl IntoView {
    let title = "Server functions";
    let notes = r#"Until now we've only worked wit the render. There is nothing much.
    And it's completely fine.
    However, applications like this one require a bit extra.

    In Js, you are going to call REST APIs, in php, you just do server-side code, and add interactivity after. 

    With leptos, we do both even in the same files!
    For this, we can choose our backend library: axum or actix. But it can be hidden from us!

    We just define server functions.
    "#;

    let code = r#"#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SlideStatistics {
    pub current: u32,
    pub total: u32,
}

#[server]
pub async fn update_slides(stats: SlideStatistics) -> Result<(), ServerFnError> {
    let ws = leptos_ws::ReadOnlySignal::new("slidestats", SlideStatistics::default()).unwrap();
    ws.update(move |v| *v = stats);
    Ok(())
}
"#;

    view! {
      <Slide title=title notes=notes>
                <Replace id=1>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 place-items-center">
    <p class="text-xl">Axum</p>
    <p class="text-xl">Actix</p>
        </div>
                </Replace>
                <Appear id=3>klasjdlksjdslakd</Appear>
                <Appear id=2><Code code=code/>
                </Appear>
      </Slide>
    }
}
#[component]
pub fn Server2() -> impl IntoView {
    let title = "Server functions";
    let notes = r#"Show the developer console.

    The effect executes every time the signal changes."#;

    let code = r#"#[component]
fn SlideDeck() -> impl IntoView {
    let current = RwSignal::new(0_u32);
    provide_context(current);

    let slides = slides!(...);

    let total = slides.len() as u32;
    let stats = move || SlideStatistics {current: current.get() + 1, total, };

    Effect::new(move || {
        let s = stats();
        spawn_local(async {update_slides(s).await.unwrap();});
    });

    view! {...}
}"#;

    view! {
      <Slide title=title notes=notes>
        <p>"Call function from frontend. It's an API call!"</p>
                <Code code=code/>
      </Slide>
    }
}
