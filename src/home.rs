use leptos::prelude::*;
use leptos::task::spawn_local;
use singlestage::input::*;
use singlestage::radio::*;
use singlestage::*;

use crate::components::ProgressBar;
use crate::server_fns::send_answer;
use crate::server_fns::Answer;
use crate::server_fns::{register_user, Question, SlideStatistics, QUESTION_SIGNAL};
/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let slide_stats =
        leptos_ws::ReadOnlySignal::new("slidestats", SlideStatistics::default()).unwrap();
    let question = leptos_ws::ReadOnlySignal::new(
        QUESTION_SIGNAL,
        Question {
            text: String::new(),
            answers: vec![],
        },
    )
    .unwrap();

    let cloned = slide_stats.clone();
    view! {
        <div class="min-h-screen max-h-screen flex flex-col">
            <header>
                <ProgressBar stats=move || slide_stats.get() />
            </header>
            <h1 class="text-2xl">"Welcome to Leptos!"</h1>
            <p>Our slidedeck is in page {move || cloned.get().current}</p>

            <QuizzArea />

        </div>
    }
}

#[component]
pub fn QuizzArea() -> impl IntoView {
    let username = RwSignal::new("".to_string());
    let loged_in = RwSignal::new(false);
    let log_in = move |_| {
        spawn_local(async move {
            let _ = register_user(username.get()).await;
        });
        loged_in.set(true);
    };

    view! {
        <Show
            when=move || { loged_in.get() }
            fallback=move || {
                view! {
                    <div class="gap-4 max-w-100">
                        <Input input_type="text" value=username>
                            "What's your name?"
                        </Input>

                        <Button size="normal" variant="primary" on:click=log_in>
                            "Join the quizz"
                        </Button>
                    </div>
                }
            }
        >
            <QuestionArea username=username />

        </Show>
    }
}

#[component]
pub fn QuestionArea(username: RwSignal<String>) -> impl IntoView {
    let question = leptos_ws::ReadOnlySignal::new(
        QUESTION_SIGNAL,
        Question {
            text: String::new(),
            answers: vec![],
        },
    )
    .unwrap();
    let q = {
        let q = question.clone();
        move || q.get()
    };

    fn send_answer_internal(user: &str, question: &str, answer: &str) {
        if !answer.is_empty() {
            let a = Answer {
                user: user.into(),
                question: question.into(),
                answer: answer.into(),
            };
            spawn_local(async { send_answer(a).await.unwrap() });
        }
    }

    view! {
        <Show
            when=move || q().is_initialized()
            fallback=move || {
                view! {
                    <h4>"Hello "{username}</h4>
                    <p>"We are waiting for a quizz. Relax and just listen"</p>
                }
            }
        >

            {move || {
                let question = leptos_ws::ReadOnlySignal::new(
                        QUESTION_SIGNAL,
                        Question {
                            text: String::new(),
                            answers: vec![],
                        },
                    )
                    .unwrap();
                let q = question.get();
                let title = q.text.clone();
                let answers = q.answers.clone();
                let value = RwSignal::new(String::new());
                Effect::new({
                    let t = title.clone();
                    move || { send_answer_internal(&username.get(), &t, &value.get()) }
                });

                view! {
                    <h3>{title.clone()}</h3>
                    <RadioGroup value=value>
                        {answers
                            .into_iter()
                            .map(|a| {
                                view! { <Radio value=a.clone()>{a.clone()}</Radio> }
                            })
                            .collect_view()}

                    </RadioGroup>
                }
            }}

        </Show>
    }
}
