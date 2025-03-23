use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[derive(Clone)]
struct GridItem {
    title: String,
    subtitle: String,
    transition_index: i32,
    items: Vec<SharedItemData>,
}

#[derive(Clone)]
struct SharedItemData {
    title: String,
    price: String,
    transition_index: i32,
    chevron_index: i32,
}

#[component]
pub fn DemoJsGridCollection() -> impl IntoView {
    let grid_items = vec![
        GridItem {
            title: "Daily Needs".to_string(),
            subtitle: "4 items".to_string(),
            transition_index: 1,
            items: vec![
                SharedItemData {
                    title: "Groceries".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 1,
                    chevron_index: 1,
                },
                SharedItemData {
                    title: "Groceries".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 2,
                    chevron_index: 5,
                },
                SharedItemData {
                    title: "Groceries".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 3,
                    chevron_index: 6,
                },
                SharedItemData {
                    title: "Groceries".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 4,
                    chevron_index: 7,
                },
            ],
        },
        GridItem {
            title: "Utilities".to_string(),
            subtitle: "3 items".to_string(),
            transition_index: 2,
            items: vec![
                SharedItemData {
                    title: "Electricity".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 5,
                    chevron_index: 2,
                },
                SharedItemData {
                    title: "Water".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 6,
                    chevron_index: 9,
                },
                SharedItemData {
                    title: "Internet".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 7,
                    chevron_index: 10,
                },
            ],
        },
        GridItem {
            title: "Subscriptions".to_string(),
            subtitle: "4 items".to_string(),
            transition_index: 3,
            items: vec![
                SharedItemData {
                    title: "Streaming".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 8,
                    chevron_index: 3,
                },
                SharedItemData {
                    title: "Courses".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 9,
                    chevron_index: 12,
                },
                SharedItemData {
                    title: "Software".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 10,
                    chevron_index: 13,
                },
                SharedItemData {
                    title: "Streaming".to_string(),
                    price: "$80.00".to_string(),
                    transition_index: 11,
                    chevron_index: 14,
                },
            ],
        },
    ];

    view! {
        <Stylesheet id="grid-collection" href="/components/grid_collection.css" />
        <script src="/components/grid_collection.js" />

        <div class="mainDiv">
            <div class="grid">
                {grid_items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <SharedGridItem
                                title=item.title
                                subtitle=item.subtitle
                                transition_index=item.transition_index
                            >
                                {item
                                    .items
                                    .into_iter()
                                    .map(|shared_item| {
                                        view! {
                                            <SharedItem
                                                title=shared_item.title
                                                price=shared_item.price
                                                transition_index=shared_item.transition_index
                                                chevron_index=shared_item.chevron_index
                                            />
                                        }
                                    })
                                    .collect_view()}
                            </SharedGridItem>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn SharedGridItem(
    children: Children,
    title: String,
    subtitle: String,
    transition_index: i32,
) -> impl IntoView {
    let transition_grid_item = format!("view-transition-name: grid-item-{}", transition_index);
    let transition_grid_icon = format!("view-transition-name: icon-grid-{}", transition_index);

    let transition_name_content = format!("view-transition-name: content-{}", transition_index);
    let transition_name_title = format!("view-transition-name: title-{}", transition_index);
    let transition_name_close_icon =
        format!("view-transition-name: close-icon-{}", transition_index);

    let transition_name_count = format!("view-transition-name: count-{}", transition_index);
    let transition_name_chevron = format!("view-transition-name: chevron-{}", transition_index);

    //
    //
    const CLASS_CLOSE_ICON: &str =
        "close-icon flex items-center justify-center rounded-full size-6 bg-neutral-100 hidden";

    view! {
        <div class="grid-item" style=transition_grid_item>
            <div class="icon-grid" style=transition_grid_icon>
                {children()}
            </div>

            <div class="content" style=transition_name_content>
                <div
                    class="text-base font-medium text-zinc-800 w-fit h-fit"
                    style=transition_name_title
                >
                    {title}
                </div>
                <div class=CLASS_CLOSE_ICON style=transition_name_close_icon>
                    <svg
                        width="16"
                        height="16"
                        viewBox="0 0 16 16"
                        fill="none"
                        stroke="currentColor"
                    >
                        <path d="M12 4L4 12M4 4l8 8" stroke-width="2" stroke-linecap="round" />
                    </svg>
                </div>
                <div
                    class="displayNoneWhenExpanded text-muted-foreground"
                    style=transition_name_count
                >
                    {subtitle}
                </div>
            </div>
            <div class="chevron" style=transition_name_chevron>
                <SvgChevron />
            </div>
        </div>
    }
}

#[component]
pub fn SharedItem(
    title: String,
    price: String,
    transition_index: i32,
    chevron_index: i32,
) -> impl IntoView {
    let transition_title = format!("view-transition-name: item-title-{}", transition_index);
    let transition_price = format!("view-transition-name: item-price-{}", transition_index);

    let transition_item = format!("view-transition-name: item-{}", transition_index);
    let transition_icon = format!("view-transition-name: icon-{}", transition_index);

    let transition_chevron = format!("view-transition-name: chevron-{}", chevron_index);

    view! {
        <div class="item" style=transition_item>
            <div class="icon" style=transition_icon>
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <circle cx="12" cy="12" r="10" stroke-width="2" />
                    <path d="M10 8l6 4-6 4V8z" stroke-width="2" />
                </svg>
            </div>

            <div class="item-content text-neutral-500">
                <div class="item-title" style=transition_title>
                    {title}
                </div>
                <div class="item-price" style=transition_price>
                    {price}
                </div>
            </div>

            <div class="chevron" style=transition_chevron>
                <SvgChevron />
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn SvgChevron() -> impl IntoView {
    view! {
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M9 18l6-6-6-6" stroke-width="2" stroke-linecap="round" />
        </svg>
    }
}
