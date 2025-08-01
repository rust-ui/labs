use leptos::prelude::*;

use crate::shared::components::button::Button;

#[component]
pub fn DemoButtonMultiState() -> impl IntoView {
    view! {
        <style>
            {r#"
@layer demo {
    @media (prefers-reduced-motion: no-preference) {
      /* give the elements a name so they morph */
      button {
        view-transition-name: buy-btn;
      }
      
      /* remove aspect-ratio and opt into squishing */
      ::view-transition-old(buy-btn),
      ::view-transition-new(buy-btn) {
        height: 100%;
        width: 100%;
      }
    }
  }
  
  @layer demo.support {
    .mainDiv {
      display: grid;
      place-content: center;
      padding: var(--size-5);
      gap: var(--size-5);
    }
  }
            "#}
        </style>

        <div class="mainDiv">
            <Button id="demo">Do some hard work</Button>
        </div>

        <script src="/components/button_multi_state.js"></script>
    }
}
