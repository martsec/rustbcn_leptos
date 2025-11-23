use chrono::{DateTime, Utc};
use leptos::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SlideStatistics {
    pub current: u32,
    pub total: u32,
}

#[server]
pub async fn update_slides(stats: SlideStatistics) -> Result<(), ServerFnError> {
    let ws = leptos_ws::ReadOnlySignal::new("slidestats", SlideStatistics::default()).unwrap();
    ws.update(move |v| *v = stats);
    Ok(())
}

#[allow(unused)]
static USERS: Lazy<RwLock<HashMap<String, DateTime<Utc>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[server]
pub async fn register_user(username: String) -> Result<(), ServerFnError> {
    let mut users = USERS.write().unwrap();

    if users.contains_key(&username) {
        return Err(ServerFnError::ServerError("username already exists".into()));
    }

    users.insert(username, Utc::now());
    Ok(())
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Question {
    pub text: String,
    pub answers: Vec<String>,
}

impl Question {
    pub fn is_initialized(&self) -> bool {
        !self.text.is_empty()
    }
}

pub static QUESTION_SIGNAL: &str = "question";

#[server]
pub async fn send_question(q: Question) -> Result<(), ServerFnError> {
    let ws = leptos_ws::ReadOnlySignal::new(QUESTION_SIGNAL, Question::default()).unwrap();
    ws.update(move |v| *v = q);
    Ok(())
}

pub static ANSWERS_SIGNAL: &str = "answer";
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Answer {
    pub user: String,
    pub question: String,
    pub answer: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct AnsweredQuestion {
    /// username, answer
    pub answers: HashMap<String, String>,
}
impl AnsweredQuestion {
    /// Returns answer, count of users that marked this answer
    pub fn stats(&self) -> HashMap<String, u8> {
        let mut counts: HashMap<String, u8> = HashMap::new();
        for (_, ans) in &self.answers {
            let entry = counts.entry(ans.clone()).or_insert(0);
            *entry = entry.saturating_add(1);
        }
        counts
    }
}

#[allow(unused)]
static ANSWERS: Lazy<RwLock<HashMap<String, AnsweredQuestion>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
#[server]
pub async fn send_answer(a: Answer) -> Result<(), ServerFnError> {
    let mut answers = ANSWERS.write().unwrap();
    let entry = answers.entry(a.question.clone()).or_default();
    entry.answers.insert(a.user.clone(), a.answer.clone());
    let snapshot = answers.clone();

    let default: HashMap<String, AnsweredQuestion> = HashMap::new();
    let ws = leptos_ws::ReadOnlySignal::new(ANSWERS_SIGNAL, default).unwrap();

    ws.update(move |v| *v = snapshot);

    Ok(())
}
