use crate::components::BkgImg;
use crate::{
    components::QuestionBlock,
    server_fns::Question,
    slides::{Appear, Slide},
};
use leptos::prelude::*;
use singlestage::{alert::*, icon};

#[component]
pub fn EcosystemSection() -> impl IntoView {
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
                            Extra utility crates
                        </span>
                    </h2>
                </div>
            </div>
        </Slide>
        <BkgImg img="Archiv.org" alt="" />
    }
}

#[component]
pub fn LeptosUse() -> impl IntoView {
    view! {
        <Slide title="Leptos Use">
            <p>Essential web utilitites</p>
            <ul class="list-disc pl-5 space-y-1 grid md:grid-cols-2">
                <Appear id=1>
                    <li>"Local/session storage"</li>
                </Appear>
                <Appear id=2>
                    <li>"Cookies"</li>
                </Appear>
                <Appear id=3>
                    <li>"Window focus/visibility"</li>
                </Appear>
                <Appear id=4>
                    <li>"Browser permissions"</li>
                </Appear>
                <Appear id=5>
                    <li>"Service workers"</li>
                </Appear>
                <Appear id=6>
                    <li>"Notifications"</li>
                </Appear>
                <Appear id=7>
                    <li>"Screen/camera share"</li>
                </Appear>
                <Appear id=7>
                    <li>"..."</li>
                </Appear>
            </ul>

            <p>
                <a target="_blank" href="https://leptos-use.rs/">
                    "Leptos-use book"
                </a>
            </p>
        </Slide>
    }
}

#[component]
pub fn LeptosUseExample() -> impl IntoView {
    view! {
        <Slide title="Example: animate whith scroll">
            <p>
                <a target="_blank" href="https://plai.cards/hero">
                    "PLAI's hero cards"
                </a>
            </p>
            <p>
                <a target="_blank" href="https://plai.cards/hero">
                    "PLAI's random text"
                </a>
            </p>
        </Slide>
    }
}

#[component]
pub fn LeptosWebsockets() -> impl IntoView {
    view! {
        <Slide title="Leptos Websockets">
            <Alert class="not-prose">
                {icon!(icondata::LuCircleCheck)} <AlertTitle>"You've been using it"</AlertTitle>
                <AlertDescription>
                    "It has been the magic behind the top bar and the polls!"
                </AlertDescription>
            </Alert>

            <QuestionBlock q=Question {
                text: "Let's demo it with a poll. Go to your phones to choose your next front-end language!"
                    .into(),
                answers: vec![
                    "Rust".to_string(),
                    "Javascript".to_string(),
                    "Typescript".to_string(),
                    ".NET".to_string(),
                    "ruby".to_string(),
                ],
            } />

        </Slide>
    }
}

#[component]
pub fn Extra() -> impl IntoView {
    let title = "And More";
    let notes = r#""#;
    view! {
        <Slide title=title notes=notes>
            <p>scss and TailwindCSS integrated</p>
            <p>
                Component libraries like <a target="_blank" href="https://singlestage.doordesk.net">
                    Singlestage UI
                </a>or <a target="_blank" href="https://thawui.vercel.app/">
                    Thaw UI
                </a>
            </p>
            <p>Hotkeys for keyboard shortcuts</p>
            <p>Translations with <code>leptos_i18n</code>or <code>"leptos-fluent"</code></p>
            <p></p>
        </Slide>

        <BkgImg img="MachineLearningOperations" alt="" />
    }
}
