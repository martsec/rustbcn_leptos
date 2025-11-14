use leptos::prelude::*;
use serde::{Deserialize, Serialize};

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
