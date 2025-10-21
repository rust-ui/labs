use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::utils::query::QueryUtils;

#[derive(Clone)]
pub struct PaginationContext {
    pub current_page: Memo<u32>,
    pub page_href: Callback<u32, String>,
    pub max_pages: u32,
}

pub fn use_pagination(query_key: String, max_pages: u32) -> PaginationContext {
    let location = use_location();
    let current_page_str = QueryUtils::extract(query_key.clone());

    let current_page = Memo::new(move |_| current_page_str().parse::<u32>().unwrap_or(1));

    let page_href = Callback::new(move |page: u32| {
        location.query.with(|q| {
            let demo_param = q
                .get("demo")
                .map(|d| format!("demo={}&", d))
                .unwrap_or_default();
            format!("?{}{}={}", demo_param, query_key, page)
        })
    });

    PaginationContext {
        current_page,
        page_href,
        max_pages,
    }
}
