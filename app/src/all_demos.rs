use leptos::prelude::*;

use crate::__demos__::demo_alert_dialog::DemoAlertDialog;
use crate::__demos__::demo_apple_liquid_glass_ui::DemoAppleLiquidGlassUi;
use crate::__demos__::demo_bento_3_transition::DemoBento3Transition;
use crate::__demos__::demo_bg_gradient_interactive::DemoBgGradientInteractive;
use crate::__demos__::demo_bottom_bar_awwards::DemoBottomBarAwwwards;
use crate::__demos__::demo_button_multi_state::DemoButtonMultiState;
use crate::__demos__::demo_card_reorder::DemoCardReorder;
use crate::__demos__::demo_carousel::DemoCarousel;
use crate::__demos__::demo_carousel_hover_smooth::DemoCarouselHoverSmooth;
use crate::__demos__::demo_drawer::DemoDrawer;
use crate::__demos__::demo_gsap_dynamic_cursor::DemoGsapDynamicCursor;
use crate::__demos__::demo_gsap_looping_words::DemoGsapLoopingWords;
use crate::__demos__::demo_inline_picker::DemoInlinePicker;
use crate::__demos__::demo_menu_bar_interaction::DemoMenuBarInteraction;
use crate::__demos__::demo_menu_grid_transition::DemoMenuGridTransition;
use crate::__demos__::demo_menu_interaction::DemoMenuInteraction;
use crate::__demos__::demo_mobile_stack::DemoMobileStack;
use crate::__demos__::demo_password_strength_meter::DemoPasswordStrengthMeter;
use crate::__demos__::demo_slider_point::DemoSliderPoint;
use crate::__demos__::leptos_struct_table::editable::DemoLeptosStructTable_Editable;
use crate::__demos__::leptos_struct_table::selectable::DemoLeptosStructTable_Selectable;
use crate::__demos__::leptos_struct_table::simple::DemoLeptosStructTable_Simple;
use crate::__demos__::leptos_struct_table::tailwind::DemoLeptosStructTable_Tailwind;
use crate::shared::components::ui_hero_one::HeroOne;


#[derive(Clone)]
pub struct DemoItem {
    pub name: &'static str,
    pub render_fn: fn() -> AnyView,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub const ALL_DEMOS: &[DemoItem] = &[
    DemoItem {
    name: "🔥 Hero One - Rustify UI",
    render_fn: || view! { <HeroOne /> }.into_any(),
},

    DemoItem {
        name: "👉 TODO: Apple Liquid Glass UI",
        render_fn: || view! { <DemoAppleLiquidGlassUi /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Password Strength Meter",
        render_fn: || view! { <DemoPasswordStrengthMeter /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Gsap Dynamic Cursor",
        render_fn: || view! { <DemoGsapDynamicCursor /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Carousel",
        render_fn: || view! { <DemoCarousel /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Drawer",
        render_fn: || view! { <DemoDrawer /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Alert Dialog",
        render_fn: || view! { <DemoAlertDialog /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Card Reorder",
        render_fn: || view! { <DemoCardReorder /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Bottom Bar Awwwards",
        render_fn: || view! { <DemoBottomBarAwwwards /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Carousel Hover Smooth",
        render_fn: || view! { <DemoCarouselHoverSmooth /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Gsap Looping Words",
        render_fn: || view! { <DemoGsapLoopingWords /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Inline Picker",
        render_fn: || view! { <DemoInlinePicker /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Menu Interaction",
        render_fn: || view! { <DemoMenuInteraction /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Menu Bar Interaction",
        render_fn: || view! { <DemoMenuBarInteraction /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Menu Grid Transition",
        render_fn: || view! { <DemoMenuGridTransition /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Slider Point",
        render_fn: || view! { <DemoSliderPoint /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Button Multi State",
        render_fn: || view! { <DemoButtonMultiState /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Mobile Stack",
        render_fn: || view! { <DemoMobileStack /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Bg Gradient Interactive",
        render_fn: || view! { <DemoBgGradientInteractive /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Bento 3 Transition",
        render_fn: || view! { <DemoBento3Transition /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Leptos Struct Table",
        render_fn: || view! { <DemoLeptosStructTable_Simple /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Leptos Struct Table Tailwind",
        render_fn: || view! { <DemoLeptosStructTable_Tailwind /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Leptos Struct Table Editable",
        render_fn: || view! { <DemoLeptosStructTable_Editable /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Leptos Struct Table Selectable",
        render_fn: || view! { <DemoLeptosStructTable_Selectable /> }.into_any(),
    },
];
