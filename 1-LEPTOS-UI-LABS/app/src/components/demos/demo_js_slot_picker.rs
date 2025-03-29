use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::components::ui::slot_picker::{ButtonAdd, DayContainer, SlotDays, SlotTitle, TimeSlots};

#[component]
pub fn DemoJsSlotPicker() -> impl IntoView {
    view! {
        <Stylesheet id="slot-picker" href="/components/slot_picker.css" />
        <script src="/components/slot_picker.js" />

        <div class="mainDiv">
            <SlotDays>
                <div class="day" style="view-transition-name: day-1;">
                    <div class="day-header" style="view-transition-name: day-header-1;">
                        <SlotTitle>Monday</SlotTitle>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer>
                        <div class="time-slots" style="view-transition-name: time-slots-1;"></div>
                        <ButtonAdd>
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-2;">
                    <div class="day-header" style="view-transition-name: day-header-2;">
                        <SlotTitle>Tuesday</SlotTitle>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer>
                        <div class="time-slots" style="view-transition-name: time-slots-2;"></div>
                        <ButtonAdd>
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-3;">
                    <div class="day-header" style="view-transition-name: day-header-3;">
                        <SlotTitle>Wednesday</SlotTitle>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer>
                        <div class="time-slots" style="view-transition-name: time-slots-3;"></div>
                        <ButtonAdd>
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-4;">
                    <div class="day-header" style="view-transition-name: day-header-4;">
                        <SlotTitle>Thursday</SlotTitle>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer>
                        <TimeSlots />
                        <ButtonAdd>
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>

                <div class="day" style="view-transition-name: day-5;">
                    <div class="day-header" style="view-transition-name: day-header-5;">
                        <SlotTitle>Friday</SlotTitle>
                        <input type="checkbox" switch />
                    </div>
                    <DayContainer>
                        <TimeSlots />
                        <ButtonAdd>
                            <SvgIconPlus />
                            Add More
                        </ButtonAdd>
                    </DayContainer>
                </div>
            </SlotDays>
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
