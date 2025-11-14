use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_animate::animate;
use leptos_animate::animations::classes::{In, Out};
use leptos_animate::animations::flip::Flip;
use leptos_hotkeys::use_hotkeys;

use crate::components::ProgressBar;
use crate::server_fns::update_slides;
use crate::server_fns::SlideStatistics;

#[component]
pub fn SlidesPage() -> impl IntoView {
    view! { <SlideDeck /> }
}

#[component]
pub fn Slide(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="aspect-video w-full  p-8 flex flex-col gap-4">

            // use:animate=(In::default()
            // .source("opacity-0")
            // .active("duration-1500")
            // .target("opacity-100"),
            // Out::default()
            // .source("opacity-100")
            // .active("duration-1500")
            // .target("opacity-0")
            // )
            <h2 class="text-2xl font-semibold tracking-tight">{title}</h2>
            <div class="text-base">{children()}</div>
        </section>
    }
}

#[component]
pub fn SlideDeck() -> impl IntoView {
    let current = RwSignal::new(0_u32);

    let slides: Vec<AnyView> = {
        let c = current;

        vec![
            view! {
                <Show when=move || c.get() == 0>
                    <Slide title="Intro">
                        <p>"What PLAI is about."</p>
                    </Slide>
                </Show>
            }
            .into_any(),
            view! {
                <Show when=move || c.get() == 1>
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
                <Show when=move || c.get() == 2>
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

    use_hotkeys!(("arrowdown,arrowright,l,j") => move |_| {
        next_slide();
    });

    view! {
        <div class="min-h-screen max-h-screen flex flex-col">
            <header class="">
                <ProgressBar stats=stats />
            </header>

            <main class="flex-1 flex items-center justify-center">
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
                    <span class="">"2025 - "<a href="https://8vi.cat">martsec - 8vi.cat</a></span>
                    <span class="font-mono text-xs">
                        {move || format!("{}/{}", current.get() + 1, total)}
                    </span>
                </div>
            </footer>
        </div>
    }
}
