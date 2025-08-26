use leptos::prelude::*;

pub fn use_lock_body_scroll(initial_locked: bool) -> RwSignal<bool> {
    let locked = RwSignal::new(initial_locked);

    Effect::new(move |_| {
        if locked.get() {
            window()
                .document()
                .expect("document not found")
                .body()
                .expect("body not found")
                .style()
                .set_property("overflow", "hidden")
                .expect("failed to set overflow");
        } else {
            window()
                .document()
                .expect("document not found")
                .body()
                .expect("body not found")
                .style()
                .remove_property("overflow")
                .expect("failed to remove overflow");
        }
    });

    locked
}
