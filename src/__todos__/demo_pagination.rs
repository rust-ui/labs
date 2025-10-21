use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationActive, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationLinkWithButton, PaginationNext, PaginationPrevious,
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
                    <PaginationActive
                        attr:aria-current="page"
                        attr:data-active="true"
                        attr:href="#"
                    >
                        2
                    </PaginationActive>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink attr:href="#">3</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="#" />
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
