use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_animate::animate;
use leptos_animate::animations::classes::{In, Out};
use leptos_animate::animations::flip::Flip;
use leptos_hotkeys::use_hotkeys;

use crate::components::ProgressBar;
use crate::server_fns::update_slides;
use crate::server_fns::SlideStatistics;
use crate::slides::a_intro::{AboutMe, Initial};
use crate::slides::b_leptos::ViewMacro;
use crate::slides::d_final::AiBots;

mod a_intro;
mod b_leptos;
mod c_ecosystem;
mod d_final;

#[component]
pub fn SlidesPage() -> impl IntoView {
    view! { <SlideDeck /> }
}

#[component]
fn Slide(title: &'static str, children: Children) -> impl IntoView {
    let step = RwSignal::new(0_u8);
    provide_context(step);

    use_hotkeys!(("arrowdown,j") => move |_| {
        step.update(|s| *s += 1)
    });

    view! {

        <section class="md:aspect-video w-full p-8 flex flex-col gap-4 prose prose-slate md:prose-xl lg:prose-2xl max-w-none"

            use:animate=(In::default()
            .source("opacity-0")
            .active("duration-150")
            .target("opacity-100"),
            Out::default()
            .source("opacity-100")
            .active("duration-150")
            .target("opacity-0")
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
            "transition-all duration-300 opacity-100 translate-y-0"
        } else {
            "transition-all duration-300 opacity-0 translate-y-2"
        }
    };
    use leptos::html::*;
    div().class(show).child(children())
}

#[component]
fn SlideDeck() -> impl IntoView {
    let current = RwSignal::new(0_u32);
    provide_context(current);

    let slides: Vec<AnyView> = {
        let c = current;

        vec![
            view! {<Show when=move || c.get() == 0><Initial/></Show>}.into_any(),
            view! {<Show when=move || c.get() == 1><AboutMe/></Show>}.into_any(),
            view! {<Show when=move || c.get() == 2><ViewMacro/></Show>}.into_any(),
            view! {<Show when=move || c.get() == 3><AiBots/></Show>}.into_any(),
            view! {
                <Show when=move || c.get() == 4>
                    <Slide title="How it works">
                        <ul class="list-disc pl-5 space-y-1">
                            <li>"Draw cards"</li>
                            <li>"Explain tech concepts"</li>
                            <li>"Score with the team"</li>
                        </ul>

                    </Slide>
                </Show>
            }
            .into_any(),
            view! {
                <Show when=move || c.get() == 5>
                    <Slide title="Call to action">
                        <p>"Tell people how to get PLAI or join the community."</p>
                    </Slide>
                </Show>
            }
            .into_any(),
        ]
    };

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

    let slides_view = slides.into_iter().collect_view();

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
    use_hotkeys!(("arrowup,arrowleft,h,k") => move |_| {
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
                <div
                    class="w-full max-w-5xl"
                    use:animate=Flip::watch(current)
                >
                    {slides_view}
                </div>
            </main>

            <footer class="border-t border-slate-800">
                <div class="mx-auto max-w-5xl px-4 py-3 flex items-center justify-between text-sm items-stretch text-slate-600">
                    <span class="font-semibold ">"Leptos full stack - Rust BCN"</span>
                    <span class="">"2025 - "<a target="_blank" href="https://8vi.cat">martsec - 8vi.cat</a></span>
                    <span class="font-mono text-xs">
                        {move || format!("{}/{}", current.get() + 1, total)}
                    </span>
                </div>
            </footer>
        </div>
    }
}
