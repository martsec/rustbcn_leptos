use leptos::prelude::*;

use crate::slides::{Appear, Slide};

#[component]
pub fn Initial() -> impl IntoView {
    view! {
            <Slide title="Intro">
            <Appear id=1>
                <p>"What PLAI is about."</p>
        </Appear>

            <Appear id=2>
                <p>"Pray the demo gods"</p>
        </Appear>
            <Appear id=3>
                <p>"Because this is actually a demo"</p>
        </Appear>
            </Slide>
    }
}
#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
            <Slide title="About me">
        <ul class="list-disc pl-5 space-y-1">
            <Appear id=1>
                <li>"Data engineer doing graph algorithms"</li>
        </Appear>

            <Appear id=2>
                <li>"Professor Big data infra + distributed systems at BCN technology School"</li>
        </Appear>
        <Appear id=3>
        <li>"Board game author (I brought stickers!!)"</li>

        </Appear>
        </ul>

        <Appear id=4>
        <div class="grid md:grid-cols-2 gap-6">
            <div>
                <h4>"I love"</h4>
            <p>"Good old open source software"</p>
            <p>"Trying new things"</p>
            </div>
            <div>
            <h4>"I hate"</h4>
                <p>"Fake open source"</p>
            <Appear id=5>
                <p>"Javascript & environment"</p>
            </Appear>
            </div>
        </div>
        </Appear>
            </Slide>
    }
}

#[component]
pub fn AboutYou() -> impl IntoView {
    view! {
            <Slide title="About you">
        <p>"Let's try something interactive"</p>

        //TODO: add a live question and answer
        //- How much experience do you have in rust?
        //- And building APIs?
        //- And with frontend?
        //- And Full stack?

            </Slide>
    }
}
