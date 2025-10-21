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
            let demo_param = q
                .get("demo")
                .map(|d| format!("demo={}&", d))
                .unwrap_or_default();
            format!("?{}{}={}", demo_param, QUERY::PAGE, page)
        })
    });

    PaginationContext {
        current_page,
        page_href,
    }
}
