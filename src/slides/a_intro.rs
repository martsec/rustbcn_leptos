use leptos::prelude::*;
use leptos::task::spawn_local;
use singlestage::*;
use singlestage::{alert::*, icon};

use crate::components::QuestionBlock;
use crate::server_fns::{send_question, Question};
use crate::slides::{Appear, Replace, Slide};

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
                    <li>
                        "Professor Big data infra + distributed systems at BCN technology School"
                    </li>
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

            <Replace id=1>
                <QuestionBlock q=Question {
                    text: "What is your main area of expertise?".into(),
                    answers: vec![
                        "Backend".to_string(),
                        "Frontend".to_string(),
                        "Full stack".to_string(),
                        "Data".to_string(),
                        "Other".to_string(),
                        "None".to_string(),
                    ],
                } />
            </Replace>
            <Replace id=2>
                <QuestionBlock q=Question {
                    text: "Have you ever built a website?".into(),
                    answers: vec!["Yes".to_string(), "No".to_string()],
                } />
            </Replace>

        // - How much experience do you have in rust?
        // - And building APIs?
        // - And with frontend?
        // - And Full stack?
        </Slide>
    }
}

#[component]
pub fn WhatIsAbout() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Slide title="What are we going to see?">
            <Button size="large" variant="normal" on:click=on_click>
                "Click Me: "
                {count}
            </Button>
            <div class="my-10 max-w-200">
                <Show when=move || { count.get() >= 5 && count.get() < 10 }>
                    <Alert class="not-prose">
                        {icon!(icondata::LuCircleCheck)}
                        <AlertTitle>"Success! You have tested Reactivity"</AlertTitle>
                        <AlertDescription>"You can stop clicking now"</AlertDescription>
                    </Alert>
                </Show>
                <Show when=move || { count.get() >= 10 }>

                    <Alert variant="destructive" class="not-prose">
                        {icon!(icondata::FiAlertCircle)}
                        <AlertTitle>"That is enough!"</AlertTitle>
                        <AlertDescription>"Nothing else will happen. Trust me"</AlertDescription>
                    </Alert>
                </Show>
            </div>
        </Slide>
    }
}
