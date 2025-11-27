use leptos::prelude::*;

use crate::components::BkgImg;
use crate::slides::{Appear, Slide};

#[component]
pub fn FinalSection() -> impl IntoView {
    let title = "";
    let notes = r#""#;
    view! {
        <Slide title=title notes=notes>
            <div class="not-prose flex items-center gap-4">
                <div class="h-px flex-1 bg-gradient-to-r from-orange-500/60 to-pink-500/60"></div>

                <div class="text-right">
                    <p class="text-xs font-semibold tracking-[0.25em] uppercase text-slate-400">
                        Leptos full stack
                    </p>
                    <h2 class="text-2xl md:text-7xl font-bold tracking-tight">
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
                            Final
                        </span>
                    </h2>
                </div>
            </div>
        </Slide>
        <BkgImg img="BoardChaos" alt="" />
    }
}

#[component]
pub fn CsrExample() -> impl IntoView {
    let title = "Client Side Render Example";
    let notes = r#"How do we tie everything together?"#;
    view! {
        <Slide title=title notes=notes>
            <h5 class="text-center">
                <a href="https://cata.wine" target="_blank">
                    "🍷 cata.wine"
                </a>
            </h5>
            <Appear id=1>
                <ul class="list-disc pl-5 space-y-1 md:grid md:grid-cols-2">
                    <li>"Persisted state"</li>
                    <li>"Local storage"</li>
                    <li>"Export/import"</li>
                    <li>"Different languages"</li>
                    <li>"Conditional components"</li>
                    <li>"Responsive + mobile webapp"</li>
                    <li>"Future: iOS/Andriod w/ Tauri"</li>
                </ul>

            </Appear>
            <BkgImg img="TheCEO" alt="" />
        </Slide>
    }
}

#[component]
pub fn SsrExample() -> impl IntoView {
    let title = "Server Side Request Example";
    let notes = r#"How do we tie everything together?"#;
    view! {
        <BkgImg img="Ethics" alt="" />
        <Slide title=title notes=notes>
            <h5 class="text-center">
                <a href="https://plai.cards" target="_blank">
                    "🃏 plai.cards"
                </a>
            </h5>
            <Appear id=1>
                <ul class="list-disc pl-5 space-y-1 md:grid md:grid-cols-2">
                    <li>"Different languages"</li>
                    <li>"Backoffice"</li>
                    <li>"Sqlite-like DB"</li>
                    <li>"Auth"</li>
                    <li>"PoW protection"</li>
                    <li>"Spreadsheet export"</li>

                </ul>

            </Appear>
        </Slide>
    }
}

#[component]
pub fn AiBots() -> impl IntoView {
    view! {
        <Slide title="What about LLMs creating code?">

            <h2>"Tried but..."</h2>
            <p>"I haven't had a lot of luck with working code"</p>
            <p>"It provides an ok approach but lacking correctness"</p>
            <p>"Not enough training data + can't adapt to library changes"</p>
            <BkgImg
                img="AIBotArmy"
                alt="an army of bots, part of the art for the board game plai"
            />

        </Slide>
    }
}

#[component]
pub fn GoodAndBad() -> impl IntoView {
    let title = "Good, meh, bad";
    let notes = r#""#;
    view! {
        <BkgImg img="Bias" alt="" />
        <Slide title=title notes=notes>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8 not-prose my-auto">

                <div class="p-6 rounded-xl bg-green-50 border border-green-200">
                    <h3 class="text-xl font-bold text-green-700 mb-3">Good</h3>
                    <ul class="space-y-1 text-green-900">
                        <li>Actively developed</li>
                        <li>Typed everything</li>
                        <li>"Same interfaces & classes"</li>
                        <li>"Your trusted Rust ecosystem (cargo, crates, bacon...)"</li>
                        <li>Integration with JS libraries</li>
                    </ul>
                </div>

                <div class="p-6 rounded-xl bg-yellow-50 border border-yellow-200">
                    <h3 class="text-xl font-bold text-yellow-700 mb-3">Meh</h3>
                    <ul class="space-y-1 text-yellow-900">
                        <li>Compilation times</li>
                        <li>Big-ish WASM bundle</li>
                        <li>Occasional lib patching due to wasm-bindgen versions</li>
                    </ul>
                </div>

                <div class="p-6 rounded-xl bg-red-50 border border-red-200">
                    <h3 class="text-xl font-bold text-red-700 mb-3">Ugly</h3>
                    <ul class="space-y-1 text-red-900">
                        <li>No std library</li>
                        <li>"No native time, use "<code>chrono</code></li>
                        <li>WASM sizes penalize SEO</li>
                    </ul>
                </div>

            </div>

        </Slide>
    }
}

