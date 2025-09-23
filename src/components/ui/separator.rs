use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: Signal<SeparatorOrientation>,
    #[prop(into, optional)] class: Signal<String>,
    // children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        let orientation = orientation.get();
        let separator = SeparatorClass { orientation };
        separator.with_class(class.get())
    });

    view! { <div class=class role="separator" /> }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🧬 STRUCT 🧬                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwClass, Default)]
#[tw(class = "shrink-0 bg-border")]
pub struct SeparatorClass {
    orientation: SeparatorOrientation,
}

#[derive(TwVariant)]
pub enum SeparatorOrientation {
    #[tw(default, class = "w-full h-[1px]")]
    Default,
    #[tw(class = "h-full w-[1px]")]
    Vertical,
}
