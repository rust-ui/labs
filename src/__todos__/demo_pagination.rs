use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationActive, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationNext, PaginationPrevious,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination attr:role="navigation" attr:aria-label="pagination">
            <PaginationContent>
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
                    <PaginationActive
                        attr:aria-current="page"
                        attr:data-active="true"
                        attr:href="#"
                    >
                        5
                    </PaginationActive>
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
            </PaginationContent>
        </Pagination>
    }
}
