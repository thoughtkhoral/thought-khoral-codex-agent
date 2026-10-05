// SPDX-License-Identifier: Apache-2.0
use crate::protocol::MAX_SAFE_INTEGER;
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
pub struct UsageTracker {
    thread: String,
    turn: String,
    model: String,
    reading: Option<Value>,
    observed: Option<DateTime<Utc>>,
}
impl UsageTracker {
    pub fn new(thread: String, turn: String, model: String) -> Self {
        Self {
            thread,
            turn,
            model,
            reading: None,
            observed: None,
        }
    }
    pub fn report(&mut self, report: &Value, received: DateTime<Utc>) -> Option<Value> {
        if report["threadId"] != self.thread || report["turnId"] != self.turn {
            return None;
        }
        if self.observed.is_some_and(|prior| received < prior) {
            return self.snapshot();
        }
        self.observed = Some(received);
        let numerator = report["tokenUsage"]["last"]["totalTokens"]
            .as_u64()
            .filter(|n| *n <= MAX_SAFE_INTEGER);
        let Some(numerator) = numerator else {
            self.reading = None;
            return None;
        };
        let window = report["tokenUsage"]["modelContextWindow"]
            .as_u64()
            .filter(|n| *n > 0 && *n <= MAX_SAFE_INTEGER);
        self.reading = Some(
            json!({"lastTotalTokens":numerator,"modelContextWindow":window,"reportedAt":received,"model":self.model,"freshness":if window.is_some(){"fresh"}else{"unavailable"}}),
        );
        self.snapshot()
    }
    pub fn snapshot(&self) -> Option<Value> {
        self.reading.clone()
    }
    pub fn mark_stale(&mut self) {
        if let Some(reading) = self.reading.as_mut() {
            if reading["freshness"] == "fresh" {
                reading["freshness"] = json!("stale");
            }
        }
    }
    pub fn compacted(&mut self) {
        self.reading = None;
        self.observed = None;
    }
    pub fn reset(&mut self, model: String) {
        self.model = model;
        self.compacted();
    }
}
