// SPDX-License-Identifier: Apache-2.0
use chrono::{Duration, Utc};
use serde_json::{Value, json};
use thought_khoral_codex_agent::{catalog::Catalog, protocol::RuntimeError, usage::UsageTracker};
fn page() -> Value {
    json!({"data":[{"id":"option-a","model":"native-a","displayName":"A","description":"Test","hidden":false,"isDefault":true,"defaultReasoningEffort":"medium","supportedReasoningEfforts":[{"reasoningEffort":"medium","description":"Medium"},{"reasoningEffort":"high","description":"High"}]}],"nextCursor":null})
}
fn report(thread: &str, turn: &str, last: Value, window: Value) -> Value {
    json!({"threadId":thread,"turnId":turn,"tokenUsage":{"last":{"totalTokens":last,"inputTokens":8,"outputTokens":4,"cachedInputTokens":5,"reasoningOutputTokens":3},"total":{"totalTokens":9000,"inputTokens":8000,"outputTokens":1000,"cachedInputTokens":4000,"reasoningOutputTokens":100},"modelContextWindow":window}})
}
#[test]
fn catalog_maps_opaque_options_and_rejects_hidden_disallowed_or_unsupported_choices() {
    let mut catalog = Catalog::new("revision-1".into(), vec!["option-a".into()]);
    catalog.add_page(&page()).unwrap();
    let selection = catalog.resolve("option-a", "effort-medium").unwrap();
    assert_eq!(selection.model, "native-a");
    assert_eq!(selection.effort, "medium");
    assert_eq!(
        catalog.resolve("option-a", "medium").unwrap_err(),
        RuntimeError::InvalidTaskInput
    );
    assert!(catalog.resolve("unadmitted", "effort-medium").is_err());
    let view = catalog.page();
    assert_eq!(
        view["data"][0]["supportedReasoningEfforts"][0]["id"],
        "effort-medium"
    );
    assert_eq!(view["data"][0]["defaultReasoningEffort"], "effort-medium");
    assert!(!view.to_string().contains("native-a"));
}
#[test]
fn duplicate_catalog_entries_and_unsupported_defaults_fail_closed() {
    let mut catalog = Catalog::new("revision-1".into(), vec!["option-a".into()]);
    catalog.add_page(&page()).unwrap();
    assert!(catalog.add_page(&page()).is_err());
    let mut bad = page();
    bad["data"][0]["defaultReasoningEffort"] = json!("low");
    assert!(
        Catalog::new("r".into(), vec!["option-a".into()])
            .add_page(&bad)
            .is_err()
    );
}
#[test]
fn last_request_tokens_are_not_cumulative_or_double_counted() {
    let now = Utc::now();
    let mut tracker = UsageTracker::new("thread".into(), "turn".into(), "option-a".into());
    let snapshot = tracker
        .report(&report("thread", "turn", json!(12), json!(100)), now)
        .unwrap();
    assert_eq!(snapshot["lastTotalTokens"], 12);
    assert_eq!(snapshot["modelContextWindow"], 100);
    assert_eq!(snapshot["freshness"], "fresh");
    let zero = tracker
        .report(
            &report("thread", "turn", json!(0), json!(100)),
            now + Duration::seconds(1),
        )
        .unwrap();
    assert_eq!(zero["lastTotalTokens"], 0);
}
#[test]
fn invalid_window_keeps_numerator_unavailable_and_invalid_numerator_never_invents_zero() {
    for window in [Value::Null, json!(0), json!(-1)] {
        let mut tracker = UsageTracker::new("thread".into(), "turn".into(), "option-a".into());
        let snapshot = tracker
            .report(&report("thread", "turn", json!(12), window), Utc::now())
            .unwrap();
        assert_eq!(snapshot["modelContextWindow"], Value::Null);
        assert_eq!(snapshot["freshness"], "unavailable");
        assert_eq!(snapshot["lastTotalTokens"], 12);
    }
    for numerator in [
        Value::Null,
        json!(-1),
        json!(1.5),
        json!(9_007_199_254_740_992_u64),
    ] {
        let mut tracker = UsageTracker::new("thread".into(), "turn".into(), "option-a".into());
        assert!(
            tracker
                .report(&report("thread", "turn", numerator, json!(100)), Utc::now())
                .is_none()
        );
    }
}
#[test]
fn binding_freshness_model_switch_and_compaction_never_reuse_a_stale_denominator() {
    let now = Utc::now();
    let mut tracker = UsageTracker::new("thread".into(), "turn".into(), "option-a".into());
    assert!(
        tracker
            .report(&report("other", "turn", json!(12), json!(100)), now)
            .is_none()
    );
    assert!(
        tracker
            .report(&report("thread", "other", json!(12), json!(100)), now)
            .is_none()
    );
    tracker
        .report(&report("thread", "turn", json!(120), json!(100)), now)
        .unwrap();
    let preserved = tracker
        .report(
            &report("thread", "turn", json!(10), json!(100)),
            now - Duration::seconds(1),
        )
        .unwrap();
    assert_eq!(preserved["lastTotalTokens"], 120);
    tracker.mark_stale();
    assert_eq!(tracker.snapshot().unwrap()["freshness"], "stale");
    tracker.compacted();
    assert!(tracker.snapshot().is_none());
    tracker
        .report(&report("thread", "turn", json!(8), json!(100)), now)
        .unwrap();
    tracker.reset("option-b".into());
    assert!(tracker.snapshot().is_none());
}
