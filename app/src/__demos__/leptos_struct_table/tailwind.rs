use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_struct_table::{ColumnSort, TableClassesProvider, TableRow};
use leptos_struct_table::{TableContent, TableDataProvider};

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

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn thead_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-300",
            template_classes
        )
    }

    fn thead_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-5 py-2 {sort_class} {template_classes}"
        )
    }

    fn thead_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index.is_multiple_of(2) {
            if selected {
                "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
            } else {
                "bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800"
            }
        } else if selected {
            "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
        } else {
            "bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700"
        };

        format!(
            "{} {} {}",
            "border-b dark:border-gray-700", bg_color, template_classes
        )
    }

    fn loading_cell(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!("{} {}", "px-5 py-2", prop_class)
    }

    fn loading_cell_inner(&self, row_index: usize, _col_index: usize, prop_class: &str) -> String {
        let width = match row_index % 4 {
            0 => "w-[calc(85%-2.5rem)]",
            1 => "w-[calc(90%-2.5rem)]",
            2 => "w-[calc(75%-2.5rem)]",
            _ => "w-[calc(60%-2.5rem)]",
        };
        format!(
            "animate-pulse h-2 bg-gray-200 rounded-full dark:bg-gray-700 inline-block align-middle {width} {prop_class}"
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}
