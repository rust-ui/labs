use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_struct_table::{ColumnSort, TableRow};
use leptos_struct_table::{TableContent, TableDataProvider};

use super::tailwind_classes::TailwindClassesPreset;

// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider
)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    #[table(
        cell_class = "text-red-600 dark:text-red-400",
        head_class = "text-red-700 dark:text-red-300"
    )]
    pub publish_date: NaiveDate,
}

#[component]
pub fn DemoLeptosStructTable_Tailwind() -> impl IntoView {
    let rows = vec![
        Book {
            id: 1,
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
            publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).expect("Invalid date"),
        },
        Book {
            id: 2,
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
            publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).expect("Invalid date"),
        },
        Book {
            id: 3,
            title: "Nineteen Eighty-Four".to_string(),
            author: "George Orwell".to_string(),
            publish_date: NaiveDate::from_ymd_opt(1949, 6, 8).expect("Invalid date"),
        },
        Book {
            id: 4,
            title: "Ulysses".to_string(),
            author: "James Joyce".to_string(),
            publish_date: NaiveDate::from_ymd_opt(1922, 2, 2).expect("Invalid date"),
        },
    ];

    view! {
        <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 float-left"
            .to_string()>
            <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                <TableContent rows=rows scroll_container="html" />
            </table>
        </div>
    }
}
