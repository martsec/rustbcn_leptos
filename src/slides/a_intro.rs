use leptos::prelude::*;
use singlestage::icon;
use singlestage::*;

use crate::components::{BkgImg, QuestionBlock};
use crate::server_fns::Question;
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
            <BkgImg
                img="10XEngineer"
                alt="it's you, a 10x engineer"
            />
    }
}

#[component]
pub fn WhatIsAbout() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let theme_context = expect_context::<ThemeProviderContext>();
    Effect::new(move || {
        if count.get() > 15 {
            theme_context.mode.set(
                match count.get() % 2 {
                    0 => "light",
                    _ => "dark",
                }
                .into(),
            );
        }
    });
    view! {
        <Slide title="What are we going to see?">
            <Appear id=1>
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
            </Appear>
            <Appear id=2>
                    <Alert class="not-prose">
                        {icon!(icondata::ImEye)}
                        <AlertTitle>"Interactive web apps"</AlertTitle>
                        <AlertDescription>"With HTML sent from the server"</AlertDescription>
                    </Alert>
            </Appear>
        </Slide>
    }
}

#[component]
pub fn MpaSpa() -> impl IntoView {
    let images = [
        "/1-mpa.png",
        "/2-mpa-jquery.png",
        "/3-spa.png",
        "/4-spa-server.png",
        "/5-wasm.png",
    ];

    view! {
        <Slide title="MPA to SPA">
            <div class="h-[50vh] flex items-center justify-center">
                {images
                    .into_iter()
                    .enumerate()
                    .map(|(id, src)| {
                        view! {
                            <Replace id=id as u8>
                                <img
                                    class="max-h-full w-auto object-contain"
                                    src=src
                                />
                            </Replace>
                        }
                    })
                    .collect_view()
                }
            </div>
        </Slide>
    }
}
