use dioxus::prelude::*;

use crate::{
    components::navbar::Navbar, routing::page_blog_id::PageBlogId, routing::page_home::PageHome,
    routing::page_test::PageTest,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Routes {
    #[layout(Navbar)]
    
    #[route("/")]
    PageHome {},
    #[route("/test")]
    PageTest {},

    #[route("/blog/:id")]
    PageBlogId { id: i32 },
}
