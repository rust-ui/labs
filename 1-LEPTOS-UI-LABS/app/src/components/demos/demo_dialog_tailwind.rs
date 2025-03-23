use leptos::prelude::*;

use crate::components::ui::{
    button::{Button, ButtonVariant},
    dialog_tailwind::{Dialog, DialogClose, DialogFooter, DialogTitle, DialogTrigger},
    form::{Form, FormField},
    input::Input,
};

#[component]
pub fn DemoDialogTailwind() -> impl IntoView {
    view! {
        <div class="p-5">
            <DialogTrigger>"Open modal"</DialogTrigger>

            <Dialog>
                <div class="flex flex-col gap-4">
                    <DialogTitle>"Reset your password"</DialogTitle>

                    <Form>
                        <FormField>
                            <label>"Email"</label>
                            <Input />
                        </FormField>

                        <DialogFooter>
                            <DialogClose variant=ButtonVariant::Outline>"Close"</DialogClose>
                            <Button>"Reset Password"</Button>
                        </DialogFooter>
                    </Form>

                </div>
            </Dialog>
        </div>
    }
}
