use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationEllipsis, PaginationItem, PaginationLink, PaginationList, PaginationNext,
    PaginationPrevious,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination query_key="page" max_pages=10u32>
            <PaginationList>
                <PaginationItem>
                    <PaginationPrevious />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=1>1</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=4>4</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=5>5</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=6>6</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=10>10</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext />
                </PaginationItem>
            </PaginationList>
        </Pagination>
    }
}
