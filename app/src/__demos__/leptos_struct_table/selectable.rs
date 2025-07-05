use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_struct_table::{ColumnSort, Selection, SelectionChangeEvent, SortingMode, TableRow};
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
pub fn DemoLeptosStructTable_Selectable() -> impl IntoView {
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

    let selected_index = RwSignal::new(None);
    let (selected_row, set_selected_row) = signal(None);

    let on_selection_change = move |evt: SelectionChangeEvent<Book>| {
        set_selected_row.set(Some(evt.row));
    };

    view! {
        <div class="float-left m-10 rounded-md border dark:border-gray-700 overflow-clip">
            <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                <TableContent
                    rows=rows
                    scroll_container="html"
                    selection=Selection::Single(selected_index)
                    row_class="select-none"
                    on_selection_change=on_selection_change
                    sorting_mode=SortingMode::SingleColumn
                />
            </table>
        </div>

        {move || {
            selected_row
                .get()
                .map(|selected_row| {
                    let selected_row = selected_row.get();

                    view! {
                        <div class="float-left py-3 px-5 m-10 text-gray-700 bg-white rounded-md border dark:text-gray-400 dark:bg-gray-800 dark:border-gray-700 overflow-clip">
                            <pre>
                                "          Id:  " {selected_row.id} "\n" "       Title:  "
                                {selected_row.title} "\n" "      Author:  " {selected_row.author}
                                "\n" "Publish Date:  " {selected_row.publish_date.to_string()}
                            </pre>
                        </div>
                    }
                })
        }}
    }
}
