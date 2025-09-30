use icons::{AlignVerticalJustifyCenter, Blocks, Compass, Expand, PanelLeft, Search};
use leptos::prelude::*;

use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionLink, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordionIcons() -> impl IntoView {
    view! {
        <Accordion class="overflow-hidden rounded-lg border bg-background max-w-[200px]">
            <AccordionItem attr:open=true>
                <AccordionTrigger class="group-open:bg-accent hover:bg-accent">
                    <AccordionTitle>Registry</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="p-0">
                    <ul>
                        <li>
                            <AccordionLink attr:href="/docs/components">
                                <Blocks />
                                <span class="text-sm">Components</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/docs/extensions">
                                <Expand />
                                <span class="text-sm">Extensions</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/docs/hooks">
                                <Compass />
                                <span class="text-sm">Hooks</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/icons">
                                <Search />
                                <span class="text-sm">Icons</span>
                            </AccordionLink>
                        </li>
                    </ul>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger class="group-open:bg-accent hover:bg-accent">
                    <AccordionTitle>Blocks</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="p-0">
                    <ul>
                        <li>
                            <AccordionLink attr:href="/blocks/login">
                                <Blocks />
                                <span class="text-sm">Login</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/blocks/sidenav">
                                <PanelLeft />
                                <span class="text-sm">Sidenav</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/blocks/parallax">
                                <AlignVerticalJustifyCenter />
                                <span class="text-sm">Hooks</span>
                            </AccordionLink>
                        </li>
                    </ul>
                </AccordionContent>
            </AccordionItem>
            <AccordionLink class="p-3" attr:href="/themes">
                <AccordionTitle>Themes</AccordionTitle>
            </AccordionLink>
            <AccordionLink class="p-3" attr:href="/icons">
                <AccordionTitle>Icons</AccordionTitle>
            </AccordionLink>
        </Accordion>
    }
}
