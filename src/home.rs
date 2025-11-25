use leptos::prelude::*;
use leptos::task::spawn_local;
use singlestage::input::*;
use singlestage::radio::*;
use singlestage::*;

use crate::components::ProgressBar;
use crate::server_fns::send_answer;
use crate::server_fns::Answer;
use crate::server_fns::{register_user, Question, SlideStatistics, QUESTION_SIGNAL};
#[component]
pub fn HomePage() -> impl IntoView {
    let slide_stats =
        leptos_ws::ReadOnlySignal::new("slidestats", SlideStatistics::default()).unwrap();
    let _ = leptos_ws::ReadOnlySignal::new(
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

            <main class="flex-1 flex bg-gradient-to-b from-white to-slate-50">
                <div class="mx-auto flex w-full max-w-5xl flex-col px-6 py-10 gap-8">
                    <article class="prose prose-slate max-w-none">
                        <h1>Welcome to Leptos</h1>
                        <p>
                            {"You are participating in a small interactive quiz powered by Leptos."}
                        </p>
                        <p>{"Current slide: "} {move || cloned.get().current}</p>
                    </article>

                    <div class="mt-2 flex-1 not-prose">
                        <QuizzArea />
                    </div>
                </div>
            </main>

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
        <div class="flex h-full items-start md:items-center justify-center">
            <Show
                when=move || { loged_in.get() }
                fallback=move || {
                    view! {
                        <div class="w-full max-w-md space-y-4">
                            <div class="prose prose-slate">
                                <h2>Join the quiz</h2>
                            </div>

                            <div class="not-prose space-y-3">
                                <Input input_type="text" value=username>
                                    "What's your name?"
                                </Input>

                                <Button
                                    size="normal"
                                    variant="primary"
                                    class="w-full justify-center"
                                    on:click=log_in
                                >
                                    "Join the quiz"
                                </Button>
                            </div>
                        </div>
                    }
                }
            >
                <div class="w-full max-w-2xl space-y-6">
                    <QuestionArea username=username />
                </div>
            </Show>
        </div>
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
                    <article class="prose prose-slate">
                        <h4>"Hello "{move || username()}</h4>
                        <p>"We are waiting for a quizz. Relax and just listen"</p>
                    </article>
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
                    <div class="space-y-4">
                        <article class="prose prose-slate">
                            <h2>{title.clone()}</h2>
                        </article>

                        <div class="not-prose">
                            <RadioGroup value=value class="space-y-2">
                                {answers
                                    .into_iter()
                                    .map(|a| {
                                        view! {
                                            <Radio
                                                class="block rounded-md border border-slate-200 bg-white px-4 py-2 hover:border-orange-400 transition"
                                                value=a.clone()
                                            >
                                                {a.clone()}
                                            </Radio>
                                        }
                                    })
                                    .collect_view()}
                            </RadioGroup>
                        </div>
                    </div>
                }
            }}

        </Show>
    }
}
