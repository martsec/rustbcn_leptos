use leptos::prelude::*;
use singlestage::icon;
use singlestage::*;

use crate::components::{BkgImg, Mermaid, QuestionBlock};
use crate::server_fns::{Question, PARTICIPANTS_SIGNAL};
use crate::slides::{Appear, Replace, Slide};

#[component]
pub fn Initial() -> impl IntoView {
    view! {
        <Slide title="">

            <h1 class="not-prose text-4xl md:text-9xl font-extrabold tracking-tight text-slate-900">
                Leptos:
                <span class="block text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
                    Rust Full-Stack
                </span>
            </h1>

            <p class="text-slate-400 italic">
                "Pray to the demo gods because this is actually a demo"
            </p>
        </Slide>
        <BkgImg img="Hype" alt="" />
    }
}
#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <Slide title="About me">
            <ul class="list-disc pl-5 space-y-1">
                <Appear id=1>
                    <li>"Data engineer doing graph algorithms and bitemporality"</li>
                </Appear>

                <Appear id=2>
                    <li>
                        "Professor Big data infra + distributed systems at BCN technology School"
                    </li>
                </Appear>
                <Appear id=3>
                    <li>"Board game author (I brought stickers!!)"</li>

                </Appear>
            </ul>

            <Appear id=4>
                <div class="grid md:grid-cols-2 gap-6">
                    <div>
                        <h4>"I love"</h4>
                        <p>"Data analysis"</p>
                        <p>"Trying new libraries"</p>
                        <p>"Good old open source software"</p>
                    </div>
                    <div>
                        <Appear id=5>
                            <h4>"I hate"</h4>
                            <p>"Fake open source"</p>
                        </Appear>
                        <Appear id=6>
                            <p class="text-red-800 font-bold">"Javascript & environment"</p>

                        </Appear>
                    </div>
                </div>
            </Appear>
        </Slide>
        <BkgImg img="Privacy" alt="" />
    }
}

#[component]
pub fn AboutYou() -> impl IntoView {
    view! {
        <Slide title="About you">
            <p>
                "Let's try something interactive, go to "
                <a href="https://leptos.8vi.cat">"leptos.8vi.cat"</a>
            </p>
            <Replace id=0>
                <NameList />
            </Replace>

            <Replace id=1>
                <QuestionBlock q=Question {
                    text: "What is your main area of expertise?".into(),
                    answers: vec![
                        "Backend".to_string(),
                        "Frontend".to_string(),
                        "Full stack".to_string(),
                        "Data".to_string(),
                        "Other".to_string(),
                        "None".to_string(),
                    ],
                } />
            </Replace>
            <Replace id=2>
                <QuestionBlock q=Question {
                    text: "Have you ever built a website?".into(),
                    answers: vec!["Yes".to_string(), "No".to_string()],
                } />
            </Replace>
            <Replace id=23>
                <QuestionBlock q=Question {
                    text: "And an API?".into(),
                    answers: vec!["Yes".to_string(), "No".to_string()],
                } />
            </Replace>

        // - How much experience do you have in rust?
        // - And building APIs?
        // - And with frontend?
        // - And Full stack?
        </Slide>
        <BkgImg img="10XEngineer" alt="it's you, a 10x engineer" />
    }
}

