use std::collections::HashMap;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_animate::animate;
use leptos_animate::animations::classes::{In, Out};
use leptos_animate::animations::flip::Flip;
use leptos_hotkeys::use_hotkeys;

use crate::components::ProgressBar;
use crate::server_fns::{update_slides, ANSWERS_SIGNAL};
use crate::server_fns::{AnsweredQuestion, SlideStatistics};
use crate::slides::a_intro::*;
use crate::slides::b_leptos::*;
use crate::slides::c_ecosystem::*;
use crate::slides::d_final::*;

mod a_intro;
mod b_leptos;
mod c_ecosystem;
mod d_final;

#[component]
pub fn SlidesPage() -> impl IntoView {
    // We need to init the channels we are going to read from
    let default: HashMap<String, AnsweredQuestion> = HashMap::new();
    let _ = leptos_ws::ReadOnlySignal::new(ANSWERS_SIGNAL, default).unwrap();

    view! { <SlideDeck /> }
}

#[component]
fn Slide(
    title: &'static str,
    #[prop(default = "")] notes: &'static str,
    children: Children,
) -> impl IntoView {
    let step = RwSignal::new(0_u8);
    provide_context(step);

    use_hotkeys!(("arrowdown,j") => move |_| {
        step.update(|s| *s += 1)
    });

    use_hotkeys!(("arrowup,k") => move |_| {
        step.update(|s| if *s > 0 {*s -= 1})
    });

    view! {
        <section
            class="md:aspect-video w-full p-8 flex flex-col gap-4 prose prose-slate md:prose-xl lg:prose-2xl max-w-none"

            use:animate=(
                In::default().source("opacity-0").active("duration-150").target("opacity-100"),
                Out::default().source("opacity-100").active("duration-150").target("opacity-0"),
            )
        >
            <h1>{title}</h1>
            <div class="text-base">{children()}</div>
        </section>
    }
}

#[component]
fn Appear(#[prop(into, default = 0)] id: u8, children: Children) -> impl IntoView {
    let step: RwSignal<u8> = expect_context();
    let show = move || {
        if step.get() >= id {
            "transition-all duration-300 overflow-hidden opacity-100 translate-y-0 scale-y-100 origin-top max-h-[1000px]"
        } else {
            "transition-[max-height,opacity] duration-300 overflow-hidden opacity-0 translate-y-2 scale-y-0 origin-top max-h-0"
        }
    };
    use leptos::html::*;
    div().class(show).child(children())
}
#[component]
fn Replace(#[prop(into, default = 0)] id: u8, children: ChildrenFn) -> impl IntoView {
    let step: RwSignal<u8> = expect_context();
    view! { <Show when=move || step.get() == id>{children()}</Show> }
}

#[allow(unused_assignments)]
#[macro_export]
macro_rules! slides {
    ($current:expr, $( $comp:ident ),+ $(,)?) => {{
        use leptos::prelude::*;

        let c = $current;
        #[allow(unused_mut, unused, unused_assignments)]
        let mut idx: u32 = 0;
        let mut out: Vec<AnyView> = Vec::new();

        $(
         let this_idx = idx;
            out.push(
                view! {
                    <Show when=move || c.get() == this_idx>
                        <$comp/>
                    </Show>
                }
                .into_any()
            );
            idx += 1;
        )*

        out
    }};
}

#[component]
fn SlideDeck() -> impl IntoView {
    let current = RwSignal::new(0_u32);
    provide_context(current);

    let slides = slides!(
        current,
        Initial,
        AboutMe,
        AboutYou,
        WhatIsAbout,
        MpaSpa,
        // 2Leptos
        UiComponent,
        ViewMacro,
        Reactivity,
        Contexts,
        Stores,
        Server,
        Server2,
        // 3. Ecosystem
        EcosystemSection,
        LeptosUse,
        LeptosWebsockets,
        Extra,
        // Final
        FinalSection,
        GoodAndBad,
        AiBots,
        Overall,
        QuestionsAndCredits,
        Resources,
    );

    let total = slides.len() as u32;
    let stats = move || SlideStatistics {
        current: current.get() + 1,
        total,
    };

    Effect::new(move || {
        let s = stats();
        spawn_local(async {
            update_slides(s).await.unwrap();
        });
    });

    let next_slide = move || {
        current.update(|i| {
            if *i + 1 < total {
                *i += 1;
            }
        })
    };
    let prev_slide = move || {
        current.update(|i| {
            if *i > 0 {
                *i -= 1;
            }
        });
    };
    use_hotkeys!(("arrowleft,h") => move |_| {
        prev_slide();
    });

    use_hotkeys!(("arrowright,l") => move |_| {
        next_slide();
    });

    view! {
        <div class="min-h-screen max-h-screen flex flex-col">
            <header class="">
                <ProgressBar stats=stats />
            </header>

            <main class="flex-1 flex items-center justify-center relative overflow-x-hidden md:overflow-hidden">
                <div class="w-full max-w-5xl" use:animate=Flip::watch(current)>
                    {slides}
                </div>
            </main>

            <footer class="border-t border-slate-800">
                <div class="mx-auto max-w-5xl px-4 py-3 flex items-center justify-between text-sm items-stretch text-slate-600">
                    <span class="font-semibold ">"Leptos full stack - Rust BCN"</span>
                    <span class="">
                        "2025 - "<a target="_blank" href="https://8vi.cat">
                            martsec - 8vi.cat
                        </a>
                    </span>
                    <span class="font-mono text-xs">
                        {move || format!("{}/{}", current.get() + 1, total)}
                    </span>
                </div>
            </footer>
        </div>
    }
}