#[component]
pub fn Overall() -> impl IntoView {
    view! {
        <BkgImg img="TermsAndConditions" alt="" />
        <Slide title="Summary">
            <p>
                "Leptos feels old style open source: created for the love of providing omething useful"
            </p>

            <ul class="list-disc pl-5 space-y-1">
                <Appear id=1>
                    <li>"No company behind it, though gbj delivers consistently!"</li>
                </Appear>
                <Appear id=2>
                    <li>"Rich, voluteer-led ecosystem"</li>
                </Appear>

                <Appear id=3>
                    <li>"Evolving (e.g. wasm splitting)"</li>
                </Appear>
                <Appear id=1>
                    <li>"Tricky in some places (e.g. copy everything)"</li>
                </Appear>

            </ul>

        </Slide>
    }
}

#[component]
pub fn QuestionsAndCredits() -> impl IntoView {
    view! {
        <BkgImg img="PressCoverage" alt="" />
        <Slide title="Questions?">

            <p>"Credits and thanks"</p>
            <ul class="list-disc pl-5 space-y-1">

                <li>
                    <a href="https://github.com/gbj" target="_blank">
                        "Greg Johnston"
                    </a>
                    " for creating "
                    <a href="https://www.leptos.dev/" target="_blank">
                        leptos
                    </a>
                </li>
                <li>
                    "TimTom for "<a href="https://github.com/TimTom2016/leptos_ws" target="_blank">
                        "leptos_ws"
                    </a>
                </li>
                <li>
                    "gaucho-labs for "
                    <a href="https://github.com/gaucho-labs/leptos-hotkeys" target="_blank">
                        "leptos-hotkeys"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://singlestage.doordesk.net/">
                        "Singlestage UI"
                    </a>
                </li>
                <li>
                    "Background images by "<a href="https://plai.cards" target="_blank">
                        PLAI
                    </a>
                </li>
            </ul>

        </Slide>
    }
}

#[component]
pub fn Resources() -> impl IntoView {
    let title = "Resources";
    let notes = r#""#;
    view! {
        <Slide title=title notes=notes>
            <div />
            <ul class="list-disc pl-5 space-y-1">
                <li>
                    <a target="_blank" href="https://book.leptos.dev/">
                        "Leptos Book"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://github.com/leptos-rs/awesome-leptos">
                        "Awesome Leptos"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://www.youtube.com/@leptos-dev">
                        "Leptos Youtube channel"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://www.youtube.com/watch?v=V1cqQRmVAK0">
                        "The Future of Rust Web Applications - Greg Johnston"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://www.youtube.com/watch?v=4KtotxNAwME">
                        "The Truth about Rust/WebAssembly Performance"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://krausest.github.io/js-framework-benchmark/">
                        "js web frameworks benchmark"
                    </a>
                </li>
                <li>
                    <a target="_blank" href="https://www.youtube.com/watch?v=vAjle3c9Xqc">
                        "Build A Full Stack Chatbot in Rust (feat. Leptos & Rustformers)"
                    </a>
                </li>
            // <li><a target="_blank" href="">""</a></li>
            </ul>
        </Slide>

        <BkgImg img="DataScientist" alt="" />
    }
}
