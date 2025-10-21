use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::utils::query::{QUERY, QueryUtils};

#[derive(Clone)]
pub struct PaginationContext {
    pub current_page: Memo<u32>,
    pub page_href: Callback<u32, String>,
}

pub fn use_pagination() -> PaginationContext {
    let location = use_location();
    let current_page_str = QueryUtils::extract(QUERY::PAGE.to_string());

    let current_page = Memo::new(move |_| current_page_str().parse::<u32>().unwrap_or(1));

    let page_href = Callback::new(move |page: u32| {
        location.query.with(|q| {
            let mut params: Vec<String> = q
                .clone()
                .into_iter()
                .filter(|(key, _)| key != QUERY::PAGE)
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();

            params.push(format!("{}={}", QUERY::PAGE, page));

            format!("?{}", params.join("&"))
        })
    });

    PaginationContext {
        current_page,
        page_href,
    }
}
