use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Card, div, "rounded-lg border bg-card shadow", "p-4 w-full"}
    clx! {CardHeader, div, "flex flex-col space-y-1.5"}
    clx! {CardTitle, h3, "text-2xl font-semibold leading-none tracking-tight"}
    clx! {CardContent, div, "pt-4"}
    clx! {CardDescription, p, "text-sm", "text-muted-foreground"}
    clx! {CardFooter, footer, "mt-4", "flex items-center justify-end"}
}

pub use components::*;
