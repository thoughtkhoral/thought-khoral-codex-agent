// SPDX-License-Identifier: Apache-2.0
use crate::protocol::{RuntimeError, validate_native, validate_profile};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone)]
pub struct Selection {
    pub model: String,
    pub effort: String,
}
struct Model {
    native: String,
    public: Value,
    efforts: HashMap<String, String>,
}
pub struct Catalog {
    revision: String,
    allowlist: Vec<String>,
    seen: HashSet<String>,
    models: HashMap<String, Model>,
}
impl Catalog {
    pub fn new(revision: String, allowlist: Vec<String>) -> Self {
        Self {
            revision,
            allowlist,
            seen: HashSet::new(),
            models: HashMap::new(),
        }
    }
    pub fn add_page(&mut self, page: &Value) -> Result<(), RuntimeError> {
        validate_native("ModelListResponse", page)?;
        let data = page["data"]
            .as_array()
            .ok_or(RuntimeError::RuntimeUnavailable)?;
        if data.len() > 100 {
            return Err(RuntimeError::RuntimeUnavailable);
        }
        for item in data {
            let id = item["id"]
                .as_str()
                .ok_or(RuntimeError::RuntimeUnavailable)?
                .to_owned();
            if id.is_empty()
                || id.chars().count() > 128
                || !self.seen.insert(id.clone())
                || self.seen.len() > 10_000
            {
                return Err(RuntimeError::RuntimeUnavailable);
            }
            let mut efforts = HashMap::new();
            let mut options = vec![];
            for effort in item["supportedReasoningEfforts"]
                .as_array()
                .ok_or(RuntimeError::RuntimeUnavailable)?
            {
                let native = effort["reasoningEffort"]
                    .as_str()
                    .ok_or(RuntimeError::RuntimeUnavailable)?;
                let id = format!("effort-{native}");
                if efforts.insert(id.clone(), native.to_owned()).is_some() {
                    return Err(RuntimeError::RuntimeUnavailable);
                }
                options.push(json!({"id":id,"description":effort["description"]}));
            }
            let default = format!(
                "effort-{}",
                item["defaultReasoningEffort"]
                    .as_str()
                    .ok_or(RuntimeError::RuntimeUnavailable)?
            );
            if !efforts.contains_key(&default) {
                return Err(RuntimeError::RuntimeUnavailable);
            }
            if item["hidden"] == true || !self.allowlist.contains(&id) {
                continue;
            }
            let native = item["model"]
                .as_str()
                .filter(|s| !s.is_empty() && s.chars().count() <= 128)
                .ok_or(RuntimeError::RuntimeUnavailable)?
                .to_owned();
            let public = json!({"id":id,"displayName":item["displayName"],"defaultReasoningEffort":default,"supportedReasoningEfforts":options});
            validate_profile(
                "catalog",
                &json!({"profileVersion":crate::protocol::PROFILE_VERSION,"catalogRevision":self.revision,"data":[public.clone()],"nextCursor":null}),
            )?;
            if self.models.values().any(|model| model.native == native) {
                return Err(RuntimeError::RuntimeUnavailable);
            }
            self.models.insert(
                id,
                Model {
                    native,
                    public,
                    efforts,
                },
            );
        }
        Ok(())
    }
    pub fn resolve(&self, model: &str, effort: &str) -> Result<Selection, RuntimeError> {
        let selected = self
            .models
            .get(model)
            .ok_or(RuntimeError::InvalidTaskInput)?;
        Ok(Selection {
            model: selected.native.clone(),
            effort: selected
                .efforts
                .get(effort)
                .ok_or(RuntimeError::InvalidTaskInput)?
                .clone(),
        })
    }
    pub fn normalize_settings(&self, model: &str, effort: Option<&str>) -> Value {
        let entry = self.models.iter().find(|(_, value)| value.native == model);
        let known = entry.map(|(id, _)| id.clone());
        let effort = entry.and_then(|(_, entry)| {
            entry
                .efforts
                .iter()
                .find(|(_, native)| Some(native.as_str()) == effort)
                .map(|(id, _)| id.clone())
        });
        let confirmed = known.is_some() && effort.is_some();
        json!({"model":known,"reasoningEffort":effort,"confirmation":if confirmed {"confirmed"}else{"unconfirmed"},"reroutedModel":null})
    }
    pub fn page(&self) -> Value {
        let mut ids = self.models.keys().collect::<Vec<_>>();
        ids.sort_unstable();
        json!({"profileVersion":crate::protocol::PROFILE_VERSION,"catalogRevision":self.revision,"data":ids.into_iter().map(|id|self.models[id].public.clone()).collect::<Vec<_>>(),"nextCursor":null})
    }
}
