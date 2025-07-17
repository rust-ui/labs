use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Card, div, "rounded-lg border bg-card shadow", "p-4 w-full"}
    clx! {CardTitle, h3, "text-xl font-semibold leading-none tracking-tight"}
    clx! {CardHeader, div, "flex flex-col space-y-1.5"}
    clx! {CardFooter, div, "mt-4", "flex items-center justify-end"}
    clx! {CardContent, div, "pt-4"}
    clx! {CardDescription, p, "text-sm", "text-muted-foreground"}
}

pub use components::*;
