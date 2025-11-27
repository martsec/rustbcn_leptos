use crate::{
    components::{Code, Mermaid},
    slides::{Appear, Replace, Slide},
};

use crate::components::BkgImg;
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
        <button on:click=move |_| *set_count.write() += 1 >
        "Click me: "{count}</button>
        <p>"Double count: "{move || count.get() * 2}</p>
    }
}"#;
    view! {
        <Slide title=title notes=notes>
            <p>
                Similar to HTML elements, views represent a section of the DOM. They are written in a JSX-like format.
            </p>
            <p>The body is a set-up function that runs once</p>
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
    let code_bad = r#"use crate::slides::{Appear, Slide};
use leptos::prelude::*;

#[component]
pub fn ViewMacro() -> impl IntoView {
    let code = "...";
    view! {
        <Slide title="Typecheck your HTML">
            <p>"Let's apply recursion on this page" // We made a mistake!!!
            <code>
                {code}
            </code>
        </Slide>
    }
}"#;
    let code_error = r#"error: wrong close tag found
  --> src/slides/b_leptos.rs:86:9
   |
86 |         </Slide>
   |         ^^^^^^^^
   |
help: open tag that should be closed; it's started here
  --> src/slides/b_leptos.rs:83:13
   |
83 |             <p>"Let's apply recursion on this page"
   |             ^^^"#;
    view! {
        <Slide title="Typecheck your HTML" notes=notes>
            <p>"Let's apply recursion on this page"</p>
            <Replace id=0>
                <Code code=code_bad />
            </Replace>
            <Replace id=1>
                <Code code=code_error />
            </Replace>
            <Replace id=2>
                <Code code=code />
            </Replace>
        </Slide>
    }
}

#[component]
pub fn NoMacroSyntax() -> impl IntoView {
    let title = "You don't like macros? Leptos got you";
    let notes = r#""#;
    let code = r#"use crate::slides::{Appear, Slide, SlideProps};
use leptos::prelude::*;
use leptos::html::*;

#[component]
pub fn ViewMacro() -> impl IntoView {
    let code = "...".to_string();

    Slide(SlideProps {
        title: "Typecheck your HTML".into(),
        children: Box::new(move || {
            let p = p().child("Let's apply recursion on this page");
            let code_el = code().child(code.clone());
            vec![p.into_view(), code_el.into_view()].into_view()
        }),
        ..Default::default()
    })
}"#;
    view! {
      <Slide title=title notes=notes>
                <Code code=code />
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
            <Button 
                size="large" variant="normal" 
                on:click=on_click 
                class:bg-red-500=move || count.get() % 2 == 1>
                "Click Me: "{count}
            </Button>
        </Slide>
    }
}"#;

    view! {
        <Slide title="Signals and Reactivity">

            <p>
                "Signals are the "
                <a
                    target="_blank"
                    href="https://book.leptos.dev/reactivity/working_with_signals.html"
                >
                    building block of reactivity
                </a>.<br />"Every time they change, everything that reads from it reevaluates."
            </p>

            <p>
                Special syntax for listeners <code>r#"on:click"#</code>and dynamic attributes
                <code>r#"class:red=move || count.get() % 2 == 1"#</code>
            </p>
            <Button
                size="large"
                variant="normal"
                on:click=on_click
                class:bg-red-500=move || count.get() % 2 == 1
            >
                "Click Me: "
                {count}
            </Button>
            <Appear id=1>
                <Code code=code />
            </Appear>
        </Slide>
    }
}

#[component]
pub fn Contexts() -> impl IntoView {
    let title = "Passing contexts";
    let notes = r#"Communication from parents to childrens and vice-versa.

    The child can send notificactions about state or events.
    "#;
    let diagram = r#"
flowchart LR
    App["App (root)
provide_context(Theme)"]

    Page["Page / Parent
provide_context(User)"]

    Child1["Child 1
expect_context(User)"]

    Child2["Child 2
expect_context(User)"]

    Grand["Grandchild
expect_context(Theme)"]

    %% component tree
    App --> Page
    Page --> Child1
    Page --> Child2
    Child2 --> Grand

    %% Theme context from root
    App -. "Theme ctx" .-> Page
    App -. "Theme ctx" .-> Child1
    App -. "Theme ctx" .-> Grand

    %% User context from Page
    Page -. "User ctx" .-> Child1
    Page -. "User ctx" .-> Child2
"#;
    view! {
        <Slide title=title notes=notes>

            <p>
                "Instead of passing signals to components, set them as context. "
                <code>"<Child2 user=user theme=theme/>"</code>
            </p>
            <Mermaid code=diagram class="max-h-150" />
            <Appear id=1>
                <code>provide_context(12_u8);</code>
                and
                <code>let n: u8 = expect_context();</code>
                <p>
                    <i>Bad</i>
                    thing: contexts are identified only by its type.
                    <i>Worst</i>
                    thing:
                    "Now it's"
                    a
                    <b>runtime check.</b>
                </p>
            </Appear>

        </Slide>
    }
}

#[component]
pub fn Stores() -> impl IntoView {
    let title = "Reactive Stores";
    let notes = r#""#;
    let definition = r#"#[derive(Store, Clone, ...)]
pub struct Sale {
    pub vt_begin: DateTime<Utc>,
    pub vt_end: DateTime<Utc>,
    pub id: u32,
    pub client: Client,
    pub num_units_sold: u8,
    pub unit_price_with_vat: f64,
    pub vat_type: VatType,
    pub payment: PaymentPlatform,
    pub shipping: Envio,
    pub shipping_with_vat: f64,
    pub shipping_number: String,
    pub notes: String,
    pub state: SaleState,
}
"#;
    let usage = r#"#[component]
pub fn SaleForm(
    on_save: impl Fn(Sale) + 'static + Copy,
) -> impl IntoView {
    let state = expect_context::<Store<SalesState>>();
    let submit_form = move |_| {on_save(sale.get());};
    view! {
        <ClientInfoSection client=sale.client() />
        <SimpleNumberInput
            label="Units sold"
            value=sale.num_units_sold() />
        <button type="button"
            on:click=submit_form
            disabled=move || sale.has_errors().get() >
            "Save sale"
        </button>
    }
"#;
    view! {
        <BkgImg img="InternationalDataTransfer" alt="international data transfer" />
        <Slide title=title notes=notes>
            <p>
                <code>struct</code>
                +
                <code>Signal</code>
                for complex or global state.
            </p>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 place-items-center">
                <Code code=definition />
                <Code code=usage />
            </div>
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
            <Appear id=1>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 place-items-center">
                    <p class="text-xl">Axum</p>
                    <p class="text-xl">Actix</p>
                </div>
            </Appear>
            <Appear id=3>"Serverside: It's just a function"</Appear>
            <Appear id=2>
                <Code code=code />
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
            <Code code=code />
        </Slide>
    }
}
