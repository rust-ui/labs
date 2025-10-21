use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationEllipsis, PaginationItem, PaginationLink, PaginationList, PaginationNext,
    PaginationPrevious,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination attr:role="navigation" attr:aria-label="pagination">
            <PaginationList>
                <PaginationItem>
                    <PaginationPrevious href="#" />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink attr:href="#">1</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink attr:href="#">4</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink attr:aria-current="page" attr:href="#">
                        5
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink attr:href="#">6</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink attr:href="#">10</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="#" />
                </PaginationItem>
            </PaginationList>
        </Pagination>
    }
}