#[component]
pub fn NameList() -> impl IntoView {
    let default_users: Vec<String> = vec![];
    let ws = leptos_ws::ReadOnlySignal::new(PARTICIPANTS_SIGNAL, default_users).unwrap();

    view! {
        <img src="/qr.png" class="h-60 w-60" />

        <p>
            {move || {
                ws.get()
                    .into_iter()
                    .map(|n| {
                        view! {
                            " "
                            <Kbd>{n}</Kbd>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </p>
    }
}

#[component]
pub fn WhatIsAbout() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let theme_context = expect_context::<ThemeProviderContext>();
    Effect::new(move || {
        if count.get() > 15 {
            theme_context.mode.set(
                match count.get() % 2 {
                    0 => "light",
                    _ => "dark",
                }
                .into(),
            );
        }
    });
    view! {
        <BkgImg img="Surveillance" alt="" />
        <Slide title="What are we going to see?">
            <Appear id=1>
                <Button size="large" variant="normal" on:click=on_click>
                    "Click Me: "
                    {count}
                </Button>
                <div class="my-10 mx-auto max-w-100">
                    <Show when=move || { count.get() >= 5 && count.get() < 10 }>
                        <Alert class="not-prose">
                            {icon!(icondata::LuCircleCheck)}
                            <AlertTitle>"Success! You have tested Reactivity"</AlertTitle>
                            <AlertDescription>"You can stop clicking now"</AlertDescription>
                        </Alert>
                    </Show>
                    <Show when=move || { count.get() >= 10 }>

                        <Alert variant="destructive" class="not-prose">
                            {icon!(icondata::FiAlertCircle)}
                            <AlertTitle>"That is enough!"</AlertTitle>
                            <AlertDescription>
                                "Nothing else will happen. Trust me"
                            </AlertDescription>
                        </Alert>
                    </Show>

                </div>
            </Appear>
            <Appear id=2>
                <QuestionBlock q=Question {
                    text: "Impessive, right?".into(),
                    answers: vec![
                        "Yeah!".to_string(),
                        "Ecstatic".to_string(),
                        "Exhilarant".to_string(),
                    ],
                } />
                <Alert class="not-prose max-w-100">
                    {icon!(icondata::ImEye)} <AlertTitle>"Interactive web apps"</AlertTitle>
                    <AlertDescription>"With HTML sent from the server"</AlertDescription>
                </Alert>
            </Appear>
        </Slide>
    }
}

#[component]
pub fn Alternatives() -> impl IntoView {
    let title = "What options are there?";
    let notes = r#""#;
    view! {
        <Slide title=title notes=notes>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 not-prose my-auto">
                <div class="p-6 rounded-xl bg-red-50 border border-red-200">
                    <h3 class="text-xl font-bold text-red-700 mb-3">JavaScript</h3>
                    <ul class="space-y-1 text-red-900">
                        <li>React</li>
                        <li>Vue</li>
                        <li>Angular</li>
                        <li>Svelte</li>
                        <li>Solid</li>
                    </ul>
                </div>
                <div class="p-6 rounded-xl bg-green-50 border border-green-200">
                    <h3 class="text-xl font-bold text-green-700 mb-3">Rust</h3>
                    <ul class="space-y-1 text-green-900">
                        <li>Yew</li>
                        <li>Leptos</li>
                        <li>Dioxus</li>
                        <li>Sycamore</li>
                    </ul>
                </div>

            </div>

        </Slide>
    }
}

#[component]
pub fn MpaSpa() -> impl IntoView {
    let _images = [
        "/1-mpa.png",
        "/2-mpa-jquery.png",
        "/3-spa.png",
        "/4-spa-server.png",
        "/5-wasm.png",
    ];

    let diagrams = [
        r#"
sequenceDiagram
    title Multi Page App
    participant Client
    participant Server

    Client->>Server: URL, link or form
    Server->>Client: HTML, js, css
"#,
        r#"
sequenceDiagram
    title Multi Page App + jQuery
    participant Client
    participant JS
    participant Server

    Client->>Server: URL, link or form
    Server->>Client: HTML, JS, css
    Client->>JS: User events
    JS->>Client: DOM updates (animations, UI effects)
"#,
        r#"
sequenceDiagram
    title Single Page App
    participant Client
    participant JS
    participant Server

    Server-->>Client: html, JS, css
    Client->>JS: User events
    JS->>Server: JSON
    Server->>JS:
    JS->>Client: DOM updates 
"#,
        r#"
sequenceDiagram
    title Server Side Rendering + JS on server
    participant Client
    participant JS
    participant Server as JAVASCRIPT Server

    Server-->>Client: html, JS, css
    Client->>JS: User events
    JS->>Server: JSON
    Server->>JS:
    JS->>Client: DOM updates 
"#,
        r#"
sequenceDiagram
    title Web Assembly 
    participant Client
    participant JS as WASM
    participant Server

    Server-->>Client: html, js, css, WASM
    Client->>JS: User events
    JS->>Server: JSON/serialized
    Server->>JS:
    JS->>Client: DOM updates
"#,
    ];

    view! {
        <Slide title="MPA to SPA">
            <div class="h-[50vh] flex items-center justify-center">
                // images
                {diagrams
                    .into_iter()
                    .enumerate()
                    .map(|(id, src)| {
                        view! {
                            <Replace id=id as u8>
                                // <img
                                // class="max-h-full w-auto object-contain"
                                // src=src
                                // />
                                <Mermaid code=src class="grow" />
                            </Replace>
                        }
                    })
                    .collect_view()}
            </div>
        </Slide>
    }
}

#[component]
pub fn Wasm() -> impl IntoView {
    let title = "WebAssembly (Wasm)";
    let notes = r#"

    Mozilla test from 2017 where they showed that it could be an order of magniture faster https://hacks.mozilla.org/2017/03/why-webassembly-is-faster-than-asm-js/
    
    

Use it for 
    "#;
    view! {
        <Slide title=title notes=notes>
            <p>
                Low-level binary format that runs in the browser at
                near-native speed.
            </p>
            <ul class="list-disc pl-5 space-y-1">
                <li>Web standard</li>
                <li>
                    Sandboxed
                    <a target="_blank" href="https://github.com/krausest/js-framework-benchmark">
                        native-like execution
                    </a>.
                </li>
                <li>C, C++, Rust, Go, Python, JVM, Julia, Ruby...</li>
                <li>Portable compilation target.</li>
                <li>Can call JS and browser APIs.</li>
                <li>Parsed faster than JS.</li>
                <li>
                    Check <a target="_blank" href="https://bevy.org/examples/">
                        Bevy, a rust game engine
                    </a>that compiles to wasm.
                </li>
            </ul>

        </Slide>
    }
}
