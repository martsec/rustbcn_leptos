use leptos::prelude::*;

use crate::components::BkgImg;
use crate::slides::{Appear, Slide};

#[component]
pub fn LeptosMobile() -> impl IntoView {
    view! {
        <Slide title="Beyond web">
            // TODO:
            <p>Tauri</p>

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
pub fn Overall() -> impl IntoView {
    view! {
        <Slide title="Summary">
            <p>
                "Leptos feels old style open source: created for the love of providing better software"
            </p>

            <ul class="list-disc pl-5 space-y-1">
                <Appear id=1>
                    <li>"No company behind it, though gbj delivers consistently!"</li>
                </Appear>
                <Appear id=2>
                    <li>"Rich, voluteer-led ecosystem (leptos_animate, "</li>
                </Appear>

                <Appear id=3>
                    <li>""</li>
                </Appear>
                <Appear id=1>
                    <li>"Tricky in some places"</li>
                </Appear>

            </ul>

        </Slide>
    }
}
