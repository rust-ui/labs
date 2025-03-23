use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "bg-gray-300 p-4 flex items-center space-x-4",

            Link { to: Routes::PageHome {}, "Home" }
            Link { to: Routes::PageTest {}, "Test" }
            Link { to: Routes::PageBlogId { id: 1 }, "Blog" }
        }

        Outlet::<Routes> {}
    }
}
