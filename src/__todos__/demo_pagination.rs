use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationDirection, PaginationItem, PaginationLink, PaginationList,
    PaginationNavButton,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination query_key="page" max_pages=10u32>
            <PaginationList>
                <PaginationItem>
                    <PaginationNavButton direction=PaginationDirection::Previous />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=1>1</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=2>2</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=3>3</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=4>4</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=5>5</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNavButton direction=PaginationDirection::Next />
                </PaginationItem>
            </PaginationList>
        </Pagination>
    }
}
