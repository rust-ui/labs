use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::components::ui::slot_picker::{ButtonAdd, DayContainer};

#[component]
pub fn DemoJsSlotPicker() -> impl IntoView {
    view! {
        <Stylesheet id="slot-picker" href="/components/slot_picker.css" />
        <script src="/components/slot_picker.js" />

        <div class="mainDiv">
            <div class="days">
                <div class="day" style="view-transition-name: day-1;">
                    <div class="day-header" style="view-transition-name: day-header-1;">
                        <span class="day-title">Monday</span>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer style="view-transition-name: day-container-1;">
                        <div class="time-slots" style="view-transition-name: time-slots-1;"></div>
                        <ButtonAdd style="view-transition-name: add-btn-1;">
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-2;">
                    <div class="day-header" style="view-transition-name: day-header-2;">
                        <span class="day-title">Tuesday</span>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer style="view-transition-name: day-container-2;">
                        <div class="time-slots" style="view-transition-name: time-slots-2;"></div>
                        <ButtonAdd style="view-transition-name: add-btn-2;">
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-3;">
                    <div class="day-header" style="view-transition-name: day-header-3;">
                        <span class="day-title">Wednesday</span>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer style="view-transition-name: day-container-3;">
                        <div class="time-slots" style="view-transition-name: time-slots-3;"></div>
                        <ButtonAdd style="view-transition-name: add-btn-3;">
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-4;">
                    <div class="day-header" style="view-transition-name: day-header-4;">
                        <span class="day-title">Thursday</span>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer style="view-transition-name: day-container-4;">
                        <div class="time-slots"></div>
                        <ButtonAdd style="view-transition-name: add-btn-4;">
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-5;">
                    <div class="day-header" style="view-transition-name: day-header-5;">
                        <span class="day-title">Friday</span>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer style="view-transition-name: day-container-5;">
                        <div class="time-slots"></div>
                        <ButtonAdd style="view-transition-name: add-btn-5;">
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn SvgIconPlus() -> impl IntoView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path
                d="M8 2v12M2 8h12"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
            />
        </svg>
    }
}
