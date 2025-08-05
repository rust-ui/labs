use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationActive, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationNext, PaginationPrevious,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination role="navigation" aria_label="pagination">
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious href="#" />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#">1</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationActive aria_current="page" data_active="true" href="#">
                        2
                    </PaginationActive>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#">3</PaginationLink>
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

// <Pagination>
// <PaginationContent>
//   <PaginationItem>
//     <PaginationPrevious href="#" />
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationLink href="#">1</PaginationLink>
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationLink href="#" isActive>
//       2
//     </PaginationLink>
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationLink href="#">3</PaginationLink>
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationEllipsis />
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationNext href="#" />
//   </PaginationItem>
// </PaginationContent>
// </Pagination>
