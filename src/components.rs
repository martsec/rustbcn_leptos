use crate::server_fns::SlideStatistics;
use leptos::prelude::*;

#[component]
pub fn ProgressBar(stats: impl Fn() -> SlideStatistics + Send + 'static) -> impl IntoView {
    let progress_style = move || {
        if stats().total == 0 {
            "width: 0%;".to_string()
        } else {
            let p = (stats().current) as f32 / stats().total as f32;
            format!("width: {:.0}%;", p * 100.0)
        }
    };

    view! {
        <div class="w-full ">
            <div class="mx-auto">
                <div class="w-full h-[5px] rounded-full overflow-hidden">
                    <div
                        class="h-full bg-red-800 transition-all duration-200"
                        style=progress_style
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BkgImg(
    #[prop(into)] img: String,
    #[prop(into, default = "Image part of the board game PLAI".into())] alt: String,
) -> impl IntoView {
    view! {
            <img
                src=format!("/{}.png", img)
                alt=alt
                class="pointer-events-none select-none absolute left-[25%] top-1/8 md:top-5/8 -translate-y-1/2  opacity-20"
                style="transform: rotate(25deg);"
            />

    }
}
