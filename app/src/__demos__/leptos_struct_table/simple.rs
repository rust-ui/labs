use leptos::prelude::*;
use leptos_struct_table::*;

#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
}

#[component]
pub fn DemoLeptosStructTable_Simple() -> impl IntoView {
    let rows = vec![
        Person {
            id: 1,
            name: "John".to_string(),
            age: 32,
        },
        Person {
            id: 2,
            name: "Jane".to_string(),
            age: 28,
        },
        Person {
            id: 3,
            name: "Bob".to_string(),
            age: 45,
        },
    ];

    view! {
        <table>
            <TableContent rows scroll_container="html" />
        </table>
    }
}
