use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::shared::constants::QUERY;

pub struct QueryUtils;

impl QueryUtils {
    pub fn extract_demo() -> Memo<String> {
        let location = use_location();

        Memo::new(move |_| {
            location.query.with(|q| {
                q.get(QUERY::DEMO)
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            })
        })
    }
}
