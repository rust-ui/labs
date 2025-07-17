use leptos::prelude::*;

#[derive(Clone)]
struct ImageBox {
    id: &'static str,
    url: &'static str,
    value: &'static str,
}

#[component]
pub fn DemoBento3Transition() -> impl IntoView {
    let images = RwSignal::new(vec![
        ImageBox {
            id: "box-0",
            url: "https://images.unsplash.com/photo-1614348532352-c5dd2ad60420?crop=entropy&cs=srgb&fm=jpg&q=85",
            value: "box-0",
        },
        ImageBox {
            id: "box-1",
            url: "https://images.unsplash.com/photo-1566681682641-af84728ee122?crop=entropy&cs=srgb&fm=jpg&q=85",
            value: "box-1",
        },
        ImageBox {
            id: "box-2",
            url: "https://images.unsplash.com/photo-1616093266211-d98e19d1f5bc?crop=entropy&cs=srgb&fm=jpg&q=85",
            value: "box-2",
        },
    ]);

    let rotate_images = {
        move |_| {
            images.update(|imgs| {
                if let Some(last) = imgs.pop() {
                    imgs.insert(0, last);
                }
            });
        }
    };

    view! {
            <div class="flex justify-center items-center">

           <div class="grid grid-cols-2 grid-rows-2 gap-4 w-[50%] max-w-6xl auto-rows-[100px]">
        <input class="hidden peer/box-0" id="box-0" name="bento" type="radio" value="box-0" checked />
        <input class="hidden peer/box-1" id="box-1" name="bento" type="radio" value="box-1" />
        <input class="hidden peer/box-2" id="box-2" name="bento" type="radio" value="box-2" />

        <label
            on:click=rotate_images
            r#for="box-0"
            class="row-span-2 aspect-auto rounded-xl bg-cover bg-center transition-all duration-700 ease-in-out cursor-pointer"
            style=move || format!("background-image: url('{}');", images.get()[0].url)
        ></label>

        <label
            on:click=rotate_images
            r#for="box-1"
            class="aspect-square rounded-xl bg-cover bg-center transition-all duration-700 ease-in-out cursor-pointer"
            style=move || format!("background-image: url('{}');", images.get()[1].url)
        ></label>

        <label
            on:click=rotate_images
            r#for="box-2"
            class="aspect-square rounded-xl bg-cover bg-center transition-all duration-700 ease-in-out cursor-pointer"
            style=move || format!("background-image: url('{}');", images.get()[2].url)
        ></label>
    </div>
                </div>
        }
}
