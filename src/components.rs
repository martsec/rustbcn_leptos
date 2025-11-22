use std::collections::HashMap;

use crate::server_fns::{
    send_question, AnsweredQuestion, Question, SlideStatistics, ANSWERS_SIGNAL, QUESTION_SIGNAL,
};
use leptos::{prelude::*, task::spawn_local};

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

#[component]
pub fn QuestionBlock(q: Question) -> impl IntoView {
    Effect::new({
        let q = q.clone();
        move || {
            let q_c = q.clone();
            spawn_local(async {
                send_question(q_c).await.unwrap();
            });
        }
    });

    let default: HashMap<String, AnsweredQuestion> = HashMap::new();
    let answer = leptos_ws::ReadOnlySignal::new(ANSWERS_SIGNAL, default).unwrap();
    let q2 = q.clone();
    let answers = Memo::new(move |_| match answer.get().get(&q2.text) {
        Some(a) => a.clone(),
        None => AnsweredQuestion::default(),
    });

    view! {
        <h3>{q.text}</h3>
        <AnswerPlot answers=answers />
    }
}

#[component]
pub fn AnswerPlot(answers: Memo<AnsweredQuestion>) -> impl IntoView {
    let stats = Memo::new(move |_| answers.get().stats());
    let max = Memo::new(move |_| stats.get().values().copied().max().unwrap_or(1));
    Effect::new(move |_| {
        leptos::logging::debug_log!("{:?}", stats.get());
        leptos::logging::debug_log!("Max: {:?}", max.get());
    });
    view! {
        <div class="space-y-4">
            <For
                each=move || {
                    let s = stats.get();
                    let mut answers: Vec<String> = s.keys().cloned().collect();
                    answers.sort_by(|a, b| s[b].cmp(&s[a]));
                    answers
                }
                key=|answer| answer.clone()
                children=move |answer: String| {
                    let count = {
                        let answer = answer.clone();
                        move || { stats.get().get(&answer).copied().unwrap_or(0) }
                    };
                    let pct = {
                        let count = count.clone();
                        move || {
                            let m = max.get();
                            if m == 0 { 0.0 } else { (count() as f32 / m as f32) * 100.0 }
                        }
                    };

                    view! {
                        <div class="space-y-1 transition-all duration-500">
                            <div class="flex justify-between text-xs text-gray-700">
                                <span class="text-xl">{answer.clone()}</span>
                                <span class="text-xl">{count}</span>
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
                                <div
                                    class="h-3 rounded-full bg-red-500 transition-[width] duration-500"
                                    style=move || format!("width: {}%;", pct())
                                ></div>
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}
