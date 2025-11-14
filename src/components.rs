use crate::server_fns::SlideStatistics;
use leptos::prelude::*;

#[component]
pub fn ProgressBar(stats: impl Fn() -> SlideStatistics + Send + 'static) -> impl IntoView {
    let progress_style = move || {
        if stats().total == 0 {
            "width: 0%;".to_string()
        } else {
            let p = (stats().current + 1) as f32 / stats().total as f32;
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
