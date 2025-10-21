use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::ui::pagination::{
    Pagination, PaginationEllipsis, PaginationItem, PaginationLink, PaginationList, PaginationNext,
    PaginationPrevious,
};
use crate::utils::query::QueryUtils;

#[component]
pub fn DemoPagination() -> impl IntoView {
    let location = use_location();
    let current_page = QueryUtils::extract("page".to_string());

    let is_current_page = move |page: u32| current_page().parse::<u32>().unwrap_or(1) == page;

    let page_href = move |page: u32| {
        location.query.with(|q| {
            let demo_param = q
                .get("demo")
                .map(|d| format!("demo={}&", d))
                .unwrap_or_default();
            format!("?{}page={}", demo_param, page)
        })
    };

    let prev_href = Signal::derive(move || {
        let current = current_page().parse::<u32>().unwrap_or(1);
        if current > 1 {
            page_href(current - 1)
        } else {
            "#".to_string()
        }
    });

    let next_href = Signal::derive(move || {
        let current = current_page().parse::<u32>().unwrap_or(1);
        if current < 10 {
            page_href(current + 1)
        } else {
            "#".to_string()
        }
    });

    view! {
        <Pagination attr:role="navigation" attr:aria-label="pagination">
            <PaginationList>
                <PaginationItem>
                    <PaginationPrevious href=prev_href />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink
                        attr:href=page_href(1)
                        attr:aria-current=move || if is_current_page(1) { "page" } else { "" }
                    >
                        1
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink
                        attr:href=page_href(4)
                        attr:aria-current=move || if is_current_page(4) { "page" } else { "" }
                    >
                        4
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink
                        attr:href=page_href(5)
                        attr:aria-current=move || if is_current_page(5) { "page" } else { "" }
                    >
                        5
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink
                        attr:href=page_href(6)
                        attr:aria-current=move || if is_current_page(6) { "page" } else { "" }
                    >
                        6
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink
                        attr:href=page_href(10)
                        attr:aria-current=move || if is_current_page(10) { "page" } else { "" }
                    >
                        10
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href=next_href />
                </PaginationItem>
            </PaginationList>
        </Pagination>
    }
}
