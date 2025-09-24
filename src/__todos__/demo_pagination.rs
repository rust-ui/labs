use leptos::prelude::*;

use crate::components::ui::pagination::{
    Pagination, PaginationActive, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationLinkWithButton, PaginationNext, PaginationPrevious,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10">

            // WITHOUT BUTTON
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

            // WITH BUTTON FOR A TAG
            <Pagination attr:role="navigation" attr:aria-label="pagination">
                <PaginationContent>
                    <PaginationItem>
                        <PaginationPrevious href="#" />
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationLinkWithButton href="#">1</PaginationLinkWithButton>
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
                        <PaginationLinkWithButton href="#">3</PaginationLinkWithButton>
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationEllipsis />
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationNext href="#" />
                    </PaginationItem>
                </PaginationContent>
            </Pagination>
        </div>
    }
}

// <Pagination>
// <PaginationContent>
//   <PaginationItem>
//     <PaginationPrevious attr:href="#" />
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationLink attr:href="#">1</PaginationLink>
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationLink attr:href="#" isActive>
//       2
//     </PaginationLink>
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationLink attr:href="#">3</PaginationLink>
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationEllipsis />
//   </PaginationItem>
//   <PaginationItem>
//     <PaginationNext attr:href="#" />
//   </PaginationItem>
// </PaginationContent>
// </Pagination>
