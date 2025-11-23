use crate::slides::{Appear, Slide};
use leptos::prelude::*;
use singlestage::{alert::*, icon};

#[component]
pub fn LeptosUse() -> impl IntoView {
    view! {
        <Slide title="Leptos Use">
        <p>Essential web utilitites</p>
            <ul class="list-disc pl-5 space-y-1 grid md:grid-cols-2">
        <Appear id=1><li>"Local/session storage"</li></Appear>
        <Appear id=2><li>"Cookies"</li></Appear>
        <Appear id=3><li>"Window focus/visibility"</li></Appear>
        <Appear id=4><li>"Browser permissions"</li></Appear>
        <Appear id=5><li>"Service workers"</li></Appear>
        <Appear id=6><li>"Notifications"</li></Appear>
        <Appear id=7><li>"Screen/camera share"</li></Appear>
        <Appear id=7><li>"..."</li></Appear>
            </ul>

        <p><a target="_blank" href="https://leptos-use.rs/">"Leptos-use book"</a></p>
        </Slide>
    }
}

#[component]
pub fn LeptosUseExample() -> impl IntoView {
    view! {
        <Slide title="Example: animate whith scroll">
        <p><a target="_blank" href="https://plai.cards/hero">"PLAI's hero cards"</a></p>
        <p><a target="_blank" href="https://plai.cards/hero">"PLAI's random text"</a></p>
        </Slide>
    }
}

#[component]
pub fn LeptosWebsockets() -> impl IntoView {
    view! {
        <Slide title="Leptos Websockets">
            // TODO:
        //https://github.com/TimTom2016/leptos_ws
        <p>TODO</p>

                    <Alert class="not-prose">
                        {icon!(icondata::LuCircleCheck)}
                        <AlertTitle>"You've been using it"</AlertTitle>
                        <AlertDescription>"It has been the magic behind the top bar and the polls!"</AlertDescription>
                    </Alert>
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
        <p>Component libraries like <a target="_blank" href="https://singlestage.doordesk.net">Singlestage UI</a> or <a target="_blank" href="https://thawui.vercel.app/">Thaw UI</a></p>
        <p>Hotkeys for keyboard shortcuts</p>
        <p>Translations with <code>leptos_i18n</code> or <code>"leptos-fluent"</code></p>
        <p></p>
      </Slide>
    }
}
