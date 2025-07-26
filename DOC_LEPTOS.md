---
Creation date: 2025-07-26
Leptos version: 0.8.5
---

Estimated tokens: 46.2k

Directory structure:
└── leptos-rs-leptos/
    └── leptos/
        ├── build.rs
        ├── Cargo.toml
        ├── Makefile.toml
        ├── src/
        │   ├── animated_show.rs
        │   ├── attribute_interceptor.rs
        │   ├── await_.rs
        │   ├── callback.rs
        │   ├── children.rs
        │   ├── component.rs
        │   ├── error_boundary.rs
        │   ├── for_loop.rs
        │   ├── form.rs
        │   ├── from_form_data.rs
        │   ├── into_view.rs
        │   ├── lib.rs
        │   ├── logging.rs
        │   ├── mount.rs
        │   ├── nonce.rs
        │   ├── portal.rs
        │   ├── provider.rs
        │   ├── show.rs
        │   ├── suspense_component.rs
        │   ├── text_prop.rs
        │   ├── transition.rs
        │   └── hydration/
        │       ├── hydration_script.js
        │       ├── island_script.js
        │       ├── islands_routing.js
        │       ├── mod.rs
        │       └── reload_script.js



================================================
FILE: leptos/build.rs
================================================
use rustc_version::{version_meta, Channel};

fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    // Set cfg flags depending on release channel
    if matches!(version_meta().unwrap().channel, Channel::Nightly) {
        println!("cargo:rustc-cfg=rustc_nightly");
    }
    // Set cfg flag for getrandom wasm_js
    if target == "wasm32-unknown-unknown" {
        // Set a custom cfg flag for wasm builds
        println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
    }
}



================================================
FILE: leptos/Cargo.toml
================================================
[package]
name = "leptos"
version = { workspace = true }
authors = ["Greg Johnston"]
license = "MIT"
repository = "https://github.com/leptos-rs/leptos"
description = "Leptos is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces."
readme = "../README.md"
rust-version.workspace = true
edition.workspace = true

[dependencies]
throw_error = { workspace = true }
any_spawner = { workspace = true, features = [
  "wasm-bindgen",
  "futures-executor",
] }
base64 = { optional = true, workspace = true, default-features = true }
cfg-if = { workspace = true, default-features = true }
hydration_context = { workspace = true }
either_of = { workspace = true }
leptos_dom = { workspace = true }
leptos_hot_reload = { workspace = true }
leptos_macro = { workspace = true }
leptos_server = { workspace = true, features = ["tachys"] }
leptos_config = { workspace = true }
leptos-spin-macro = { optional = true, workspace = true, default-features = true }
oco_ref = { workspace = true }
or_poisoned = { workspace = true }
paste = { workspace = true, default-features = true }
rand = { optional = true, workspace = true, default-features = true }
# NOTE: While not used directly, `getrandom`'s `wasm_js` feature is needed when `rand` is used on WASM to
#       avoid a compilation error
getrandom = { optional = true, workspace = true, default-features = true }
reactive_graph = { workspace = true, features = ["serde"] }
rustc-hash = { workspace = true, default-features = true }
tachys = { workspace = true, features = [
  "reactive_graph",
  "reactive_stores",
  "oco",
] }
thiserror = { workspace = true, default-features = true }
tracing = { optional = true, workspace = true, default-features = true }
typed-builder = { workspace = true, default-features = true }
typed-builder-macro = { workspace = true, default-features = true }
serde = { workspace = true, default-features = true }
serde_json = { workspace = true, default-features = true }
server_fn = { workspace = true, features = ["form-redirects", "browser"] }
web-sys = { features = [
  "ShadowRoot",
  "ShadowRootInit",
  "ShadowRootMode",
], workspace = true, default-features = true }
wasm-bindgen = { workspace = true, default-features = true }
wasm-bindgen-futures = { workspace = true, default-features = true }
serde_qs = { workspace = true, default-features = true }
slotmap = { workspace = true, default-features = true }
futures = { workspace = true, default-features = true }
send_wrapper = { workspace = true, default-features = true }
wasm_split_helpers.workspace = true

[features]
hydration = [
  "reactive_graph/hydration",
  "leptos_server/hydration",
  "hydration_context/browser",
  "leptos_dom/hydration",
]
csr = ["leptos_macro/csr", "reactive_graph/effects", "getrandom?/wasm_js"]
hydrate = [
  "leptos_macro/hydrate",
  "hydration",
  "tachys/hydrate",
  "reactive_graph/effects",
  "getrandom?/wasm_js",
]
default-tls = ["server_fn/default-tls"]
rustls = ["server_fn/rustls"]
ssr = [
  "leptos_macro/ssr",
  "leptos_server/ssr",
  "server_fn/ssr",
  "hydration",
  "tachys/ssr",
]
nightly = ["leptos_macro/nightly", "reactive_graph/nightly", "tachys/nightly"]
rkyv = ["server_fn/rkyv", "leptos_server/rkyv"]
tracing = [
  "dep:tracing",
  "reactive_graph/tracing",
  "tachys/tracing",
  "leptos_macro/tracing",
  "leptos_dom/tracing",
  "leptos_server/tracing",
]
nonce = ["base64", "rand", "dep:getrandom"]
spin = ["leptos-spin-macro"]
islands = ["leptos_macro/islands"]
trace-component-props = [
  "leptos_macro/trace-component-props",
  "leptos_dom/trace-component-props",
]
delegation = ["tachys/delegation"]
islands-router = ["tachys/mark_branches"]

[dev-dependencies]
tokio = { features = [
  "rt-multi-thread",
  "macros",
], workspace = true, default-features = true }
tokio-test = { workspace = true, default-features = true }
any_spawner = { workspace = true, features = ["futures-executor", "tokio"] }

[build-dependencies]
rustc_version = { workspace = true, default-features = true }

# Having an erasure feature rather than normal --cfg erase_components for the proc macro crate is a workaround for this rust issue:
# https://github.com/rust-lang/cargo/issues/4423
# TLDR proc macros will ignore RUSTFLAGS when --target is specified on the cargo command.
# This works around the issue by the non proc-macro crate which does see RUSTFLAGS enabling the replacement feature on the proc-macro crate, which wouldn't.
# This is automatic as long as the leptos crate is depended upon,
# downstream usage should never manually enable this feature.
[target.'cfg(erase_components)'.dependencies]
leptos_macro = { workspace = true, features = ["__internal_erase_components"] }

[package.metadata.cargo-all-features]
denylist = [
  "tracing",
  "template_macro",
  "rustls",
  "default-tls",
  "wasm-bindgen",
  "rkyv",                  # was causing clippy issues on nightly
  "trace-component-props",
  "spin",
  "islands",
]
skip_feature_sets = [
  ["csr", "ssr"],
  ["csr", "hydrate"],
  ["ssr", "hydrate"],
  ["serde", "serde-lite"],
  ["serde-lite", "miniserde"],
  ["serde", "miniserde"],
  ["serde", "rkyv"],
  ["miniserde", "rkyv"],
  ["serde-lite", "rkyv"],
  ["default-tls", "rustls"],
  # do not test against nightly feature alone
  ["nightly"],
]
max_combination_size = 2

[package.metadata.docs.rs]
rustdoc-args = ["--generate-link-to-definition"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = [
  'cfg(leptos_debuginfo)',
  'cfg(rustc_nightly)',
] }



================================================
FILE: leptos/Makefile.toml
================================================
extend = { path = "../cargo-make/main.toml" }

[tasks.check]
clear = true
dependencies = [
  "clippy-each-feature",
  "check-wasm",
  "check-release",
  "check-wasm-release",
]

[tasks.check-wasm]
clear = true
dependencies = ["check-hydrate", "check-csr"]

[tasks.check-wasm-release]
clear = true
dependencies = ["check-hydrate-release", "check-csr-release"]

[tasks.check-hydrate]
command = "cargo"
args = [
  "check",
  "--no-default-features",
  "--features=hydrate",
  "--target=wasm32-unknown-unknown",
]

[tasks.check-csr]
command = "cargo"
args = [
  "check",
  "--no-default-features",
  "--features=csr",
  "--target=wasm32-unknown-unknown",
]

[tasks.check-hydrate-release]
command = "cargo"
args = [
  "check",
  "--release",
  "--no-default-features",
  "--features=hydrate",
  "--target=wasm32-unknown-unknown",
]

[tasks.check-csr-release]
command = "cargo"
args = [
  "check",
  "--release",
  "--no-default-features",
  "--features=csr",
  "--target=wasm32-unknown-unknown",
]

[tasks.check-release]
command = "cargo"
args = ["check", "--release"]



================================================
FILE: leptos/src/animated_show.rs
================================================
use crate::{children::ChildrenFn, component, control_flow::Show, IntoView};
use core::time::Duration;
use leptos_dom::helpers::TimeoutHandle;
use leptos_macro::view;
use reactive_graph::{
    effect::RenderEffect,
    owner::{on_cleanup, StoredValue},
    signal::RwSignal,
    traits::{Get, GetUntracked, GetValue, Set, SetValue},
    wrappers::read::Signal,
};
use tachys::prelude::*;

/// A component that will show its children when the `when` condition is `true`.
/// Additionally, you need to specify a `hide_delay`. If the `when` condition changes to `false`,
/// the unmounting of the children will be delayed by the specified Duration.
/// If you provide the optional `show_class` and `hide_class`, you can create very easy mount /
/// unmount animations.
///
/// ```rust
/// # use core::time::Duration;
/// # use leptos::prelude::*;
/// # #[component]
/// # pub fn App() -> impl IntoView {
/// let show = RwSignal::new(false);
///
/// view! {
///     <div
///         class="hover-me"
///         on:mouseenter=move |_| show.set(true)
///         on:mouseleave=move |_| show.set(false)
///     >
///         "Hover Me"
///     </div>
///
///     <AnimatedShow
///        when=show
///        show_class="fade-in-1000"
///        hide_class="fade-out-1000"
///        hide_delay=Duration::from_millis(1000)
///     >
///        <div class="here-i-am">
///            "Here I Am!"
///        </div>
///     </AnimatedShow>
/// }
/// # }
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[component]
pub fn AnimatedShow(
    /// The components Show wraps
    children: ChildrenFn,
    /// If the component should show or not
    #[prop(into)]
    when: Signal<bool>,
    /// Optional CSS class to apply if `when == true`
    #[prop(optional)]
    show_class: &'static str,
    /// Optional CSS class to apply if `when == false`
    #[prop(optional)]
    hide_class: &'static str,
    /// The timeout after which the component will be unmounted if `when == false`
    hide_delay: Duration,
) -> impl IntoView {
    let handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let cls = RwSignal::new(if when.get_untracked() {
        show_class
    } else {
        hide_class
    });
    let show = RwSignal::new(when.get_untracked());

    let eff = RenderEffect::new(move |_| {
        if when.get() {
            // clear any possibly active timer
            if let Some(h) = handle.get_value() {
                h.clear();
            }

            cls.set(show_class);
            show.set(true);
        } else {
            cls.set(hide_class);

            let h = leptos_dom::helpers::set_timeout_with_handle(
                move || show.set(false),
                hide_delay,
            )
            .expect("set timeout in AnimatedShow");
            handle.set_value(Some(h));
        }
    });

    on_cleanup(move || {
        if let Some(Some(h)) = handle.try_get_value() {
            h.clear();
        }
        drop(eff);
    });

    view! {
        <Show when=move || show.get() fallback=|| ()>
            <div class=move || cls.get()>{children()}</div>
        </Show>
    }
}



================================================
FILE: leptos/src/attribute_interceptor.rs
================================================
use crate::attr::{
    any_attribute::{AnyAttribute, IntoAnyAttribute},
    Attribute, NextAttribute,
};
use leptos::prelude::*;

/// Function stored to build/rebuild the wrapped children when attributes are added.
type ChildBuilder<T> = dyn Fn(AnyAttribute) -> T + Send + Sync + 'static;

/// Intercepts attributes passed to your component, allowing passing them to any element.
///
/// By default, Leptos passes any attributes passed to your component (e.g. `<MyComponent
/// attr:class="some-class"/>`) to the top-level element in the view returned by your component.
/// [`AttributeInterceptor`] allows you to intercept this behavior and pass it onto any element in
/// your component instead.
///
/// Must be the top level element in your component's view.
///
/// ## Example
///
/// Any attributes passed to MyComponent will be passed to the #inner element.
///
/// ```
/// # use leptos::prelude::*;
/// use leptos::attribute_interceptor::AttributeInterceptor;
///
/// #[component]
/// pub fn MyComponent() -> impl IntoView {
///     view! {
///         <AttributeInterceptor let:attrs>
///             <div id="wrapper">
///                 <div id="inner" {..attrs} />
///             </div>
///         </AttributeInterceptor>
///     }
/// }
/// ```
#[component(transparent)]
pub fn AttributeInterceptor<Chil, T>(
    /// The elements that will be rendered, with the attributes this component received as a
    /// parameter.
    children: Chil,
) -> impl IntoView
where
    Chil: Fn(AnyAttribute) -> T + Send + Sync + 'static,
    T: IntoView + 'static,
{
    AttributeInterceptorInner::new(children)
}

/// Wrapper to intercept attributes passed to a component so you can apply them to a different
/// element.
struct AttributeInterceptorInner<T: IntoView, A> {
    children_builder: Box<ChildBuilder<T>>,
    children: T,
    attributes: A,
}

impl<T: IntoView> AttributeInterceptorInner<T, ()> {
    /// Use this as the returned view from your component to collect the attributes that are passed
    /// to your component so you can manually handle them.
    pub fn new<F>(children: F) -> Self
    where
        F: Fn(AnyAttribute) -> T + Send + Sync + 'static,
    {
        let children_builder = Box::new(children);
        let children = children_builder(().into_any_attr());

        Self {
            children_builder,
            children,
            attributes: (),
        }
    }
}

impl<T: IntoView, A: Attribute> Render for AttributeInterceptorInner<T, A> {
    type State = <T as Render>::State;

    fn build(self) -> Self::State {
        self.children.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.children.rebuild(state);
    }
}

impl<T: IntoView + 'static, A> AddAnyAttr for AttributeInterceptorInner<T, A>
where
    A: Attribute,
{
    type Output<SomeNewAttr: leptos::attr::Attribute> =
        AttributeInterceptorInner<T, <<A as NextAttribute>::Output<SomeNewAttr> as Attribute>::CloneableOwned>;

    fn add_any_attr<NewAttr: leptos::attr::Attribute>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        let attributes =
            self.attributes.add_any_attr(attr).into_cloneable_owned();

        let children =
            (self.children_builder)(attributes.clone().into_any_attr());

        AttributeInterceptorInner {
            children_builder: self.children_builder,
            children,
            attributes,
        }
    }
}

impl<T: IntoView + 'static, A: Attribute> RenderHtml
    for AttributeInterceptorInner<T, A>
{
    type AsyncOutput = T::AsyncOutput;
    type Owned = AttributeInterceptorInner<T, A::CloneableOwned>;

    const MIN_LENGTH: usize = T::MIN_LENGTH;

    fn dry_resolve(&mut self) {
        self.children.dry_resolve()
    }

    fn resolve(
        self,
    ) -> impl std::future::Future<Output = Self::AsyncOutput> + Send {
        self.children.resolve()
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut leptos::tachys::view::Position,
        escape: bool,
        mark_branches: bool,
        _extra_attrs: Vec<AnyAttribute>,
    ) {
        self.children.to_html_with_buf(
            buf,
            position,
            escape,
            mark_branches,
            vec![],
        )
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &leptos::tachys::hydration::Cursor,
        position: &leptos::tachys::view::PositionState,
    ) -> Self::State {
        self.children.hydrate::<FROM_SERVER>(cursor, position)
    }

    async fn hydrate_async(
        self,
        cursor: &leptos::tachys::hydration::Cursor,
        position: &leptos::tachys::view::PositionState,
    ) -> Self::State {
        self.children.hydrate_async(cursor, position).await
    }

    fn into_owned(self) -> Self::Owned {
        AttributeInterceptorInner {
            children_builder: self.children_builder,
            children: self.children,
            attributes: self.attributes.into_cloneable_owned(),
        }
    }
}



================================================
FILE: leptos/src/await_.rs
================================================
use crate::{prelude::Suspend, suspense_component::Suspense, IntoView};
use leptos_macro::{component, view};
use leptos_server::ArcOnceResource;
use reactive_graph::prelude::ReadUntracked;
use serde::{de::DeserializeOwned, Serialize};

#[component]
/// Allows you to inline the data loading for an `async` block or
/// server function directly into your view. This is the equivalent of combining a
/// [`create_resource`] that only loads once (i.e., with a source signal `|| ()`) with
/// a [`Suspense`] with no `fallback`.
///
/// Adding `let:{variable name}` to the props makes the data available in the children
/// that variable name, when resolved.
/// ```
/// # use leptos::prelude::*;
/// # if false {
/// async fn fetch_monkeys(monkey: i32) -> i32 {
///     // do some expensive work
///     3
/// }
///
/// view! {
///     <Await
///         future=fetch_monkeys(3)
///         let:data
///     >
///         <p>{*data} " little monkeys, jumping on the bed."</p>
///     </Await>
/// }
/// # ;
/// # }
/// ```
pub fn Await<T, Fut, Chil, V>(
    /// A [`Future`](std::future::Future) that will the component will `.await`
    /// before rendering.
    future: Fut,
    /// If `true`, the component will create a blocking resource, preventing
    /// the HTML stream from returning anything before `future` has resolved.
    #[prop(optional)]
    blocking: bool,
    /// A function that takes a reference to the resolved data from the `future`
    /// renders a view.
    ///
    /// ## Syntax
    /// This can be passed in the `view` children of the `<Await/>` by using the
    /// `let:` syntax to specify the name for the data variable.
    ///
    /// ```rust
    /// # use leptos::prelude::*;
    /// # if false {
    /// # async fn fetch_monkeys(monkey: i32) -> i32 {
    /// #    3
    /// # }
    /// view! {
    ///     <Await
    ///         future=fetch_monkeys(3)
    ///         let:data
    ///     >
    ///         <p>{*data} " little monkeys, jumping on the bed."</p>
    ///     </Await>
    /// }
    /// # ;
    /// # }
    /// ```
    /// is the same as
    ///  ```rust
    /// # use leptos::prelude::*;
    /// # if false {
    /// # async fn fetch_monkeys(monkey: i32) -> i32 {
    /// #    3
    /// # }
    /// view! {
    ///     <Await
    ///         future=fetch_monkeys(3)
    ///         children=|data| view! {
    ///           <p>{*data} " little monkeys, jumping on the bed."</p>
    ///         }
    ///     />
    /// }
    /// # ;
    /// # }
    /// ```
    children: Chil,
) -> impl IntoView
where
    T: Send + Sync + Serialize + DeserializeOwned + 'static,
    Fut: std::future::Future<Output = T> + Send + 'static,
    Chil: FnOnce(&T) -> V + Send + 'static,
    V: IntoView + 'static,
{
    let res = ArcOnceResource::<T>::new_with_options(future, blocking);
    let ready = res.ready();

    view! {
        <Suspense fallback=|| ()>
            {Suspend::new(async move {
                ready.await;
                children(res.read_untracked().as_ref().unwrap())
            })}

        </Suspense>
    }
}



================================================
FILE: leptos/src/callback.rs
================================================
//! Callbacks define a standard way to store functions and closures. They are useful
//! for component properties, because they can be used to define optional callback functions,
//! which generic props don’t support.
//!
//! # Usage
//! Callbacks can be created manually from any function or closure, but the easiest way
//! to create them is to use `#[prop(into)]]` when defining a component.
//! ```
//! use leptos::prelude::*;
//!
//! #[component]
//! fn MyComponent(
//!     #[prop(into)] render_number: Callback<(i32,), String>,
//! ) -> impl IntoView {
//!     view! {
//!         <div>
//!             {render_number.run((1,))}
//!             // callbacks can be called multiple times
//!             {render_number.run((42,))}
//!         </div>
//!     }
//! }
//! // you can pass a closure directly as `render_number`
//! fn test() -> impl IntoView {
//!     view! {
//!         <MyComponent render_number=|x: i32| x.to_string()/>
//!     }
//! }
//! ```
//!
//! *Notes*:
//! - The `render_number` prop can receive any type that implements `Fn(i32) -> String`.
//! - Callbacks are most useful when you want optional generic props.
//! - All callbacks implement the [`Callable`](leptos::callback::Callable) trait, and can be invoked with `my_callback.run(input)`.
//! - The callback types implement [`Copy`], so they can easily be moved into and out of other closures, just like signals.
//!
//! # Types
//! This modules implements 2 callback types:
//! - [`Callback`](leptos::callback::Callback)
//! - [`UnsyncCallback`](leptos::callback::UnsyncCallback)
//!
//! Use `SyncCallback` if the function is not `Sync` and `Send`.

use reactive_graph::{
    owner::{LocalStorage, StoredValue},
    traits::{Dispose, WithValue},
};
use std::{fmt, rc::Rc, sync::Arc};

/// A wrapper trait for calling callbacks.
pub trait Callable<In: 'static, Out: 'static = ()> {
    /// calls the callback with the specified argument.
    ///
    /// Returns None if the callback has been disposed
    fn try_run(&self, input: In) -> Option<Out>;
    /// calls the callback with the specified argument.
    ///
    /// # Panics
    /// Panics if you try to run a callback that has been disposed
    fn run(&self, input: In) -> Out;
}

/// A callback type that is not required to be `Send + Sync`.
pub struct UnsyncCallback<In: 'static, Out: 'static = ()>(
    StoredValue<Rc<dyn Fn(In) -> Out>, LocalStorage>,
);

impl<In> fmt::Debug for UnsyncCallback<In> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_str("Callback")
    }
}

impl<In, Out> Copy for UnsyncCallback<In, Out> {}

impl<In, Out> Clone for UnsyncCallback<In, Out> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<In, Out> Dispose for UnsyncCallback<In, Out> {
    fn dispose(self) {
        self.0.dispose();
    }
}

impl<In, Out> UnsyncCallback<In, Out> {
    /// Creates a new callback from the given function.
    pub fn new<F>(f: F) -> UnsyncCallback<In, Out>
    where
        F: Fn(In) -> Out + 'static,
    {
        Self(StoredValue::new_local(Rc::new(f)))
    }

    /// Returns `true` if both callbacks wrap the same underlying function pointer.
    #[inline]
    pub fn matches(&self, other: &Self) -> bool {
        self.0.with_value(|self_value| {
            other
                .0
                .with_value(|other_value| Rc::ptr_eq(self_value, other_value))
        })
    }
}

impl<In: 'static, Out: 'static> Callable<In, Out> for UnsyncCallback<In, Out> {
    fn try_run(&self, input: In) -> Option<Out> {
        self.0.try_with_value(|fun| fun(input))
    }

    fn run(&self, input: In) -> Out {
        self.0.with_value(|fun| fun(input))
    }
}

macro_rules! impl_unsync_callable_from_fn {
    ($($arg:ident),*) => {
        impl<F, $($arg,)* T, Out> From<F> for UnsyncCallback<($($arg,)*), Out>
        where
            F: Fn($($arg),*) -> T + 'static,
            T: Into<Out> + 'static,
            $($arg: 'static,)*
        {
            fn from(f: F) -> Self {
                paste::paste!(
                    Self::new(move |($([<$arg:lower>],)*)| f($([<$arg:lower>]),*).into())
                )
            }
        }
    };
}

impl_unsync_callable_from_fn!();
impl_unsync_callable_from_fn!(P1);
impl_unsync_callable_from_fn!(P1, P2);
impl_unsync_callable_from_fn!(P1, P2, P3);
impl_unsync_callable_from_fn!(P1, P2, P3, P4);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5, P6);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_unsync_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_unsync_callable_from_fn!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12
);

/// Callbacks define a standard way to store functions and closures.
///
/// # Example
/// ```
/// # use leptos::prelude::*;
/// # use leptos::callback::{Callable, Callback};
/// #[component]
/// fn MyComponent(
///     #[prop(into)] render_number: Callback<(i32,), String>,
/// ) -> impl IntoView {
///     view! {
///         <div>
///             {render_number.run((42,))}
///         </div>
///     }
/// }
///
/// fn test() -> impl IntoView {
///     view! {
///         <MyComponent render_number=move |x: i32| x.to_string()/>
///     }
/// }
/// ```
pub struct Callback<In, Out = ()>(
    StoredValue<Arc<dyn Fn(In) -> Out + Send + Sync>>,
)
where
    In: 'static,
    Out: 'static;

impl<In, Out> fmt::Debug for Callback<In, Out> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_str("SyncCallback")
    }
}

impl<In, Out> Callable<In, Out> for Callback<In, Out> {
    fn try_run(&self, input: In) -> Option<Out> {
        self.0.try_with_value(|fun| fun(input))
    }

    fn run(&self, input: In) -> Out {
        self.0.with_value(|f| f(input))
    }
}

impl<In, Out> Clone for Callback<In, Out> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<In, Out> Dispose for Callback<In, Out> {
    fn dispose(self) {
        self.0.dispose();
    }
}

impl<In, Out> Copy for Callback<In, Out> {}

macro_rules! impl_callable_from_fn {
    ($($arg:ident),*) => {
        impl<F, $($arg,)* T, Out> From<F> for Callback<($($arg,)*), Out>
        where
            F: Fn($($arg),*) -> T + Send + Sync + 'static,
            T: Into<Out> + 'static,
            $($arg: Send + Sync + 'static,)*
        {
            fn from(f: F) -> Self {
                paste::paste!(
                    Self::new(move |($([<$arg:lower>],)*)| f($([<$arg:lower>]),*).into())
                )
            }
        }
    };
}

impl_callable_from_fn!();
impl_callable_from_fn!(P1);
impl_callable_from_fn!(P1, P2);
impl_callable_from_fn!(P1, P2, P3);
impl_callable_from_fn!(P1, P2, P3, P4);
impl_callable_from_fn!(P1, P2, P3, P4, P5);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_callable_from_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);

impl<In: 'static, Out: 'static> Callback<In, Out> {
    /// Creates a new callback from the given function.
    pub fn new<F>(fun: F) -> Self
    where
        F: Fn(In) -> Out + Send + Sync + 'static,
    {
        Self(StoredValue::new(Arc::new(fun)))
    }

    /// Returns `true` if both callbacks wrap the same underlying function pointer.
    #[inline]
    pub fn matches(&self, other: &Self) -> bool {
        self.0
            .try_with_value(|self_value| {
                other.0.try_with_value(|other_value| {
                    Arc::ptr_eq(self_value, other_value)
                })
            })
            .flatten()
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::Callable;
    use crate::callback::{Callback, UnsyncCallback};
    use reactive_graph::traits::Dispose;

    struct NoClone {}

    #[test]
    fn clone_callback() {
        let callback = Callback::new(move |_no_clone: NoClone| NoClone {});
        let _cloned = callback;
    }

    #[test]
    fn clone_unsync_callback() {
        let callback =
            UnsyncCallback::new(move |_no_clone: NoClone| NoClone {});
        let _cloned = callback;
    }

    #[test]
    fn runback_from() {
        let _callback: Callback<(), String> = (|| "test").into();
        let _callback: Callback<(i32, String), String> =
            (|num, s| format!("{num} {s}")).into();
    }

    #[test]
    fn sync_callback_from() {
        let _callback: UnsyncCallback<(), String> = (|| "test").into();
        let _callback: UnsyncCallback<(i32, String), String> =
            (|num, s| format!("{num} {s}")).into();
    }

    #[test]
    fn sync_callback_try_run() {
        let callback = Callback::new(move |arg| arg);
        assert_eq!(callback.try_run((0,)), Some((0,)));
        callback.dispose();
        assert_eq!(callback.try_run((0,)), None);
    }

    #[test]
    fn unsync_callback_try_run() {
        let callback = UnsyncCallback::new(move |arg| arg);
        assert_eq!(callback.try_run((0,)), Some((0,)));
        callback.dispose();
        assert_eq!(callback.try_run((0,)), None);
    }

    #[test]
    fn callback_matches_same() {
        let callback1 = Callback::new(|x: i32| x * 2);
        let callback2 = callback1;
        assert!(callback1.matches(&callback2));
    }

    #[test]
    fn callback_matches_different() {
        let callback1 = Callback::new(|x: i32| x * 2);
        let callback2 = Callback::new(|x: i32| x + 1);
        assert!(!callback1.matches(&callback2));
    }

    #[test]
    fn unsync_callback_matches_same() {
        let callback1 = UnsyncCallback::new(|x: i32| x * 2);
        let callback2 = callback1;
        assert!(callback1.matches(&callback2));
    }

    #[test]
    fn unsync_callback_matches_different() {
        let callback1 = UnsyncCallback::new(|x: i32| x * 2);
        let callback2 = UnsyncCallback::new(|x: i32| x + 1);
        assert!(!callback1.matches(&callback2));
    }
}



================================================
FILE: leptos/src/children.rs
================================================
use crate::into_view::{IntoView, View};
use std::{
    fmt::{self, Debug},
    sync::Arc,
};
use tachys::view::{
    any_view::{AnyView, IntoAny},
    fragment::{Fragment, IntoFragment},
    RenderHtml,
};

/// The most common type for the `children` property on components,
/// which can only be called once.
///
/// This does not support iterating over individual nodes within the children.
/// To iterate over children, use [`ChildrenFragment`].
pub type Children = Box<dyn FnOnce() -> AnyView + Send>;

/// A type for the `children` property on components that can be called only once,
/// and provides a collection of all the children passed to this component.
pub type ChildrenFragment = Box<dyn FnOnce() -> Fragment + Send>;

/// A type for the `children` property on components that can be called
/// more than once.
pub type ChildrenFn = Arc<dyn Fn() -> AnyView + Send + Sync>;

/// A type for the `children` property on components that can be called more than once,
/// and provides a collection of all the children passed to this component.
pub type ChildrenFragmentFn = Arc<dyn Fn() -> Fragment + Send>;

/// A type for the `children` property on components that can be called
/// more than once, but may mutate the children.
pub type ChildrenFnMut = Box<dyn FnMut() -> AnyView + Send>;

/// A type for the `children` property on components that can be called more than once,
/// but may mutate the children, and provides a collection of all the children
/// passed to this component.
pub type ChildrenFragmentMut = Box<dyn FnMut() -> Fragment + Send>;

// This is to still support components that accept `Box<dyn Fn() -> AnyView>` as a children.
type BoxedChildrenFn = Box<dyn Fn() -> AnyView + Send>;

/// This trait can be used when constructing a component that takes children without needing
/// to know exactly what children type the component expects. This is used internally by the
/// `view!` macro implementation, and can also be used explicitly when using the builder syntax.
///
///
/// Different component types take different types for their `children` prop, some of which cannot
/// be directly constructed. Using `ToChildren` allows the component user to pass children without
/// explicitly constructing the correct type.
///
/// ## Examples
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::p;
/// # use leptos::IntoView;
/// # use leptos_macro::component;
/// # use leptos::children::ToChildren;
/// use leptos::context::{Provider, ProviderProps};
/// use leptos::control_flow::{Show, ShowProps};
///
/// #[component]
/// fn App() -> impl IntoView {
///     (
///       Provider(
///         ProviderProps::builder()
///             .children(ToChildren::to_children(|| {
///                 p().child("Foo")
///             }))
///             // ...
///            .value("Foo")
///            .build(),
///        ),
///        Show(
///          ShowProps::builder()
///             .children(ToChildren::to_children(|| {
///                 p().child("Foo")
///             }))
///             // ...
///             .when(|| true)
///             .fallback(|| p().child("foo"))
///             .build(),
///        )
///     )
/// }
pub trait ToChildren<F> {
    /// Convert the provided type (generally a closure) to Self (generally a "children" type,
    /// e.g., [Children]). See the implementations to see exactly which input types are supported
    /// and which "children" type they are converted to.
    fn to_children(f: F) -> Self;
}

/// Compiler optimisation, can be used with certain type to avoid unique closures in the view!{} macro.
pub struct ChildrenOptContainer<T>(pub T);

impl<F, C> ToChildren<F> for Children
where
    F: FnOnce() -> C + Send + 'static,
    C: RenderHtml + Send + 'static,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Box::new(move || f().into_any())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for Children
where
    T: IntoAny + Send + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Box::new(move || t.0.into_any())
    }
}

impl<F, C> ToChildren<F> for ChildrenFn
where
    F: Fn() -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Arc::new(move || f().into_any())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for ChildrenFn
where
    T: IntoAny + Clone + Send + Sync + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Arc::new(move || t.0.clone().into_any())
    }
}

impl<F, C> ToChildren<F> for ChildrenFnMut
where
    F: Fn() -> C + Send + 'static,
    C: RenderHtml + Send + 'static,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Box::new(move || f().into_any())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for ChildrenFnMut
where
    T: IntoAny + Clone + Send + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Box::new(move || t.0.clone().into_any())
    }
}

impl<F, C> ToChildren<F> for BoxedChildrenFn
where
    F: Fn() -> C + Send + 'static,
    C: RenderHtml + Send + 'static,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Box::new(move || f().into_any())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for BoxedChildrenFn
where
    T: IntoAny + Clone + Send + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Box::new(move || t.0.clone().into_any())
    }
}

impl<F, C> ToChildren<F> for ChildrenFragment
where
    F: FnOnce() -> C + Send + 'static,
    C: IntoFragment,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Box::new(move || f().into_fragment())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for ChildrenFragment
where
    T: IntoAny + Send + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Box::new(move || Fragment::new(vec![t.0.into_any()]))
    }
}

impl<F, C> ToChildren<F> for ChildrenFragmentFn
where
    F: Fn() -> C + Send + 'static,
    C: IntoFragment,
{
    #[inline]
    fn to_children(f: F) -> Self {
        Arc::new(move || f().into_fragment())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for ChildrenFragmentFn
where
    T: IntoAny + Clone + Send + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Arc::new(move || Fragment::new(vec![t.0.clone().into_any()]))
    }
}

impl<F, C> ToChildren<F> for ChildrenFragmentMut
where
    F: FnMut() -> C + Send + 'static,
    C: IntoFragment,
{
    #[inline]
    fn to_children(mut f: F) -> Self {
        Box::new(move || f().into_fragment())
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for ChildrenFragmentMut
where
    T: IntoAny + Clone + Send + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        Box::new(move || Fragment::new(vec![t.0.clone().into_any()]))
    }
}

/// New-type wrapper for a function that returns a view with `From` and `Default` traits implemented
/// to enable optional props in for example `<Show>` and `<Suspense>`.
#[derive(Clone)]
pub struct ViewFn(Arc<dyn Fn() -> AnyView + Send + Sync + 'static>);

impl Default for ViewFn {
    fn default() -> Self {
        Self(Arc::new(|| ().into_any()))
    }
}

impl<F, C> From<F> for ViewFn
where
    F: Fn() -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
{
    fn from(value: F) -> Self {
        Self(Arc::new(move || value().into_any()))
    }
}

impl ViewFn {
    /// Execute the wrapped function
    pub fn run(&self) -> AnyView {
        (self.0)()
    }
}

/// New-type wrapper for a function, which will only be called once and returns a view with `From` and
/// `Default` traits implemented to enable optional props in for example `<Show>` and `<Suspense>`.
pub struct ViewFnOnce(Box<dyn FnOnce() -> AnyView + Send + 'static>);

impl Default for ViewFnOnce {
    fn default() -> Self {
        Self(Box::new(|| ().into_any()))
    }
}

impl<F, C> From<F> for ViewFnOnce
where
    F: FnOnce() -> C + Send + 'static,
    C: RenderHtml + Send + 'static,
{
    fn from(value: F) -> Self {
        Self(Box::new(move || value().into_any()))
    }
}

impl ViewFnOnce {
    /// Execute the wrapped function
    pub fn run(self) -> AnyView {
        (self.0)()
    }
}

/// A typed equivalent to [`Children`], which takes a generic but preserves type information to
/// allow the compiler to optimize the view more effectively.
pub struct TypedChildren<T>(Box<dyn FnOnce() -> View<T> + Send>);

impl<T> TypedChildren<T> {
    /// Extracts the inner `children` function.
    pub fn into_inner(self) -> impl FnOnce() -> View<T> + Send {
        self.0
    }
}

impl<F, C> ToChildren<F> for TypedChildren<C>
where
    F: FnOnce() -> C + Send + 'static,
    C: IntoView,
    C::AsyncOutput: Send,
{
    #[inline]
    fn to_children(f: F) -> Self {
        TypedChildren(Box::new(move || f().into_view()))
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for TypedChildren<T>
where
    T: IntoView + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        TypedChildren(Box::new(move || t.0.into_view()))
    }
}

/// A typed equivalent to [`ChildrenFnMut`], which takes a generic but preserves type information to
/// allow the compiler to optimize the view more effectively.
pub struct TypedChildrenMut<T>(Box<dyn FnMut() -> View<T> + Send>);

impl<T> Debug for TypedChildrenMut<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypedChildrenMut").finish()
    }
}

impl<T> TypedChildrenMut<T> {
    /// Extracts the inner `children` function.
    pub fn into_inner(self) -> impl FnMut() -> View<T> + Send {
        self.0
    }
}

impl<F, C> ToChildren<F> for TypedChildrenMut<C>
where
    F: FnMut() -> C + Send + 'static,
    C: IntoView,
    C::AsyncOutput: Send,
{
    #[inline]
    fn to_children(mut f: F) -> Self {
        TypedChildrenMut(Box::new(move || f().into_view()))
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for TypedChildrenMut<T>
where
    T: IntoView + Clone + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        TypedChildrenMut(Box::new(move || t.0.clone().into_view()))
    }
}

/// A typed equivalent to [`ChildrenFn`], which takes a generic but preserves type information to
/// allow the compiler to optimize the view more effectively.
pub struct TypedChildrenFn<T>(Arc<dyn Fn() -> View<T> + Send + Sync>);

impl<T> Debug for TypedChildrenFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypedChildrenFn").finish()
    }
}

impl<T> Clone for TypedChildrenFn<T> {
    // Manual implementation to avoid the `T: Clone` bound.
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> TypedChildrenFn<T> {
    /// Extracts the inner `children` function.
    pub fn into_inner(self) -> Arc<dyn Fn() -> View<T> + Send + Sync> {
        self.0
    }
}

impl<F, C> ToChildren<F> for TypedChildrenFn<C>
where
    F: Fn() -> C + Send + Sync + 'static,
    C: IntoView,
    C::AsyncOutput: Send,
{
    #[inline]
    fn to_children(f: F) -> Self {
        TypedChildrenFn(Arc::new(move || f().into_view()))
    }
}

impl<T> ToChildren<ChildrenOptContainer<T>> for TypedChildrenFn<T>
where
    T: IntoView + Clone + Sync + 'static,
{
    #[inline]
    fn to_children(t: ChildrenOptContainer<T>) -> Self {
        TypedChildrenFn(Arc::new(move || t.0.clone().into_view()))
    }
}



================================================
FILE: leptos/src/component.rs
================================================
//! Utility traits and functions that allow building components,
//! as either functions of their props or functions with no arguments,
//! without knowing the name of the props struct.

pub trait Component<P> {}

pub trait Props {
    type Builder;

    fn builder() -> Self::Builder;
}

#[doc(hidden)]
pub trait PropsOrNoPropsBuilder {
    type Builder;

    fn builder_or_not() -> Self::Builder;
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Default)]
pub struct EmptyPropsBuilder {}

impl EmptyPropsBuilder {
    pub fn build(self) {}
}

impl<P: Props> PropsOrNoPropsBuilder for P {
    type Builder = <P as Props>::Builder;

    fn builder_or_not() -> Self::Builder {
        Self::builder()
    }
}

impl PropsOrNoPropsBuilder for EmptyPropsBuilder {
    type Builder = EmptyPropsBuilder;

    fn builder_or_not() -> Self::Builder {
        EmptyPropsBuilder {}
    }
}

impl<F, R> Component<EmptyPropsBuilder> for F where F: FnOnce() -> R {}

impl<P, F, R> Component<P> for F
where
    F: FnOnce(P) -> R,
    P: Props,
{
}

pub fn component_props_builder<P: PropsOrNoPropsBuilder>(
    _f: &impl Component<P>,
) -> <P as PropsOrNoPropsBuilder>::Builder {
    <P as PropsOrNoPropsBuilder>::builder_or_not()
}

pub fn component_view<P, T>(f: impl ComponentConstructor<P, T>, props: P) -> T {
    f.construct(props)
}
pub trait ComponentConstructor<P, T> {
    fn construct(self, props: P) -> T;
}

impl<Func, T> ComponentConstructor<(), T> for Func
where
    Func: FnOnce() -> T,
{
    fn construct(self, (): ()) -> T {
        (self)()
    }
}

impl<Func, T, P> ComponentConstructor<P, T> for Func
where
    Func: FnOnce(P) -> T,
    P: PropsOrNoPropsBuilder,
{
    fn construct(self, props: P) -> T {
        (self)(props)
    }
}



================================================
FILE: leptos/src/error_boundary.rs
================================================
use crate::{children::TypedChildren, IntoView};
use futures::{channel::oneshot, future::join_all};
use hydration_context::{SerializedDataId, SharedContext};
use leptos_macro::component;
use or_poisoned::OrPoisoned;
use reactive_graph::{
    computed::ArcMemo,
    effect::RenderEffect,
    owner::{provide_context, ArcStoredValue, Owner},
    signal::ArcRwSignal,
    traits::{Get, Update, With, WithUntracked, WriteValue},
};
use rustc_hash::FxHashMap;
use std::{
    collections::VecDeque,
    fmt::Debug,
    mem,
    sync::{Arc, Mutex},
};
use tachys::{
    html::attribute::{any_attribute::AnyAttribute, Attribute},
    hydration::Cursor,
    reactive_graph::OwnedView,
    ssr::{StreamBuilder, StreamChunk},
    view::{
        add_attr::AddAnyAttr, Mountable, Position, PositionState, Render,
        RenderHtml,
    },
};
use throw_error::{Error, ErrorHook, ErrorId};

/// When you render a `Result<_, _>` in your view, in the `Err` case it will
/// render nothing, and search up through the view tree for an `<ErrorBoundary/>`.
/// This component lets you define a fallback that should be rendered in that
/// error case, allowing you to handle errors within a section of the interface.
///
/// ```
/// # use leptos::prelude::*;
/// #[component]
/// pub fn ErrorBoundaryExample() -> impl IntoView {
///   let (value, set_value) = signal(Ok(0));
///   let on_input =
///     move |ev| set_value.set(event_target_value(&ev).parse::<i32>());
///
///   view! {
///     <input type="text" on:input=on_input/>
///     <ErrorBoundary
///       fallback=move |_| view! { <p class="error">"Enter a valid number."</p>}
///     >
///       <p>"Value is: " {move || value.get()}</p>
///     </ErrorBoundary>
///   }
/// }
/// ```
///
/// ## Beginner's Tip: ErrorBoundary Requires Your Error To Implement std::error::Error.
/// `ErrorBoundary` requires your `Result<T,E>` to implement [IntoView](https://docs.rs/leptos/latest/leptos/trait.IntoView.html).
/// `Result<T,E>` only implements `IntoView` if `E` implements [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html).
/// So, for instance, if you pass a `Result<T,String>` where `T` implements [IntoView](https://docs.rs/leptos/latest/leptos/trait.IntoView.html)
/// and attempt to render the error for the purposes of `ErrorBoundary` you'll get a compiler error like this.
///
/// ```rust,ignore
/// error[E0599]: the method `into_view` exists for enum `Result<ViewableLoginFlow, String>`, but its trait bounds were not satisfied
///    --> src/login.rs:229:32
///     |
/// 229 |                     err => err.into_view(),
///     |                                ^^^^^^^^^ method cannot be called on `Result<ViewableLoginFlow, String>` due to unsatisfied trait bounds
///     |
///     = note: the following trait bounds were not satisfied:
///             `<&Result<ViewableLoginFlow, std::string::String> as FnOnce<()>>::Output = _`
///             which is required by `&Result<ViewableLoginFlow, std::string::String>: leptos::IntoView`
///    ... more notes here ...
/// ```
///
/// For more information about how to easily implement `Error` see
/// [thiserror](https://docs.rs/thiserror/latest/thiserror/)
#[component]
pub fn ErrorBoundary<FalFn, Fal, Chil>(
    /// The elements that will be rendered, which may include one or more `Result<_>` types.
    children: TypedChildren<Chil>,
    /// A fallback that will be shown if an error occurs.
    fallback: FalFn,
) -> impl IntoView
where
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: IntoView + Send + 'static,
    Chil: IntoView + Send + 'static,
{
    let sc = Owner::current_shared_context();
    let boundary_id = sc.as_ref().map(|sc| sc.next_id()).unwrap_or_default();
    let initial_errors =
        sc.map(|sc| sc.errors(&boundary_id)).unwrap_or_default();

    let hook = Arc::new(ErrorBoundaryErrorHook::new(
        boundary_id.clone(),
        initial_errors,
    ));
    let errors = hook.errors.clone();
    let errors_empty = ArcMemo::new({
        let errors = errors.clone();
        move |_| errors.with(|map| map.is_empty())
    });
    let hook = hook as Arc<dyn ErrorHook>;

    let _guard = throw_error::set_error_hook(Arc::clone(&hook));
    let suspended_children = ErrorBoundarySuspendedChildren::default();

    let owner = Owner::new();
    let children = owner.with(|| {
        provide_context(Arc::clone(&hook));
        provide_context(suspended_children.clone());
        children.into_inner()()
    });

    OwnedView::new_with_owner(
        ErrorBoundaryView {
            hook,
            boundary_id,
            errors_empty,
            children,
            errors,
            fallback,
            suspended_children,
        },
        owner,
    )
}

pub(crate) type ErrorBoundarySuspendedChildren =
    ArcStoredValue<Vec<oneshot::Receiver<()>>>;

struct ErrorBoundaryView<Chil, FalFn> {
    hook: Arc<dyn ErrorHook>,
    boundary_id: SerializedDataId,
    errors_empty: ArcMemo<bool>,
    children: Chil,
    fallback: FalFn,
    errors: ArcRwSignal<Errors>,
    suspended_children: ErrorBoundarySuspendedChildren,
}

struct ErrorBoundaryViewState<Chil, Fal> {
    // the children are always present; we toggle between them and the fallback as needed
    children: Chil,
    fallback: Option<Fal>,
}

impl<Chil, Fal> Mountable for ErrorBoundaryViewState<Chil, Fal>
where
    Chil: Mountable,
    Fal: Mountable,
{
    fn unmount(&mut self) {
        if let Some(fallback) = &mut self.fallback {
            fallback.unmount();
        } else {
            self.children.unmount();
        }
    }

    fn mount(
        &mut self,
        parent: &tachys::renderer::types::Element,
        marker: Option<&tachys::renderer::types::Node>,
    ) {
        if let Some(fallback) = &mut self.fallback {
            fallback.mount(parent, marker);
        } else {
            self.children.mount(parent, marker);
        }
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        if let Some(fallback) = &self.fallback {
            fallback.insert_before_this(child)
        } else {
            self.children.insert_before_this(child)
        }
    }

    fn elements(&self) -> Vec<tachys::renderer::types::Element> {
        if let Some(fallback) = &self.fallback {
            fallback.elements()
        } else {
            self.children.elements()
        }
    }
}

impl<Chil, FalFn, Fal> Render for ErrorBoundaryView<Chil, FalFn>
where
    Chil: Render + 'static,
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: Render + 'static,
{
    type State = RenderEffect<ErrorBoundaryViewState<Chil::State, Fal::State>>;

    fn build(mut self) -> Self::State {
        let hook = Arc::clone(&self.hook);
        let _hook = throw_error::set_error_hook(Arc::clone(&hook));
        let mut children = Some(self.children.build());
        RenderEffect::new(
            move |prev: Option<
                ErrorBoundaryViewState<Chil::State, Fal::State>,
            >| {
                let _hook = throw_error::set_error_hook(Arc::clone(&hook));
                if let Some(mut state) = prev {
                    match (self.errors_empty.get(), &mut state.fallback) {
                        // no errors, and was showing fallback
                        (true, Some(fallback)) => {
                            fallback.insert_before_this(&mut state.children);
                            fallback.unmount();
                            state.fallback = None;
                        }
                        // yes errors, and was showing children
                        (false, None) => {
                            state.fallback = Some(
                                (self.fallback)(self.errors.clone()).build(),
                            );
                            state
                                .children
                                .insert_before_this(&mut state.fallback);
                            state.children.unmount();
                        }
                        // either there were no errors, and we were already showing the children
                        // or there are errors, but we were already showing the fallback
                        // in either case, rebuilding doesn't require us to do anything
                        _ => {}
                    }
                    state
                } else {
                    let fallback = (!self.errors_empty.get())
                        .then(|| (self.fallback)(self.errors.clone()).build());
                    ErrorBoundaryViewState {
                        children: children.take().unwrap(),
                        fallback,
                    }
                }
            },
        )
    }

    fn rebuild(self, state: &mut Self::State) {
        let new = self.build();
        let mut old = std::mem::replace(state, new);
        old.insert_before_this(state);
        old.unmount();
    }
}

impl<Chil, FalFn, Fal> AddAnyAttr for ErrorBoundaryView<Chil, FalFn>
where
    Chil: RenderHtml + 'static,
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: RenderHtml + Send + 'static,
{
    type Output<SomeNewAttr: Attribute> =
        ErrorBoundaryView<Chil::Output<SomeNewAttr::CloneableOwned>, FalFn>;

    fn add_any_attr<NewAttr: Attribute>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        let ErrorBoundaryView {
            hook,
            boundary_id,
            errors_empty,
            children,
            fallback,
            errors,
            suspended_children,
        } = self;
        ErrorBoundaryView {
            hook,
            boundary_id,
            errors_empty,
            children: children.add_any_attr(attr.into_cloneable_owned()),
            fallback,
            errors,
            suspended_children,
        }
    }
}

impl<Chil, FalFn, Fal> RenderHtml for ErrorBoundaryView<Chil, FalFn>
where
    Chil: RenderHtml + Send + 'static,
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: RenderHtml + Send + 'static,
{
    type AsyncOutput = ErrorBoundaryView<Chil::AsyncOutput, FalFn>;
    type Owned = Self;

    const MIN_LENGTH: usize = Chil::MIN_LENGTH;

    fn dry_resolve(&mut self) {
        self.children.dry_resolve();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        let ErrorBoundaryView {
            hook,
            boundary_id,
            errors_empty,
            children,
            fallback,
            errors,
            suspended_children,
            ..
        } = self;
        ErrorBoundaryView {
            hook,
            boundary_id,
            errors_empty,
            children: children.resolve().await,
            fallback,
            errors,
            suspended_children,
        }
    }

    fn to_html_with_buf(
        mut self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        // first, attempt to serialize the children to HTML, then check for errors
        let _hook = throw_error::set_error_hook(self.hook);
        let mut new_buf = String::with_capacity(Chil::MIN_LENGTH);
        let mut new_pos = *position;
        self.children.to_html_with_buf(
            &mut new_buf,
            &mut new_pos,
            escape,
            mark_branches,
            extra_attrs.clone(),
        );

        // any thrown errors would've been caught here
        if self.errors.with_untracked(|map| map.is_empty()) {
            buf.push_str(&new_buf);
        } else {
            // otherwise, serialize the fallback instead
            (self.fallback)(self.errors).to_html_with_buf(
                buf,
                position,
                escape,
                mark_branches,
                extra_attrs,
            );
        }
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        mut self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) where
        Self: Sized,
    {
        let _hook = throw_error::set_error_hook(Arc::clone(&self.hook));

        // first, attempt to serialize the children to HTML, then check for errors
        let mut new_buf = StreamBuilder::new(buf.clone_id());
        let mut new_pos = *position;
        self.children.to_html_async_with_buf::<OUT_OF_ORDER>(
            &mut new_buf,
            &mut new_pos,
            escape,
            mark_branches,
            extra_attrs.clone(),
        );

        let suspense_children =
            mem::take(&mut *self.suspended_children.write_value());

        // not waiting for any suspended children: just render
        if suspense_children.is_empty() {
            // any thrown errors would've been caught here
            if self.errors.with_untracked(|map| map.is_empty()) {
                buf.append(new_buf);
            } else {
                // otherwise, serialize the fallback instead
                let mut fallback = String::with_capacity(Fal::MIN_LENGTH);
                (self.fallback)(self.errors).to_html_with_buf(
                    &mut fallback,
                    position,
                    escape,
                    mark_branches,
                    extra_attrs,
                );
                buf.push_sync(&fallback);
            }
        } else {
            let mut position = *position;
            // if we're waiting for suspended children, we'll first wait for them to load
            // in this implementation, an ErrorBoundary that *contains* Suspense essentially acts
            // like a Suspense: it will wait for (all top-level) child Suspense to load before rendering anything
            let mut view_buf = StreamBuilder::new(new_buf.clone_id());
            view_buf.next_id();
            let hook = Arc::clone(&self.hook);
            view_buf.push_async(async move {
                let _hook = throw_error::set_error_hook(Arc::clone(&hook));
                let _ = join_all(suspense_children).await;

                let mut my_chunks = VecDeque::new();
                for chunk in new_buf.take_chunks() {
                    match chunk {
                        StreamChunk::Sync(data) => {
                            my_chunks.push_back(StreamChunk::Sync(data))
                        }
                        StreamChunk::Async { chunks } => {
                            let chunks = chunks.await;
                            my_chunks.extend(chunks);
                        }
                        StreamChunk::OutOfOrder { chunks } => {
                            let chunks = chunks.await;
                            my_chunks.push_back(StreamChunk::OutOfOrder {
                                chunks: Box::pin(async move { chunks }),
                            });
                        }
                    }
                }

                if self.errors.with_untracked(|map| map.is_empty()) {
                    // if no errors, just go ahead with the stream
                    my_chunks
                } else {
                    // otherwise, serialize the fallback instead
                    let mut fallback = String::with_capacity(Fal::MIN_LENGTH);
                    (self.fallback)(self.errors).to_html_with_buf(
                        &mut fallback,
                        &mut position,
                        escape,
                        mark_branches,
                        extra_attrs,
                    );
                    my_chunks.clear();
                    my_chunks.push_back(StreamChunk::Sync(fallback));
                    my_chunks
                }
            });
            buf.append(view_buf);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        mut self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let mut children = Some(self.children);
        let hook = Arc::clone(&self.hook);
        let cursor = cursor.to_owned();
        let position = position.to_owned();
        RenderEffect::new(
            move |prev: Option<
                ErrorBoundaryViewState<Chil::State, Fal::State>,
            >| {
                let _hook = throw_error::set_error_hook(Arc::clone(&hook));
                if let Some(mut state) = prev {
                    match (self.errors_empty.get(), &mut state.fallback) {
                        // no errors, and was showing fallback
                        (true, Some(fallback)) => {
                            fallback.insert_before_this(&mut state.children);
                            state.fallback.unmount();
                            state.fallback = None;
                        }
                        // yes errors, and was showing children
                        (false, None) => {
                            state.fallback = Some(
                                (self.fallback)(self.errors.clone()).build(),
                            );
                            state
                                .children
                                .insert_before_this(&mut state.fallback);
                            state.children.unmount();
                        }
                        // either there were no errors, and we were already showing the children
                        // or there are errors, but we were already showing the fallback
                        // in either case, rebuilding doesn't require us to do anything
                        _ => {}
                    }
                    state
                } else {
                    let children = children.take().unwrap();
                    let (children, fallback) = if self.errors_empty.get() {
                        (
                            children.hydrate::<FROM_SERVER>(&cursor, &position),
                            None,
                        )
                    } else {
                        (
                            children.build(),
                            Some(
                                (self.fallback)(self.errors.clone())
                                    .hydrate::<FROM_SERVER>(&cursor, &position),
                            ),
                        )
                    };

                    ErrorBoundaryViewState { children, fallback }
                }
            },
        )
    }

    async fn hydrate_async(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let mut children = Some(self.children);
        let hook = Arc::clone(&self.hook);
        let cursor = cursor.to_owned();
        let position = position.to_owned();

        let fallback_fn = Arc::new(Mutex::new(self.fallback));
        let initial = {
            let errors_empty = self.errors_empty.clone();
            let errors = self.errors.clone();
            let fallback_fn = Arc::clone(&fallback_fn);
            async move {
                let children = children.take().unwrap();
                let (children, fallback) = if errors_empty.get() {
                    (children.hydrate_async(&cursor, &position).await, None)
                } else {
                    let children = children.build();
                    let fallback =
                        (fallback_fn.lock().or_poisoned())(errors.clone());
                    let fallback =
                        fallback.hydrate_async(&cursor, &position).await;
                    (children, Some(fallback))
                };

                ErrorBoundaryViewState { children, fallback }
            }
        };

        RenderEffect::new_with_async_value(
            move |prev: Option<
                ErrorBoundaryViewState<Chil::State, Fal::State>,
            >| {
                let _hook = throw_error::set_error_hook(Arc::clone(&hook));
                if let Some(mut state) = prev {
                    match (self.errors_empty.get(), &mut state.fallback) {
                        // no errors, and was showing fallback
                        (true, Some(fallback)) => {
                            fallback.insert_before_this(&mut state.children);
                            state.fallback.unmount();
                            state.fallback = None;
                        }
                        // yes errors, and was showing children
                        (false, None) => {
                            state.fallback = Some(
                                (fallback_fn.lock().or_poisoned())(
                                    self.errors.clone(),
                                )
                                .build(),
                            );
                            state
                                .children
                                .insert_before_this(&mut state.fallback);
                            state.children.unmount();
                        }
                        // either there were no errors, and we were already showing the children
                        // or there are errors, but we were already showing the fallback
                        // in either case, rebuilding doesn't require us to do anything
                        _ => {}
                    }
                    state
                } else {
                    unreachable!()
                }
            },
            initial,
        )
        .await
    }

    fn into_owned(self) -> Self::Owned {
        self
    }
}

#[derive(Debug)]
struct ErrorBoundaryErrorHook {
    errors: ArcRwSignal<Errors>,
    id: SerializedDataId,
    shared_context: Option<Arc<dyn SharedContext + Send + Sync>>,
}

impl ErrorBoundaryErrorHook {
    pub fn new(
        id: SerializedDataId,
        initial_errors: impl IntoIterator<Item = (ErrorId, Error)>,
    ) -> Self {
        Self {
            errors: ArcRwSignal::new(Errors(
                initial_errors.into_iter().collect(),
            )),
            id,
            shared_context: Owner::current_shared_context(),
        }
    }
}

impl ErrorHook for ErrorBoundaryErrorHook {
    fn throw(&self, error: Error) -> ErrorId {
        // generate a unique ID
        let key: ErrorId = Owner::current_shared_context()
            .map(|sc| sc.next_id())
            .unwrap_or_default()
            .into();

        // register it with the shared context, so that it can be serialized from server to client
        // as needed
        if let Some(sc) = &self.shared_context {
            sc.register_error(self.id.clone(), key.clone(), error.clone());
        }

        // add it to the reactive map of errors
        self.errors.update(|map| {
            map.insert(key.clone(), error);
        });

        // return the key, which will be owned by the Result being rendered and can be used to
        // unregister this error if it is rebuilt
        key
    }

    fn clear(&self, id: &throw_error::ErrorId) {
        self.errors.update(|map| {
            map.remove(id);
        });
    }
}

/// A struct to hold all the possible errors that could be provided by child Views
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct Errors(FxHashMap<ErrorId, Error>);

impl Errors {
    /// Returns `true` if there are no errors.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add an error to Errors that will be processed by `<ErrorBoundary/>`
    pub fn insert<E>(&mut self, key: ErrorId, error: E)
    where
        E: Into<Error>,
    {
        self.0.insert(key, error.into());
    }

    /// Add an error with the default key for errors outside the reactive system
    pub fn insert_with_default_key<E>(&mut self, error: E)
    where
        E: Into<Error>,
    {
        self.0.insert(Default::default(), error.into());
    }

    /// Remove an error to Errors that will be processed by `<ErrorBoundary/>`
    pub fn remove(&mut self, key: &ErrorId) -> Option<Error> {
        self.0.remove(key)
    }

    /// An iterator over all the errors, in arbitrary order.
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl IntoIterator for Errors {
    type Item = (ErrorId, Error);
    type IntoIter = IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

/// An owning iterator over all the errors contained in the [`Errors`] struct.
#[repr(transparent)]
pub struct IntoIter(std::collections::hash_map::IntoIter<ErrorId, Error>);

impl Iterator for IntoIter {
    type Item = (ErrorId, Error);

    #[inline(always)]
    fn next(
        &mut self,
    ) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        self.0.next()
    }
}

/// An iterator over all the errors contained in the [`Errors`] struct.
#[repr(transparent)]
pub struct Iter<'a>(std::collections::hash_map::Iter<'a, ErrorId, Error>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a ErrorId, &'a Error);

    #[inline(always)]
    fn next(
        &mut self,
    ) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        self.0.next()
    }
}



================================================
FILE: leptos/src/for_loop.rs
================================================
use crate::into_view::IntoView;
use leptos_macro::component;
use reactive_graph::{
    owner::Owner,
    signal::{ArcRwSignal, ReadSignal},
    traits::Set,
};
use std::hash::Hash;
use tachys::{
    reactive_graph::OwnedView,
    view::keyed::{keyed, SerializableKey},
};

/// Iterates over children and displays them, keyed by the `key` function given.
///
/// This is much more efficient than naively iterating over nodes with `.iter().map(|n| view! { ... })...`,
/// as it avoids re-creating DOM nodes that are not being changed.
///
/// ```
/// # use leptos::prelude::*;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// struct Counter {
///   id: usize,
///   count: RwSignal<i32>
/// }
///
/// #[component]
/// fn Counters() -> impl IntoView {
///   let (counters, set_counters) = create_signal::<Vec<Counter>>(vec![]);
///
///   view! {
///     <div>
///       <For
///         // a function that returns the items we're iterating over; a signal is fine
///         each=move || counters.get()
///         // a unique key for each item
///         key=|counter| counter.id
///         // renders each item to a view
///         children=move |counter: Counter| {
///           view! {
///             <button>"Value: " {move || counter.count.get()}</button>
///           }
///         }
///       />
///     </div>
///   }
/// }
/// ```
///
/// For convenience, you can also choose to write template code directly in the `<For>`
/// component, using the `let` syntax:
///
/// ```
/// # use leptos::prelude::*;
///
/// # #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// # struct Counter {
/// #   id: usize,
/// #   count: RwSignal<i32>
/// # }
/// #
/// # #[component]
/// # fn Counters() -> impl IntoView {
/// #   let (counters, set_counters) = create_signal::<Vec<Counter>>(vec![]);
/// #
///   view! {
///     <div>
///         <For
///           each=move || counters.get()
///           key=|counter| counter.id
///           let(counter)
///         >
///             <button>"Value: " {move || counter.count.get()}</button>
///         </For>
///     </div>
///   }
/// # }
/// ```
///
/// The `let` syntax also supports destructuring the pattern of your data.
/// `let((one, two))` in the case of tuples, and `let(Struct { field_one, field_two })`
/// in the case of structs.
///
/// ```
/// # use leptos::prelude::*;
///
/// # #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// # struct Counter {
/// #   id: usize,
/// #   count: RwSignal<i32>
/// # }
/// #
/// # #[component]
/// # fn Counters() -> impl IntoView {
/// #   let (counters, set_counters) = create_signal::<Vec<Counter>>(vec![]);
/// #
///   view! {
///     <div>
///         <For
///           each=move || counters.get()
///           key=|counter| counter.id
///           let(Counter { id, count })
///         >
///             <button>"Value: " {move || count.get()}</button>
///         </For>
///     </div>
///   }
/// # }
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[component]
pub fn For<IF, I, T, EF, N, KF, K>(
    /// Items over which the component should iterate.
    each: IF,
    /// A key function that will be applied to each item.
    key: KF,
    /// A function that takes the item, and returns the view that will be displayed for each item.
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + Send + 'static,
    I: IntoIterator<Item = T> + Send + 'static,
    EF: Fn(T) -> N + Send + Clone + 'static,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + Send + Clone + 'static,
    K: Eq + Hash + SerializableKey + 'static,
    T: Send + 'static,
{
    // this takes the owner of the For itself
    // this will end up with N + 1 children
    // 1) the effect for the `move || keyed(...)` updates
    // 2) an owner for each child
    //
    // this means
    // a) the reactive owner for each row will not be cleared when the whole list updates
    // b) context provided in each row will not wipe out the others
    let parent = Owner::current().expect("no reactive owner");
    let children = move |_, child| {
        let owner = parent.with(Owner::new);
        let view = owner.with(|| children(child));
        (|_| {}, OwnedView::new_with_owner(view, owner))
    };
    move || keyed(each(), key.clone(), children.clone())
}

/// Iterates over children and displays them, keyed by the `key` function given.
///
/// Compared with For, it has an additional index parameter, which can be used to obtain the current index in real time.
///
/// This is much more efficient than naively iterating over nodes with `.iter().map(|n| view! { ... })...`,
/// as it avoids re-creating DOM nodes that are not being changed.
///
/// ```
/// # use leptos::prelude::*;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// struct Counter {
///   id: usize,
///   count: RwSignal<i32>
/// }
///
/// #[component]
/// fn Counters() -> impl IntoView {
///   let (counters, set_counters) = create_signal::<Vec<Counter>>(vec![]);
///
///   view! {
///     <div>
///       <ForEnumerate
///         // a function that returns the items we're iterating over; a signal is fine
///         each=move || counters.get()
///         // a unique key for each item
///         key=|counter| counter.id
///         // renders each item to a view
///         children={move |index: ReadSignal<usize>, counter: Counter| {
///           view! {
///             <button>{move || index.get()} ". Value: " {move || counter.count.get()}</button>
///           }
///         }}
///       />
///     </div>
///   }
/// }
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[component]
pub fn ForEnumerate<IF, I, T, EF, N, KF, K>(
    /// Items over which the component should iterate.
    each: IF,
    /// A key function that will be applied to each item.
    key: KF,
    /// A function that takes the index and the item, and returns the view that will be displayed for each item.
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + Send + 'static,
    I: IntoIterator<Item = T> + Send + 'static,
    EF: Fn(ReadSignal<usize>, T) -> N + Send + Clone + 'static,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + Send + Clone + 'static,
    K: Eq + Hash + SerializableKey + 'static,
    T: Send + 'static,
{
    // this takes the owner of the For itself
    // this will end up with N + 1 children
    // 1) the effect for the `move || keyed(...)` updates
    // 2) an owner for each child
    //
    // this means
    // a) the reactive owner for each row will not be cleared when the whole list updates
    // b) context provided in each row will not wipe out the others
    let parent = Owner::current().expect("no reactive owner");
    let children = move |index, child| {
        let owner = parent.with(Owner::new);
        let (index, set_index) = ArcRwSignal::new(index).split();
        let view = owner.with(|| children(index.into(), child));
        (
            move |index| set_index.set(index),
            OwnedView::new_with_owner(view, owner),
        )
    };
    move || keyed(each(), key.clone(), children.clone())
}

/*
#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use leptos_macro::view;
    use tachys::{html::element::HtmlElement, prelude::ElementChild};

    #[test]
    fn creates_list() {
        Owner::new().with(|| {
            let values = RwSignal::new(vec![1, 2, 3, 4, 5]);
            let list: View<HtmlElement<_, _, _>> = view! {
                <ol>
                    <For each=move || values.get() key=|i| *i let:i>
                        <li>{i}</li>
                    </For>
                </ol>
            };
            assert_eq!(
                list.to_html(),
                "<ol><li>1</li><li>2</li><li>3</li><li>4</li><li>5</li><!></\
                 ol>"
            );
        });
    }

    #[test]
    fn creates_list_enumerate() {
        Owner::new().with(|| {
            let values = RwSignal::new(vec![1, 2, 3, 4, 5]);
            let list: View<HtmlElement<_, _, _>> = view! {
                <ol>
                    <ForEnumerate each=move || values.get() key=|i| *i let(index, i)>
                        <li>{move || index.get()}"-"{i}</li>
                    </ForEnumerate>
                </ol>
            };
            assert_eq!(
                list.to_html(),
                "<ol><li>0<!>-<!>1</li><li>1<!>-<!>2</li><li>2<!>-<!>3</li><li>3\
                <!>-<!>4</li><li>4<!>-<!>5</li><!></ol>"
            );

            let list: View<HtmlElement<_, _, _>> = view! {
                <ol>
                    <ForEnumerate each=move || values.get() key=|i| *i let(index, i)>
                        <li>{move || index.get()}"-"{i}</li>
                    </ForEnumerate>
                </ol>
            };
            values.set(vec![5, 4, 1, 2, 3]);
            assert_eq!(
                list.to_html(),
                "<ol><li>0<!>-<!>5</li><li>1<!>-<!>4</li><li>2<!>-<!>1</li><li>3\
                <!>-<!>2</li><li>4<!>-<!>3</li><!></ol>"
            );
        });
    }
}
 */



================================================
FILE: leptos/src/form.rs
================================================
use crate::{children::Children, component, prelude::*, IntoView};
use leptos_dom::helpers::window;
use leptos_server::{ServerAction, ServerMultiAction};
use serde::de::DeserializeOwned;
use server_fn::{
    client::Client,
    codec::PostUrl,
    error::{IntoAppError, ServerFnErrorErr},
    request::ClientReq,
    Http, ServerFn,
};
use tachys::{
    either::Either,
    html::{
        element::{form, Form},
        event::submit,
    },
    reactive_graph::node_ref::NodeRef,
};
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{
    Event, FormData, HtmlButtonElement, HtmlFormElement, HtmlInputElement,
    SubmitEvent,
};

/// Automatically turns a server [Action](leptos_server::Action) into an HTML
/// [`form`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
/// progressively enhanced to use client-side routing.
///
/// ## Encoding
/// **Note:** `<ActionForm/>` only works with server functions that use the
/// default `Url` encoding. This is to ensure that `<ActionForm/>` works correctly
/// both before and after WASM has loaded.
///
/// ## Complex Inputs
/// Server function arguments that are structs with nested serializable fields
/// should make use of indexing notation of `serde_qs`.
///
/// ```rust
/// # use leptos::prelude::*;
/// use leptos::form::ActionForm;
///
/// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
/// struct HeftyData {
///     first_name: String,
///     last_name: String,
/// }
///
/// #[component]
/// fn ComplexInput() -> impl IntoView {
///     let submit = ServerAction::<VeryImportantFn>::new();
///
///     view! {
///       <ActionForm action=submit>
///         <input type="text" name="hefty_arg[first_name]" value="leptos"/>
///         <input
///           type="text"
///           name="hefty_arg[last_name]"
///           value="closures-everywhere"
///         />
///         <input type="submit"/>
///       </ActionForm>
///     }
/// }
///
/// #[server]
/// async fn very_important_fn(
///     hefty_arg: HeftyData,
/// ) -> Result<(), ServerFnError> {
///     assert_eq!(hefty_arg.first_name.as_str(), "leptos");
///     assert_eq!(hefty_arg.last_name.as_str(), "closures-everywhere");
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[component]
pub fn ActionForm<ServFn, OutputProtocol>(
    /// The action from which to build the form.
    action: ServerAction<ServFn>,
    /// A [`NodeRef`] in which the `<form>` element should be stored.
    #[prop(optional)]
    node_ref: Option<NodeRef<Form>>,
    /// Component children; should include the HTML of the form elements.
    children: Children,
) -> impl IntoView
where
    ServFn: DeserializeOwned
        + ServerFn<Protocol = Http<PostUrl, OutputProtocol>>
        + Clone
        + Send
        + Sync
        + 'static,
    <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<
        ServFn::Error,
    >>::FormData: From<FormData>,
    ServFn: Send + Sync + 'static,
    ServFn::Output: Send + Sync + 'static,
    ServFn::Error: Send + Sync + 'static,
    <ServFn as ServerFn>::Client: Client<<ServFn as ServerFn>::Error>,
{
    // if redirect hook has not yet been set (by a router), defaults to a browser redirect
    _ = server_fn::redirect::set_redirect_hook(|loc: &str| {
        if let Some(url) = resolve_redirect_url(loc) {
            _ = window().location().set_href(&url.href());
        }
    });

    let version = action.version();
    let value = action.value();

    let on_submit = {
        move |ev: SubmitEvent| {
            if ev.default_prevented() {
                return;
            }

            ev.prevent_default();

            match ServFn::from_event(&ev) {
                Ok(new_input) => {
                    action.dispatch(new_input);
                }
                Err(err) => {
                    crate::logging::error!(
                        "Error converting form field into server function \
                         arguments: {err:?}"
                    );
                    value.set(Some(Err(ServerFnErrorErr::Serialization(
                        err.to_string(),
                    )
                    .into_app_error())));
                    version.update(|n| *n += 1);
                }
            }
        }
    };

    let action_form = form()
        .action(ServFn::url())
        .method("post")
        .on(submit, on_submit)
        .child(children());
    if let Some(node_ref) = node_ref {
        Either::Left(action_form.node_ref(node_ref))
    } else {
        Either::Right(action_form)
    }
}

/// Automatically turns a server [MultiAction](leptos_server::MultiAction) into an HTML
/// [`form`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
/// progressively enhanced to use client-side routing.
#[component]
pub fn MultiActionForm<ServFn, OutputProtocol>(
    /// The action from which to build the form.
    action: ServerMultiAction<ServFn>,
    /// A [`NodeRef`] in which the `<form>` element should be stored.
    #[prop(optional)]
    node_ref: Option<NodeRef<Form>>,
    /// Component children; should include the HTML of the form elements.
    children: Children,
) -> impl IntoView
where
    ServFn: Send
        + Sync
        + Clone
        + DeserializeOwned
        + ServerFn<Protocol = Http<PostUrl, OutputProtocol>>
        + 'static,
    ServFn::Output: Send + Sync + 'static,
    <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<
        ServFn::Error,
    >>::FormData: From<FormData>,
    ServFn::Error: Send + Sync + 'static,
    <ServFn as ServerFn>::Client: Client<<ServFn as ServerFn>::Error>,
{
    // if redirect hook has not yet been set (by a router), defaults to a browser redirect
    _ = server_fn::redirect::set_redirect_hook(|loc: &str| {
        if let Some(url) = resolve_redirect_url(loc) {
            _ = window().location().set_href(&url.href());
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        if ev.default_prevented() {
            return;
        }

        ev.prevent_default();

        match ServFn::from_event(&ev) {
            Ok(new_input) => {
                action.dispatch(new_input);
            }
            Err(err) => {
                action.dispatch_sync(Err(ServerFnErrorErr::Serialization(
                    err.to_string(),
                )
                .into_app_error()));
            }
        }
    };

    let action_form = form()
        .action(ServFn::url())
        .method("post")
        .attr("method", "post")
        .on(submit, on_submit)
        .child(children());
    if let Some(node_ref) = node_ref {
        Either::Left(action_form.node_ref(node_ref))
    } else {
        Either::Right(action_form)
    }
}

/// Resolves a redirect location to an (absolute) URL.
pub(crate) fn resolve_redirect_url(loc: &str) -> Option<web_sys::Url> {
    let origin = match window().location().origin() {
        Ok(origin) => origin,
        Err(e) => {
            leptos::logging::error!("Failed to get origin: {:#?}", e);
            return None;
        }
    };

    // TODO: Use server function's URL as base instead.
    let base = origin;

    match web_sys::Url::new_with_base(loc, &base) {
        Ok(url) => Some(url),
        Err(e) => {
            leptos::logging::error!(
                "Invalid redirect location: {}",
                e.as_string().unwrap_or_default(),
            );
            None
        }
    }
}

/// Tries to deserialize a type from form data. This can be used for client-side
/// validation during form submission.
pub trait FromFormData
where
    Self: Sized + serde::de::DeserializeOwned,
{
    /// Tries to deserialize the data, given only the `submit` event.
    fn from_event(ev: &web_sys::Event) -> Result<Self, FromFormDataError>;

    /// Tries to deserialize the data, given the actual form data.
    fn from_form_data(
        form_data: &web_sys::FormData,
    ) -> Result<Self, serde_qs::Error>;
}

/// Errors that can arise when converting from an HTML event or form into a Rust data type.
#[derive(Error, Debug)]
pub enum FromFormDataError {
    /// Could not find a `<form>` connected to the event.
    #[error("Could not find <form> connected to event.")]
    MissingForm(Event),
    /// Could not create `FormData` from the form.
    #[error("Could not create FormData from <form>: {0:?}")]
    FormData(JsValue),
    /// Failed to deserialize this Rust type from the form data.
    #[error("Deserialization error: {0:?}")]
    Deserialization(serde_qs::Error),
}

impl<T> FromFormData for T
where
    T: serde::de::DeserializeOwned,
{
    fn from_event(ev: &Event) -> Result<Self, FromFormDataError> {
        let submit_ev = ev.unchecked_ref();
        let form_data = form_data_from_event(submit_ev)?;
        Self::from_form_data(&form_data)
            .map_err(FromFormDataError::Deserialization)
    }

    fn from_form_data(
        form_data: &web_sys::FormData,
    ) -> Result<Self, serde_qs::Error> {
        let data =
            web_sys::UrlSearchParams::new_with_str_sequence_sequence(form_data)
                .unwrap_throw();
        let data = data.to_string().as_string().unwrap_or_default();
        serde_qs::Config::new(5, false).deserialize_str::<Self>(&data)
    }
}

fn form_data_from_event(
    ev: &SubmitEvent,
) -> Result<FormData, FromFormDataError> {
    let submitter = ev.submitter();
    let mut submitter_name_value = None;
    let opt_form = match &submitter {
        Some(el) => {
            if let Some(form) = el.dyn_ref::<HtmlFormElement>() {
                Some(form.clone())
            } else if let Some(input) = el.dyn_ref::<HtmlInputElement>() {
                submitter_name_value = Some((input.name(), input.value()));
                Some(ev.target().unwrap().unchecked_into())
            } else if let Some(button) = el.dyn_ref::<HtmlButtonElement>() {
                submitter_name_value = Some((button.name(), button.value()));
                Some(ev.target().unwrap().unchecked_into())
            } else {
                None
            }
        }
        None => ev.target().map(|form| form.unchecked_into()),
    };
    match opt_form.as_ref().map(FormData::new_with_form) {
        None => Err(FromFormDataError::MissingForm(ev.clone().into())),
        Some(Err(e)) => Err(FromFormDataError::FormData(e)),
        Some(Ok(form_data)) => {
            if let Some((name, value)) = submitter_name_value {
                form_data
                    .append_with_str(&name, &value)
                    .map_err(FromFormDataError::FormData)?;
            }
            Ok(form_data)
        }
    }
}



================================================
FILE: leptos/src/from_form_data.rs
================================================




================================================
FILE: leptos/src/into_view.rs
================================================
use std::borrow::Cow;
use tachys::{
    html::attribute::{any_attribute::AnyAttribute, Attribute},
    hydration::Cursor,
    ssr::StreamBuilder,
    view::{
        add_attr::AddAnyAttr, Position, PositionState, Render, RenderHtml,
        ToTemplate,
    },
};

/// A wrapper for any kind of view.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct View<T>
where
    T: Sized,
{
    inner: T,
    #[cfg(debug_assertions)]
    view_marker: Option<Cow<'static, str>>,
}

impl<T> View<T> {
    /// Wraps the view.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            #[cfg(debug_assertions)]
            view_marker: None,
        }
    }

    /// Unwraps the view, returning the inner type.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Adds a view marker, which is used for hot-reloading and debug purposes.
    #[inline(always)]
    pub fn with_view_marker(
        #[allow(unused_mut)] // used in debug
        mut self,
        #[allow(unused_variables)] // used in debug
        view_marker: impl Into<Cow<'static, str>>,
    ) -> Self {
        #[cfg(debug_assertions)]
        {
            self.view_marker = Some(view_marker.into());
        }
        self
    }
}

/// A trait that is implemented for types that can be rendered.
pub trait IntoView
where
    Self: Sized + Render + RenderHtml + Send,
{
    /// Wraps the inner type.
    fn into_view(self) -> View<Self>;
}

impl<T> IntoView for T
where
    T: Sized + Render + RenderHtml + Send, //+ AddAnyAttr,
{
    fn into_view(self) -> View<Self> {
        View {
            inner: self,
            #[cfg(debug_assertions)]
            view_marker: None,
        }
    }
}

impl<T: Render> Render for View<T> {
    type State = T::State;

    fn build(self) -> Self::State {
        self.inner.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.inner.rebuild(state)
    }
}

impl<T: RenderHtml> RenderHtml for View<T> {
    type AsyncOutput = T::AsyncOutput;
    type Owned = View<T::Owned>;

    const MIN_LENGTH: usize = <T as RenderHtml>::MIN_LENGTH;
    const EXISTS: bool = <T as RenderHtml>::EXISTS;

    async fn resolve(self) -> Self::AsyncOutput {
        self.inner.resolve().await
    }

    fn dry_resolve(&mut self) {
        self.inner.dry_resolve();
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        #[cfg(debug_assertions)]
        let vm = if option_env!("LEPTOS_WATCH").is_some() {
            self.view_marker.to_owned()
        } else {
            None
        };

        #[cfg(debug_assertions)]
        if let Some(vm) = vm.as_ref() {
            buf.push_str(&format!("<!--hot-reload|{vm}|open-->"));
        }

        self.inner.to_html_with_buf(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );

        #[cfg(debug_assertions)]
        if let Some(vm) = vm.as_ref() {
            buf.push_str(&format!("<!--hot-reload|{vm}|close-->"));
        }
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) where
        Self: Sized,
    {
        #[cfg(debug_assertions)]
        let vm = if option_env!("LEPTOS_WATCH").is_some() {
            self.view_marker.to_owned()
        } else {
            None
        };

        #[cfg(debug_assertions)]
        if let Some(vm) = vm.as_ref() {
            buf.push_sync(&format!("<!--hot-reload|{vm}|open-->"));
        }

        self.inner.to_html_async_with_buf::<OUT_OF_ORDER>(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );

        #[cfg(debug_assertions)]
        if let Some(vm) = vm.as_ref() {
            buf.push_sync(&format!("<!--hot-reload|{vm}|close-->"));
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        self.inner.hydrate::<FROM_SERVER>(cursor, position)
    }

    async fn hydrate_async(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        self.inner.hydrate_async(cursor, position).await
    }

    fn into_owned(self) -> Self::Owned {
        View {
            inner: self.inner.into_owned(),
            #[cfg(debug_assertions)]
            view_marker: self.view_marker,
        }
    }
}

impl<T: ToTemplate> ToTemplate for View<T> {
    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
        position: &mut Position,
    ) {
        T::to_template(buf, class, style, inner_html, position);
    }
}

impl<T: AddAnyAttr> AddAnyAttr for View<T> {
    type Output<SomeNewAttr: Attribute> = View<T::Output<SomeNewAttr>>;

    fn add_any_attr<NewAttr: Attribute>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        let View {
            inner,
            #[cfg(debug_assertions)]
            view_marker,
        } = self;
        View {
            inner: inner.add_any_attr(attr),
            #[cfg(debug_assertions)]
            view_marker,
        }
    }
}

/// Collects some iterator of views into a list, so they can be rendered.
///
/// This is a shorthand for `.collect::<Vec<_>>()`, and allows any iterator of renderable
/// items to be collected into a renderable collection.
pub trait CollectView {
    /// The inner view type.
    type View: IntoView;

    /// Collects the iterator into a list of views.
    fn collect_view(self) -> Vec<Self::View>;
}

impl<It, V> CollectView for It
where
    It: IntoIterator<Item = V>,
    V: IntoView,
{
    type View = V;

    fn collect_view(self) -> Vec<Self::View> {
        self.into_iter().collect()
    }
}



================================================
FILE: leptos/src/lib.rs
================================================
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # About Leptos
//!
//! Leptos is a full-stack framework for building web applications in Rust. You can use it to build
//! - single-page apps (SPAs) rendered entirely in the browser, using client-side routing and loading
//!   or mutating data via async requests to the server.
//! - multi-page apps (MPAs) rendered on the server, managing navigation, data, and mutations via
//!   web-standard `<a>` and `<form>` tags.
//! - progressively-enhanced single-page apps that are rendered on the server and then hydrated on the client,
//!   enhancing your `<a>` and `<form>` navigations and mutations seamlessly when WASM is available.
//!
//! And you can do all three of these **using the same Leptos code**.
//!
//! Take a look at the [Leptos Book](https://leptos-rs.github.io/leptos/) for a walkthrough of the framework.
//! Join us on our [Discord Channel](https://discord.gg/v38Eef6sWG) to see what the community is building.
//! Explore our [Examples](https://github.com/leptos-rs/leptos/tree/main/examples) to see Leptos in action.
//!
//! # Learning by Example
//!
//! If you want to see what Leptos is capable of, check out
//! the [examples](https://github.com/leptos-rs/leptos/tree/main/examples):
//!
//! - **[`counter`]** is the classic counter example, showing the basics of client-side rendering and reactive DOM updates.
//! - **[`counter_without_macros`]** adapts the counter example to use the builder pattern for the UI and avoids other macros,
//!   instead showing the code that Leptos generates.
//! - **[`counters`]** introduces parent-child communication via contexts, and the [`<For/>`](leptos::prelude::For) component
//!   for efficient keyed list updates.
//! - **[`error_boundary`]** shows how to use [`Result`] types to handle errors.
//! - **[`parent_child`]** shows four different ways a parent component can communicate with a child, including passing a closure,
//!   context, and more.
//! - **[`fetch`]** introduces [`Resource`](leptos::prelude::Resource)s, which allow you to integrate arbitrary `async` code like an
//!   HTTP request within your reactive code.
//! - **[`router`]** shows how to use Leptos’s nested router to enable client-side navigation and route-specific, reactive data loading.
//! - **[`slots`]** shows how to use slots on components.
//! - **[`spread`]** shows how the spread syntax can be used to spread data and/or event handlers onto elements.
//! - **[`counter_isomorphic`]** shows different methods of interaction with a stateful server, including server functions,
//!   server actions, forms, and server-sent events (SSE).
//! - **[`todomvc`]** shows the basics of building an isomorphic web app. Both the server and the client import the same app code.
//!   The server renders the app directly to an HTML string, and the client hydrates that HTML to make it interactive.
//!   You might also want to see how we use [`Effect::new`](leptos::prelude::Effect) to
//!   [serialize JSON to `localStorage`](https://github.com/leptos-rs/leptos/blob/20af4928b2fffe017408d3f4e7330db22cf68277/examples/todomvc/src/lib.rs#L191-L209)
//!   and [reactively call DOM methods](https://github.com/leptos-rs/leptos/blob/16f084a71268ac325fbc4a5e50c260df185eadb6/examples/todomvc/src/lib.rs#L292-L296)
//!   on [references to elements](https://github.com/leptos-rs/leptos/blob/20af4928b2fffe017408d3f4e7330db22cf68277/examples/todomvc/src/lib.rs#L228).
//! - **[`hackernews`]** and **[`hackernews_axum`]** integrate calls to a real external REST API, routing, server-side rendering and
//!   hydration to create a fully-functional application that works as intended even before WASM has loaded and begun to run.
//! - **[`todo_app_sqlite`]** and **[`todo_app_sqlite_axum`]** show how to build a full-stack app using server functions and
//!   database connections.
//! - **[`tailwind`]** shows how to integrate TailwindCSS with `trunk` for CSR.
//!
//! [`counter`]: https://github.com/leptos-rs/leptos/tree/main/examples/counter
//! [`counter_without_macros`]: https://github.com/leptos-rs/leptos/tree/main/examples/counter_without_macros
//! [`counters`]: https://github.com/leptos-rs/leptos/tree/main/examples/counters
//! [`error_boundary`]: https://github.com/leptos-rs/leptos/tree/main/examples/error_boundary
//! [`parent_child`]: https://github.com/leptos-rs/leptos/tree/main/examples/parent_child
//! [`fetch`]: https://github.com/leptos-rs/leptos/tree/main/examples/fetch
//! [`router`]: https://github.com/leptos-rs/leptos/tree/main/examples/router
//! [`slots`]: https://github.com/leptos-rs/leptos/tree/main/examples/slots
//! [`spread`]: https://github.com/leptos-rs/leptos/tree/main/examples/spread
//! [`counter_isomorphic`]: https://github.com/leptos-rs/leptos/tree/main/examples/counter_isomorphic
//! [`todomvc`]: https://github.com/leptos-rs/leptos/tree/main/examples/todomvc
//! [`hackernews`]: https://github.com/leptos-rs/leptos/tree/main/examples/hackernews
//! [`hackernews_axum`]: https://github.com/leptos-rs/leptos/tree/main/examples/hackernews_axum
//! [`todo_app_sqlite`]: https://github.com/leptos-rs/leptos/tree/main/examples/todo_app_sqlite
//! [`todo_app_sqlite_axum`]: https://github.com/leptos-rs/leptos/tree/main/examples/todo_app_sqlite_axum
//! [`tailwind`]: https://github.com/leptos-rs/leptos/tree/main/examples/tailwind_csr
//!
//! Details on how to run each example can be found in its README.
//!
//! # Quick Links
//!
//! Here are links to the most important sections of the docs:
//! - **Reactivity**: the [`reactive_graph`] overview, and more details in
//!   + signals: [`signal`](leptos::prelude::signal), [`ReadSignal`](leptos::prelude::ReadSignal),
//!     [`WriteSignal`](leptos::prelude::WriteSignal) and [`RwSignal`](leptos::prelude::RwSignal).
//!   + computations: [`Memo`](leptos::prelude::Memo).
//!   + `async` interop: [`Resource`](leptos::prelude::Resource) for loading data using `async` functions
//!     and [`Action`](leptos::prelude::Action) to mutate data or imperatively call `async` functions.
//!   + reactions: [`Effect`](leptos::prelude::Effect) and [`RenderEffect`](leptos::prelude::RenderEffect).
//! - **Templating/Views**: the [`view`] macro and [`IntoView`] trait.
//! - **Routing**: the [`leptos_router`](https://docs.rs/leptos_router/latest/leptos_router/) crate
//! - **Server Functions**: the [`server`](macro@leptos::prelude::server) macro and [`ServerAction`](leptos::prelude::ServerAction).
//!
//! # Feature Flags
//!
//! - **`nightly`**: On `nightly` Rust, enables the function-call syntax for signal getters and setters.
//! - **`csr`** Client-side rendering: Generate DOM nodes in the browser.
//! - **`ssr`** Server-side rendering: Generate an HTML string (typically on the server).
//! - **`hydrate`** Hydration: use this to add interactivity to an SSRed Leptos app.
//! - **`rkyv`** In SSR/hydrate mode, uses [`rkyv`](https://docs.rs/rkyv/latest/rkyv/) to serialize resources and send them
//!   from the server to the client.
//! - **`tracing`** Adds support for [`tracing`](https://docs.rs/tracing/latest/tracing/).
//!
//! **Important Note:** You must enable one of `csr`, `hydrate`, or `ssr` to tell Leptos
//! which mode your app is operating in. You should only enable one of these per build target,
//! i.e., you should not have both `hydrate` and `ssr` enabled for your server binary, only `ssr`.
//!
//! # A Simple Counter
//!
//! ```rust
//! use leptos::prelude::*;
//!
//! #[component]
//! pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
//!     // create a reactive signal with the initial value
//!     let (value, set_value) = signal(initial_value);
//!
//!     // create event handlers for our buttons
//!     // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
//!     let clear = move |_| set_value.set(0);
//!     let decrement = move |_| *set_value.write() -= 1;
//!     let increment = move |_| *set_value.write() += 1;
//!
//!     view! {
//!         <div>
//!             <button on:click=clear>"Clear"</button>
//!             <button on:click=decrement>"-1"</button>
//!             <span>"Value: " {value} "!"</span>
//!             <button on:click=increment>"+1"</button>
//!         </div>
//!     }
//! }
//! ```
//!
//! Leptos is easy to use with [Trunk](https://trunkrs.dev/) (or with a simple wasm-bindgen setup):
//!
//! ```rust
//! use leptos::{mount::mount_to_body, prelude::*};
//!
//! #[component]
//! fn SimpleCounter(initial_value: i32) -> impl IntoView {
//!     // ...
//!     # _ = initial_value;
//! }
//!
//! pub fn main() {
//! # if false { // can't run in doctest
//!     mount_to_body(|| view! { <SimpleCounter initial_value=3 /> })
//! # }
//! }
//! ```

#![cfg_attr(all(feature = "nightly", rustc_nightly), feature(fn_traits))]
#![cfg_attr(all(feature = "nightly", rustc_nightly), feature(unboxed_closures))]

extern crate self as leptos;

/// Exports all the core types of the library.
pub mod prelude {
    // Traits
    // These should always be exported from the prelude
    pub use reactive_graph::prelude::*;
    pub use tachys::prelude::*;

    // Structs
    // In the future, maybe we should remove this blanket export
    // However, it is definitely useful relative to looking up every struct etc.
    mod export_types {
        #[cfg(feature = "nonce")]
        pub use crate::nonce::*;
        pub use crate::{
            callback::*, children::*, component::*, control_flow::*, error::*,
            form::*, hydration::*, into_view::*, mount::*, suspense::*,
            text_prop::*,
        };
        pub use leptos_config::*;
        pub use leptos_dom::helpers::*;
        pub use leptos_macro::*;
        pub use leptos_server::*;
        pub use oco_ref::*;
        pub use reactive_graph::{
            actions::*,
            computed::*,
            effect::*,
            graph::untrack,
            owner::*,
            signal::*,
            wrappers::{read::*, write::*},
        };
        pub use server_fn::{
            self,
            error::{FromServerFnError, ServerFnError, ServerFnErrorErr},
        };
        pub use tachys::{
            reactive_graph::{bind::BindAttribute, node_ref::*, Suspend},
            view::{fragment::Fragment, template::ViewTemplate},
        };
    }
    pub use export_types::*;
}

/// Components used for working with HTML forms, like `<ActionForm>`.
pub mod form;

/// A standard way to wrap functions and closures to pass them to components.
pub mod callback;

/// Types that can be passed as the `children` prop of a component.
pub mod children;

/// Wrapper for intercepting component attributes.
pub mod attribute_interceptor;

#[doc(hidden)]
/// Traits used to implement component constructors.
pub mod component;
mod error_boundary;

/// Tools for handling errors.
pub mod error {
    pub use crate::error_boundary::*;
    pub use throw_error::*;
}

/// Control-flow components like `<Show>`, `<For>`, and `<Await>`.
pub mod control_flow {
    pub use crate::{animated_show::*, await_::*, for_loop::*, show::*};
}
mod animated_show;
mod await_;
mod for_loop;
mod show;

/// A component that allows rendering a component somewhere else.
pub mod portal;

/// Components to enable server-side rendering and client-side hydration.
pub mod hydration;

/// Utilities for exporting nonces to be used for a Content Security Policy.
#[cfg(feature = "nonce")]
pub mod nonce;

/// Components to load asynchronous data.
pub mod suspense {
    pub use crate::{suspense_component::*, transition::*};
}

#[macro_use]
mod suspense_component;

/// Types for reactive string properties for components.
pub mod text_prop;
mod transition;
pub use leptos_macro::*;
#[doc(inline)]
pub use server_fn;
#[doc(hidden)]
pub use typed_builder;
#[doc(hidden)]
pub use typed_builder_macro;
mod into_view;
pub use into_view::IntoView;
#[doc(inline)]
pub use leptos_dom;
mod provider;
#[doc(inline)]
pub use tachys;
/// Tools to mount an application to the DOM, or to hydrate it from server-rendered HTML.
pub mod mount;
#[doc(inline)]
pub use leptos_config as config;
#[doc(inline)]
pub use oco_ref as oco;
mod from_form_data;
#[doc(inline)]
pub use either_of as either;
#[doc(inline)]
pub use reactive_graph as reactive;

/// Provide and access data along the reactive graph, sharing data without directly passing arguments.
pub mod context {
    pub use crate::provider::*;
    pub use reactive_graph::owner::{provide_context, use_context};
}

#[doc(inline)]
pub use leptos_server as server;
/// HTML attribute types.
#[doc(inline)]
pub use tachys::html::attribute as attr;
/// HTML element types.
#[doc(inline)]
pub use tachys::html::element as html;
/// HTML event types.
#[doc(no_inline)]
pub use tachys::html::event as ev;
/// MathML element types.
#[doc(inline)]
pub use tachys::mathml as math;
/// SVG element types.
#[doc(inline)]
pub use tachys::svg;

/// Utilities for simple isomorphic logging to the console or terminal.
pub mod logging {
    pub use leptos_dom::{debug_warn, error, log, warn};
}

/// Utilities for working with asynchronous tasks.
pub mod task {
    use any_spawner::Executor;
    use std::future::Future;

    /// Spawns a thread-safe [`Future`].
    #[track_caller]
    #[inline(always)]
    pub fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
        #[cfg(not(target_family = "wasm"))]
        Executor::spawn(fut);

        #[cfg(target_family = "wasm")]
        Executor::spawn_local(fut);
    }

    /// Spawns a [`Future`] that cannot be sent across threads.
    #[track_caller]
    #[inline(always)]
    pub fn spawn_local(fut: impl Future<Output = ()> + 'static) {
        Executor::spawn_local(fut)
    }

    /// Waits until the next "tick" of the current async executor.
    pub async fn tick() {
        Executor::tick().await
    }

    pub use reactive_graph::{
        spawn_local_scoped, spawn_local_scoped_with_cancellation,
    };
}

// these reexports are used in islands
#[cfg(feature = "islands")]
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use serde_json;
#[cfg(feature = "tracing")]
#[doc(hidden)]
pub use tracing;
#[doc(hidden)]
pub use wasm_bindgen;
pub use wasm_split_helpers;
#[doc(hidden)]
pub use web_sys;

#[doc(hidden)]
pub mod __reexports {
    pub use send_wrapper;
    pub use wasm_bindgen_futures;
}

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct PrefetchLazyFn(
    pub  reactive_graph::owner::ArcStoredValue<
        std::collections::HashSet<&'static str>,
    >,
);

#[doc(hidden)]
pub fn prefetch_lazy_fn_on_server(id: &'static str) {
    use crate::context::use_context;
    use reactive_graph::traits::WriteValue;

    if let Some(prefetches) = use_context::<PrefetchLazyFn>() {
        prefetches.0.write_value().insert(id);
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct WasmSplitManifest(
    pub  reactive_graph::owner::ArcStoredValue<(
        String,
        std::collections::HashMap<String, Vec<String>>,
    )>,
);



================================================
FILE: leptos/src/logging.rs
================================================
//! Utilities for simple isomorphic logging to the console or terminal.

use wasm_bindgen::JsValue;

/// Uses `println!()`-style formatting to log something to the console (in the browser)
/// or via `println!()` (if not in the browser).
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => ($crate::logging::console_log(&format_args!($($t)*).to_string()))
}

/// Uses `println!()`-style formatting to log warnings to the console (in the browser)
/// or via `eprintln!()` (if not in the browser).
#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => ($crate::logging::console_warn(&format_args!($($t)*).to_string()))
}

/// Uses `println!()`-style formatting to log errors to the console (in the browser)
/// or via `eprintln!()` (if not in the browser).
#[macro_export]
macro_rules! error {
    ($($t:tt)*) => ($crate::logging::console_error(&format_args!($($t)*).to_string()))
}

/// Uses `println!()`-style formatting to log warnings to the console (in the browser)
/// or via `eprintln!()` (if not in the browser), but only if it's a debug build.
#[macro_export]
macro_rules! debug_warn {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                $crate::warn!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}

const fn log_to_stdout() -> bool {
    cfg!(not(all(
        target_arch = "wasm32",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )))
}

/// Log a string to the console (in the browser)
/// or via `println!()` (if not in the browser).
pub fn console_log(s: &str) {
    #[allow(clippy::print_stdout)]
    if log_to_stdout() {
        println!("{s}");
    } else {
        web_sys::console::log_1(&JsValue::from_str(s));
    }
}

/// Log a warning to the console (in the browser)
/// or via `println!()` (if not in the browser).
pub fn console_warn(s: &str) {
    if log_to_stdout() {
        eprintln!("{s}");
    } else {
        web_sys::console::warn_1(&JsValue::from_str(s));
    }
}

/// Log an error to the console (in the browser)
/// or via `println!()` (if not in the browser).
#[inline(always)]
pub fn console_error(s: &str) {
    if log_to_stdout() {
        eprintln!("{s}");
    } else {
        web_sys::console::error_1(&JsValue::from_str(s));
    }
}

/// Log an error to the console (in the browser)
/// or via `println!()` (if not in the browser), but only in a debug build.
#[inline(always)]
pub fn console_debug_warn(s: &str) {
    #[cfg(debug_assertions)]
    {
        if log_to_stdout() {
            eprintln!("{s}");
        } else {
            web_sys::console::warn_1(&JsValue::from_str(s));
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let _ = s;
    }
}





================================================
FILE: leptos/src/mount.rs
================================================
#[cfg(debug_assertions)]
use crate::logging;
use crate::IntoView;
use any_spawner::Executor;
use reactive_graph::owner::Owner;
#[cfg(debug_assertions)]
use std::cell::Cell;
use tachys::{
    dom::body,
    view::{Mountable, Render},
};
#[cfg(feature = "hydrate")]
use tachys::{
    hydration::Cursor,
    view::{PositionState, RenderHtml},
};
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[cfg(feature = "hydrate")]
/// Hydrates the app described by the provided function, starting at `<body>`.
pub fn hydrate_body<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let owner = hydrate_from(body(), f);
    owner.forget();
}

#[cfg(feature = "hydrate")]
/// Hydrates the app described by the provided function, starting at `<body>`, with support
/// for lazy-loaded routes and components.
pub fn hydrate_lazy<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    // use wasm-bindgen-futures to drive the reactive system
    // we ignore the return value because an Err here just means the wasm-bindgen executor is
    // already initialized, which is not an issue
    _ = Executor::init_wasm_bindgen();

    crate::task::spawn_local(async move {
        let owner = hydrate_from_async(body(), f).await;
        owner.forget();
    })
}

#[cfg(debug_assertions)]
thread_local! {
    static FIRST_CALL: Cell<bool> = const { Cell::new(true) };
}

#[cfg(feature = "hydrate")]
/// Runs the provided closure and mounts the result to the provided element.
pub fn hydrate_from<F, N>(parent: HtmlElement, f: F) -> UnmountHandle<N::State>
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    use hydration_context::HydrateSharedContext;
    use std::sync::Arc;

    // use wasm-bindgen-futures to drive the reactive system
    // we ignore the return value because an Err here just means the wasm-bindgen executor is
    // already initialized, which is not an issue
    _ = Executor::init_wasm_bindgen();

    #[cfg(debug_assertions)]
    {
        if !cfg!(feature = "hydrate") && FIRST_CALL.get() {
            logging::warn!(
                "It seems like you're trying to use Leptos in hydration mode, \
                 but the `hydrate` feature is not enabled on the `leptos` \
                 crate. Add `features = [\"hydrate\"]` to your Cargo.toml for \
                 the crate to work properly.\n\nNote that hydration and \
                 client-side rendering now use separate functions from \
                 leptos::mount: you are calling a hydration function."
            );
        }
        FIRST_CALL.set(false);
    }

    // create a new reactive owner and use it as the root node to run the app
    let owner = Owner::new_root(Some(Arc::new(HydrateSharedContext::new())));
    let mountable = owner.with(move || {
        let view = f().into_view();
        view.hydrate::<true>(
            &Cursor::new(parent.unchecked_into()),
            &PositionState::default(),
        )
    });

    if let Some(sc) = Owner::current_shared_context() {
        sc.hydration_complete();
    }

    // returns a handle that owns the owner
    // when this is dropped, it will clean up the reactive system and unmount the view
    UnmountHandle { owner, mountable }
}

#[cfg(feature = "hydrate")]
/// Runs the provided closure and mounts the result to the provided element.
pub async fn hydrate_from_async<F, N>(
    parent: HtmlElement,
    f: F,
) -> UnmountHandle<N::State>
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    use hydration_context::HydrateSharedContext;
    use std::sync::Arc;

    // use wasm-bindgen-futures to drive the reactive system
    // we ignore the return value because an Err here just means the wasm-bindgen executor is
    // already initialized, which is not an issue
    _ = Executor::init_wasm_bindgen();

    #[cfg(debug_assertions)]
    {
        if !cfg!(feature = "hydrate") && FIRST_CALL.get() {
            logging::warn!(
                "It seems like you're trying to use Leptos in hydration mode, \
                 but the `hydrate` feature is not enabled on the `leptos` \
                 crate. Add `features = [\"hydrate\"]` to your Cargo.toml for \
                 the crate to work properly.\n\nNote that hydration and \
                 client-side rendering now use separate functions from \
                 leptos::mount: you are calling a hydration function."
            );
        }
        FIRST_CALL.set(false);
    }

    // create a new reactive owner and use it as the root node to run the app
    let owner = Owner::new_root(Some(Arc::new(HydrateSharedContext::new())));
    let mountable = owner
        .with(move || {
            use reactive_graph::computed::ScopedFuture;

            ScopedFuture::new(async move {
                let view = f().into_view();
                view.hydrate_async(
                    &Cursor::new(parent.unchecked_into()),
                    &PositionState::default(),
                )
                .await
            })
        })
        .await;

    if let Some(sc) = Owner::current_shared_context() {
        sc.hydration_complete();
    }

    // returns a handle that owns the owner
    // when this is dropped, it will clean up the reactive system and unmount the view
    UnmountHandle { owner, mountable }
}

/// Runs the provided closure and mounts the result to the `<body>`.
pub fn mount_to_body<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let owner = mount_to(body(), f);
    owner.forget();
}

/// Runs the provided closure and mounts the result to the provided element.
pub fn mount_to<F, N>(parent: HtmlElement, f: F) -> UnmountHandle<N::State>
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    // use wasm-bindgen-futures to drive the reactive system
    // we ignore the return value because an Err here just means the wasm-bindgen executor is
    // already initialized, which is not an issue
    _ = Executor::init_wasm_bindgen();

    #[cfg(debug_assertions)]
    {
        if !cfg!(feature = "csr") && FIRST_CALL.get() {
            logging::warn!(
                "It seems like you're trying to use Leptos in client-side \
                 rendering mode, but the `csr` feature is not enabled on the \
                 `leptos` crate. Add `features = [\"csr\"]` to your \
                 Cargo.toml for the crate to work properly.\n\nNote that \
                 hydration and client-side rendering now use different \
                 functions from leptos::mount. You are using a client-side \
                 rendering mount function."
            );
        }
        FIRST_CALL.set(false);
    }

    // create a new reactive owner and use it as the root node to run the app
    let owner = Owner::new();
    let mountable = owner.with(move || {
        let view = f().into_view();
        let mut mountable = view.build();
        mountable.mount(&parent, None);
        mountable
    });

    // returns a handle that owns the owner
    // when this is dropped, it will clean up the reactive system and unmount the view
    UnmountHandle { owner, mountable }
}

/// Runs the provided closure and mounts the result to the provided element.
pub fn mount_to_renderer<F, N>(
    parent: &tachys::renderer::types::Element,
    f: F,
) -> UnmountHandle<N::State>
where
    F: FnOnce() -> N + 'static,
    N: Render,
{
    // use wasm-bindgen-futures to drive the reactive system
    // we ignore the return value because an Err here just means the wasm-bindgen executor is
    // already initialized, which is not an issue
    _ = Executor::init_wasm_bindgen();

    // create a new reactive owner and use it as the root node to run the app
    let owner = Owner::new();
    let mountable = owner.with(move || {
        let view = f();
        let mut mountable = view.build();
        mountable.mount(parent, None);
        mountable
    });

    // returns a handle that owns the owner
    // when this is dropped, it will clean up the reactive system and unmount the view
    UnmountHandle { owner, mountable }
}

/// Hydrates any islands that are currently present on the page.
#[cfg(feature = "hydrate")]
pub fn hydrate_islands() {
    use hydration_context::{HydrateSharedContext, SharedContext};
    use std::sync::Arc;

    // use wasm-bindgen-futures to drive the reactive system
    // we ignore the return value because an Err here just means the wasm-bindgen executor is
    // already initialized, which is not an issue
    _ = Executor::init_wasm_bindgen();

    #[cfg(debug_assertions)]
    FIRST_CALL.set(false);

    // create a new reactive owner and use it as the root node to run the app
    let sc = HydrateSharedContext::new();
    sc.set_is_hydrating(false); // islands mode starts in "not hydrating"
    let owner = Owner::new_root(Some(Arc::new(sc)));
    owner.set();
    std::mem::forget(owner);
}

/// On drop, this will clean up the reactive [`Owner`] and unmount the view created by
/// [`mount_to`].
///
/// If you are using it to create the root of an application, you should use
/// [`UnmountHandle::forget`] to leak it.
#[must_use = "Dropping an `UnmountHandle` will unmount the view and cancel the \
              reactive system. You should either call `.forget()` to keep the \
              view permanently mounted, or store the `UnmountHandle` somewhere \
              and drop it when you'd like to unmount the view."]
pub struct UnmountHandle<M>
where
    M: Mountable,
{
    #[allow(dead_code)]
    owner: Owner,
    mountable: M,
}

impl<M> UnmountHandle<M>
where
    M: Mountable,
{
    /// Leaks the handle, preventing the reactive system from being cleaned up and the view from
    /// being unmounted. This should always be called when [`mount_to`] is used for the root of an
    /// application that should live for the long term.
    pub fn forget(self) {
        std::mem::forget(self);
    }
}

impl<M> Drop for UnmountHandle<M>
where
    M: Mountable,
{
    fn drop(&mut self) {
        self.mountable.unmount();
    }
}



================================================
FILE: leptos/src/nonce.rs
================================================
use crate::context::{provide_context, use_context};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};
use rand::{rng, RngCore};
use std::{fmt::Display, ops::Deref, sync::Arc};
use tachys::html::attribute::AttributeValue;

/// A cryptographic nonce ("number used once") which can be
/// used by Content Security Policy to determine whether or not a given
/// resource will be allowed to load.
///
/// When the `nonce` feature is enabled on one of the server integrations,
/// a nonce is generated during server rendering and added to all inline
/// scripts used for HTML streaming and resource loading.
///
/// The nonce being used during the current server response can be
/// accessed using [`use_nonce`].
///
/// ```rust,ignore
/// #[component]
/// pub fn App() -> impl IntoView {
///     provide_meta_context;
///
///     view! {
///         // use `leptos_meta` to insert a <meta> tag with the CSP
///         <Meta
///             http_equiv="Content-Security-Policy"
///             content=move || {
///                 // this will insert the CSP with nonce on the server, be empty on client
///                 use_nonce()
///                     .map(|nonce| {
///                         format!(
///                             "default-src 'self'; script-src 'strict-dynamic' 'nonce-{nonce}' \
///                             'wasm-unsafe-eval'; style-src 'nonce-{nonce}';"
///                         )
///                     })
///                     .unwrap_or_default()
///             }
///         />
///         // manually insert nonce during SSR on inline script
///         <script nonce=use_nonce()>"console.log('Hello, world!');"</script>
///         // leptos_meta <Style/> and <Script/> automatically insert the nonce
///         <Style>"body { color: blue; }"</Style>
///         <p>"Test"</p>
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nonce(pub(crate) Arc<str>);

impl Nonce {
    /// Returns a reference to the inner reference-counted string slice representing the nonce.
    pub fn as_inner(&self) -> &Arc<str> {
        &self.0
    }
}

impl Deref for Nonce {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Nonce {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AttributeValue for Nonce {
    type AsyncOutput = Self;
    type State = <Arc<str> as AttributeValue>::State;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.0.len()
    }

    fn to_html(self, key: &str, buf: &mut String) {
        <Arc<str> as AttributeValue>::to_html(self.0, key, buf)
    }

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(
        self,
        key: &str,
        el: &tachys::renderer::types::Element,
    ) -> Self::State {
        <Arc<str> as AttributeValue>::hydrate::<FROM_SERVER>(self.0, key, el)
    }

    fn build(
        self,
        el: &tachys::renderer::types::Element,
        key: &str,
    ) -> Self::State {
        <Arc<str> as AttributeValue>::build(self.0, el, key)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        <Arc<str> as AttributeValue>::rebuild(self.0, key, state)
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }
}

/// Accesses the nonce that has been generated during the current
/// server response. This can be added to inline `<script>` and
/// `<style>` tags for compatibility with a Content Security Policy.
///
/// ```rust,ignore
/// #[component]
/// pub fn App() -> impl IntoView {
///     provide_meta_context;
///
///     view! {
///         // use `leptos_meta` to insert a <meta> tag with the CSP
///         <Meta
///             http_equiv="Content-Security-Policy"
///             content=move || {
///                 // this will insert the CSP with nonce on the server, be empty on client
///                 use_nonce()
///                     .map(|nonce| {
///                         format!(
///                             "default-src 'self'; script-src 'strict-dynamic' 'nonce-{nonce}' \
///                             'wasm-unsafe-eval'; style-src 'nonce-{nonce}';"
///                         )
///                     })
///                     .unwrap_or_default()
///             }
///         />
///         // manually insert nonce during SSR on inline script
///         <script nonce=use_nonce()>"console.log('Hello, world!');"</script>
///         // leptos_meta <Style/> and <Script/> automatically insert the nonce
///         <Style>"body { color: blue; }"</Style>
///         <p>"Test"</p>
///     }
/// }
/// ```
pub fn use_nonce() -> Option<Nonce> {
    use_context::<Nonce>()
}

/// Generates a nonce and provides it via context.
pub fn provide_nonce() {
    provide_context(Nonce::new())
}

const NONCE_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl Nonce {
    /// Generates a new nonce from 16 bytes (128 bits) of random data.
    pub fn new() -> Self {
        let mut rng = rng();
        let mut bytes = [0; 16];
        rng.fill_bytes(&mut bytes);
        Nonce(NONCE_ENGINE.encode(bytes).into())
    }
}

impl Default for Nonce {
    fn default() -> Self {
        Self::new()
    }
}



================================================
FILE: leptos/src/portal.rs
================================================
use crate::{children::TypedChildrenFn, mount, IntoView};
use leptos_dom::helpers::document;
use leptos_macro::component;
use reactive_graph::{effect::Effect, graph::untrack, owner::Owner};
use std::sync::Arc;

/// Renders components somewhere else in the DOM.
///
/// Useful for inserting modals and tooltips outside of a cropping layout.
/// If no mount point is given, the portal is inserted in `document.body`;
/// it is wrapped in a `<div>` unless  `is_svg` is `true` in which case it's wrapped in a `<g>`.
/// Setting `use_shadow` to `true` places the element in a shadow root to isolate styles.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
#[component]
pub fn Portal<V>(
    /// Target element where the children will be appended
    #[prop(into, optional)]
    mount: Option<web_sys::Element>,
    /// Whether to use a shadow DOM inside `mount`. Defaults to `false`.
    #[prop(optional)]
    use_shadow: bool,
    /// When using SVG this has to be set to `true`. Defaults to `false`.
    #[prop(optional)]
    is_svg: bool,
    /// The children to teleport into the `mount` element
    children: TypedChildrenFn<V>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    if cfg!(target_arch = "wasm32")
        && Owner::current_shared_context()
            .map(|sc| sc.is_browser())
            .unwrap_or(true)
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::JsCast;

        let mount = mount.unwrap_or_else(|| {
            document().body().expect("body to exist").unchecked_into()
        });
        let children = children.into_inner();

        Effect::new(move |_| {
            let container = if is_svg {
                document()
                    .create_element_ns(Some("http://www.w3.org/2000/svg"), "g")
                    .expect("SVG element creation to work")
            } else {
                document()
                    .create_element("div")
                    .expect("HTML element creation to work")
            };

            let render_root = if use_shadow {
                container
                    .attach_shadow(&web_sys::ShadowRootInit::new(
                        web_sys::ShadowRootMode::Open,
                    ))
                    .map(|root| root.unchecked_into())
                    .unwrap_or(container.clone())
            } else {
                container.clone()
            };

            let _ = mount.append_child(&container);
            let handle = SendWrapper::new((
                mount::mount_to(render_root.unchecked_into(), {
                    let children = Arc::clone(&children);
                    move || untrack(|| children())
                }),
                mount.clone(),
                container,
            ));

            Owner::on_cleanup({
                move || {
                    let (handle, mount, container) = handle.take();
                    drop(handle);
                    let _ = mount.remove_child(&container);
                }
            })
        });
    }
}



================================================
FILE: leptos/src/provider.rs
================================================
use crate::{children::TypedChildren, component, IntoView};
use reactive_graph::owner::{provide_context, Owner};
use tachys::reactive_graph::OwnedView;

#[component]
/// Uses the context API to [`provide_context`] to its children and descendants,
/// without overwriting any contexts of the same type in its own reactive scope.
///
/// This prevents issues related to “context shadowing.”
///
/// ```rust
/// use leptos::{context::Provider, prelude::*};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     // each Provider will only provide the value to its children
///     view! {
///         <Provider value=1u8>
///             // correctly gets 1 from context
///             {use_context::<u8>().unwrap_or(0)}
///         </Provider>
///         <Provider value=2u8>
///             // correctly gets 2 from context
///             {use_context::<u8>().unwrap_or(0)}
///         </Provider>
///         // does not find any u8 in context
///         {use_context::<u8>().unwrap_or(0)}
///     }
/// }
/// ```
pub fn Provider<T, Chil>(
    /// The value to be provided via context.
    value: T,
    children: TypedChildren<Chil>,
) -> impl IntoView
where
    T: Send + Sync + 'static,
    Chil: IntoView + 'static,
{
    let owner = Owner::current()
        .expect("no current reactive Owner found")
        .child();
    let children = children.into_inner();
    let children = owner.with(|| {
        provide_context(value);
        children()
    });
    OwnedView::new_with_owner(children, owner)
}



================================================
FILE: leptos/src/show.rs
================================================
use crate::{
    children::{TypedChildrenFn, ViewFn},
    IntoView,
};
use leptos_macro::component;
use reactive_graph::{computed::ArcMemo, traits::Get};
use tachys::either::Either;

#[component]
pub fn Show<W, C>(
    /// The children will be shown whenever the condition in the `when` closure returns `true`.
    children: TypedChildrenFn<C>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false. By default this is the empty view.
    #[prop(optional, into)]
    fallback: ViewFn,
) -> impl IntoView
where
    W: Fn() -> bool + Send + Sync + 'static,
    C: IntoView + 'static,
{
    let memoized_when = ArcMemo::new(move |_| when());
    let children = children.into_inner();

    move || match memoized_when.get() {
        true => Either::Left(children()),
        false => Either::Right(fallback.run()),
    }
}



================================================
FILE: leptos/src/suspense_component.rs
================================================
use crate::{
    children::{TypedChildren, ViewFnOnce},
    error::ErrorBoundarySuspendedChildren,
    IntoView,
};
use futures::{channel::oneshot, select, FutureExt};
use hydration_context::SerializedDataId;
use leptos_macro::component;
use reactive_graph::{
    computed::{
        suspense::{LocalResourceNotifier, SuspenseContext},
        ArcMemo, ScopedFuture,
    },
    effect::RenderEffect,
    owner::{provide_context, use_context, Owner},
    signal::ArcRwSignal,
    traits::{Dispose, Get, Read, Track, With, WriteValue},
};
use slotmap::{DefaultKey, SlotMap};
use std::sync::Arc;
use tachys::{
    either::Either,
    html::attribute::{any_attribute::AnyAttribute, Attribute},
    hydration::Cursor,
    reactive_graph::{OwnedView, OwnedViewState},
    ssr::StreamBuilder,
    view::{
        add_attr::AddAnyAttr,
        either::{EitherKeepAlive, EitherKeepAliveState},
        Mountable, Position, PositionState, Render, RenderHtml,
    },
};
use throw_error::ErrorHookFuture;

/// If any [`Resource`](leptos_reactive::Resource) is read in the `children` of this
/// component, it will show the `fallback` while they are loading. Once all are resolved,
/// it will render the `children`.
///
/// Each time one of the resources is loading again, it will fall back. To keep the current
/// children instead, use [Transition](crate::Transition).
///
/// Note that the `children` will be rendered initially (in order to capture the fact that
/// those resources are read under the suspense), so you cannot assume that resources read
/// synchronously have
/// `Some` value in `children`. However, you can read resources asynchronously by using
/// [Suspend](crate::prelude::Suspend).
///
/// ```
/// # use leptos::prelude::*;
/// # if false { // don't run in doctests
/// async fn fetch_cats(how_many: u32) -> Vec<String> { vec![] }
///
/// let (cat_count, set_cat_count) = signal::<u32>(1);
///
/// let cats = Resource::new(move || cat_count.get(), |count| fetch_cats(count));
///
/// view! {
///   <div>
///     <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
///       // you can access a resource synchronously
///       {move || {
///           cats.get().map(|data| {
///             data
///               .into_iter()
///               .map(|src| {
///                   view! {
///                     <img src={src}/>
///                   }
///               })
///               .collect_view()
///           })
///         }
///       }
///       // or you can use `Suspend` to read resources asynchronously
///       {move || Suspend::new(async move {
///         cats.await
///               .into_iter()
///               .map(|src| {
///                   view! {
///                     <img src={src}/>
///                   }
///               })
///               .collect_view()
///       })}
///     </Suspense>
///   </div>
/// }
/// # ;}
/// ```
#[component]
pub fn Suspense<Chil>(
    /// A function that returns a fallback that will be shown while resources are still loading.
    /// By default this is an empty view.
    #[prop(optional, into)]
    fallback: ViewFnOnce,
    /// Children will be rendered once initially to catch any resource reads, then hidden until all
    /// data have loaded.
    children: TypedChildren<Chil>,
) -> impl IntoView
where
    Chil: IntoView + Send + 'static,
{
    let error_boundary_parent = use_context::<ErrorBoundarySuspendedChildren>();

    let owner = Owner::new();
    owner.with(|| {
        let (starts_local, id) = {
            Owner::current_shared_context()
                .map(|sc| {
                    let id = sc.next_id();
                    (sc.get_incomplete_chunk(&id), id)
                })
                .unwrap_or_else(|| (false, Default::default()))
        };
        let fallback = fallback.run();
        let children = children.into_inner()();
        let tasks = ArcRwSignal::new(SlotMap::<DefaultKey, ()>::new());
        provide_context(SuspenseContext {
            tasks: tasks.clone(),
        });
        let none_pending = ArcMemo::new(move |prev: Option<&bool>| {
            tasks.track();
            if prev.is_none() && starts_local {
                false
            } else {
                tasks.with(SlotMap::is_empty)
            }
        });

        OwnedView::new(SuspenseBoundary::<false, _, _> {
            id,
            none_pending,
            fallback,
            children,
            error_boundary_parent,
        })
    })
}

fn nonce_or_not() -> Option<Arc<str>> {
    #[cfg(feature = "nonce")]
    {
        use crate::nonce::Nonce;
        use_context::<Nonce>().map(|n| n.0)
    }
    #[cfg(not(feature = "nonce"))]
    {
        None
    }
}

pub(crate) struct SuspenseBoundary<const TRANSITION: bool, Fal, Chil> {
    pub id: SerializedDataId,
    pub none_pending: ArcMemo<bool>,
    pub fallback: Fal,
    pub children: Chil,
    pub error_boundary_parent: Option<ErrorBoundarySuspendedChildren>,
}

impl<const TRANSITION: bool, Fal, Chil> Render
    for SuspenseBoundary<TRANSITION, Fal, Chil>
where
    Fal: Render + Send + 'static,
    Chil: Render + Send + 'static,
{
    type State = RenderEffect<
        OwnedViewState<EitherKeepAliveState<Chil::State, Fal::State>>,
    >;

    fn build(self) -> Self::State {
        let mut children = Some(self.children);
        let mut fallback = Some(self.fallback);
        let none_pending = self.none_pending;
        let mut nth_run = 0;
        let outer_owner = Owner::new();

        RenderEffect::new(move |prev| {
            // show the fallback if
            // 1) there are pending futures, and
            // 2) we are either in a Suspense (not Transition), or it's the first fallback
            //    (because we initially render the children to register Futures, the "first
            //    fallback" is probably the 2nd run
            let show_b = !none_pending.get() && (!TRANSITION || nth_run < 2);
            nth_run += 1;
            let this = OwnedView::new_with_owner(
                EitherKeepAlive {
                    a: children.take(),
                    b: fallback.take(),
                    show_b,
                },
                outer_owner.clone(),
            );

            if let Some(mut state) = prev {
                this.rebuild(&mut state);
                state
            } else {
                this.build()
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {
        let new = self.build();
        let mut old = std::mem::replace(state, new);
        old.insert_before_this(state);
        old.unmount();
    }
}

impl<const TRANSITION: bool, Fal, Chil> AddAnyAttr
    for SuspenseBoundary<TRANSITION, Fal, Chil>
where
    Fal: RenderHtml + Send + 'static,
    Chil: RenderHtml + Send + 'static,
{
    type Output<SomeNewAttr: Attribute> = SuspenseBoundary<
        TRANSITION,
        Fal,
        Chil::Output<SomeNewAttr::CloneableOwned>,
    >;

    fn add_any_attr<NewAttr: Attribute>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        let attr = attr.into_cloneable_owned();
        let SuspenseBoundary {
            id,
            none_pending,
            fallback,
            children,
            error_boundary_parent,
        } = self;
        SuspenseBoundary {
            id,
            none_pending,
            fallback,
            children: children.add_any_attr(attr),
            error_boundary_parent,
        }
    }
}

impl<const TRANSITION: bool, Fal, Chil> RenderHtml
    for SuspenseBoundary<TRANSITION, Fal, Chil>
where
    Fal: RenderHtml + Send + 'static,
    Chil: RenderHtml + Send + 'static,
{
    // i.e., if this is the child of another Suspense during SSR, don't wait for it: it will handle
    // itself
    type AsyncOutput = Self;
    type Owned = Self;

    const MIN_LENGTH: usize = Chil::MIN_LENGTH;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        self.fallback.to_html_with_buf(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        mut self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) where
        Self: Sized,
    {
        buf.next_id();
        let suspense_context = use_context::<SuspenseContext>().unwrap();
        let owner = Owner::current().unwrap();

        let mut notify_error_boundary =
            self.error_boundary_parent.map(|children| {
                let (tx, rx) = oneshot::channel();
                children.write_value().push(rx);
                tx
            });

        // we need to wait for one of two things: either
        // 1. all tasks are finished loading, or
        // 2. we read from a local resource, meaning this Suspense can never resolve on the server

        // first, create listener for tasks
        let tasks = suspense_context.tasks.clone();
        let (tasks_tx, mut tasks_rx) =
            futures::channel::oneshot::channel::<()>();

        let mut tasks_tx = Some(tasks_tx);

        // now, create listener for local resources
        let (local_tx, mut local_rx) =
            futures::channel::oneshot::channel::<()>();
        provide_context(LocalResourceNotifier::from(local_tx));

        // walk over the tree of children once to make sure that all resource loads are registered
        self.children.dry_resolve();

        // check the set of tasks to see if it is empty, now or later
        let eff = reactive_graph::effect::Effect::new_isomorphic({
            move |_| {
                tasks.track();
                if let Some(tasks) = tasks.try_read() {
                    if tasks.is_empty() {
                        if let Some(tx) = tasks_tx.take() {
                            // If the receiver has dropped, it means the ScopedFuture has already
                            // dropped, so it doesn't matter if we manage to send this.
                            _ = tx.send(());
                        }
                        if let Some(tx) = notify_error_boundary.take() {
                            _ = tx.send(());
                        }
                    }
                }
            }
        });

        let mut fut = Box::pin(ScopedFuture::new(ErrorHookFuture::new(
            async move {
                // race the local resource notifier against the set of tasks
                //
                // if there are local resources, we just return the fallback immediately
                //
                // otherwise, we want to wait for resources to load before trying to resolve the body
                //
                // this is *less efficient* than just resolving the body
                // however, it means that you can use reactive accesses to resources/async derived
                // inside component props, at any level, and have those picked up by Suspense, and
                // that it will wait for those to resolve
                select! {
                    // if there are local resources, bail
                    // this will only have fired by this point for local resources accessed
                    // *synchronously*
                    _ = local_rx => {
                        let sc = Owner::current_shared_context().expect("no shared context");
                        sc.set_incomplete_chunk(self.id);
                        None
                    }
                    _ = tasks_rx => {
                        // if we ran this earlier, reactive reads would always be registered as None
                        // this is fine in the case where we want to use Suspend and .await on some future
                        // but in situations like a <For each=|| some_resource.snapshot()/> we actually
                        // want to be able to 1) synchronously read a resource's value, but still 2) wait
                        // for it to load before we render everything
                        let mut children = Box::pin(self.children.resolve().fuse());

                        // we continue racing the children against the "do we have any local
                        // resources?" Future
                        select! {
                            _ = local_rx => {
                                let sc = Owner::current_shared_context().expect("no shared context");
                                sc.set_incomplete_chunk(self.id);
                                None
                            }
                            children = children => {
                                // clean up the (now useless) effect
                                eff.dispose();

                                Some(OwnedView::new_with_owner(children, owner))
                            }
                        }
                    }
                }
            },
        )));
        match fut.as_mut().now_or_never() {
            Some(Some(resolved)) => {
                Either::<Fal, _>::Right(resolved)
                    .to_html_async_with_buf::<OUT_OF_ORDER>(
                        buf,
                        position,
                        escape,
                        mark_branches,
                        extra_attrs,
                    );
            }
            Some(None) => {
                Either::<_, Chil>::Left(self.fallback)
                    .to_html_async_with_buf::<OUT_OF_ORDER>(
                        buf,
                        position,
                        escape,
                        mark_branches,
                        extra_attrs,
                    );
            }
            None => {
                let id = buf.clone_id();

                // out-of-order streams immediately push fallback,
                // wrapped by suspense markers
                if OUT_OF_ORDER {
                    let mut fallback_position = *position;
                    buf.push_fallback(
                        self.fallback,
                        &mut fallback_position,
                        mark_branches,
                        extra_attrs.clone(),
                    );
                    buf.push_async_out_of_order_with_nonce(
                        fut,
                        position,
                        mark_branches,
                        nonce_or_not(),
                        extra_attrs,
                    );
                } else {
                    // calling this will walk over the tree, removing all event listeners
                    // and other single-threaded values from the view tree. this needs to be
                    // done because the fallback can be shifted to another thread in push_async below.
                    self.fallback.dry_resolve();

                    buf.push_async({
                        let mut position = *position;
                        async move {
                            let value = match fut.await {
                                None => Either::Left(self.fallback),
                                Some(value) => Either::Right(value),
                            };
                            let mut builder = StreamBuilder::new(id);
                            value.to_html_async_with_buf::<OUT_OF_ORDER>(
                                &mut builder,
                                &mut position,
                                escape,
                                mark_branches,
                                extra_attrs,
                            );
                            builder.finish().take_chunks()
                        }
                    });
                    *position = Position::NextChild;
                }
            }
        };
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let cursor = cursor.to_owned();
        let position = position.to_owned();

        let mut children = Some(self.children);
        let mut fallback = Some(self.fallback);
        let none_pending = self.none_pending;
        let mut nth_run = 0;
        let outer_owner = Owner::new();

        RenderEffect::new(move |prev| {
            // show the fallback if
            // 1) there are pending futures, and
            // 2) we are either in a Suspense (not Transition), or it's the first fallback
            //    (because we initially render the children to register Futures, the "first
            //    fallback" is probably the 2nd run
            let show_b = !none_pending.get() && (!TRANSITION || nth_run < 1);
            nth_run += 1;
            let this = OwnedView::new_with_owner(
                EitherKeepAlive {
                    a: children.take(),
                    b: fallback.take(),
                    show_b,
                },
                outer_owner.clone(),
            );

            if let Some(mut state) = prev {
                this.rebuild(&mut state);
                state
            } else {
                this.hydrate::<FROM_SERVER>(&cursor, &position)
            }
        })
    }

    fn into_owned(self) -> Self::Owned {
        self
    }
}

/// A wrapper that prevents [`Suspense`] from waiting for any resource reads that happen inside
/// `Unsuspend`.
pub struct Unsuspend<T>(Box<dyn FnOnce() -> T + Send>);

impl<T> Unsuspend<T> {
    /// Wraps the given function, such that it is not called until all resources are ready.
    pub fn new(fun: impl FnOnce() -> T + Send + 'static) -> Self {
        Self(Box::new(fun))
    }
}

impl<T> Render for Unsuspend<T>
where
    T: Render,
{
    type State = T::State;

    fn build(self) -> Self::State {
        (self.0)().build()
    }

    fn rebuild(self, state: &mut Self::State) {
        (self.0)().rebuild(state);
    }
}

impl<T> AddAnyAttr for Unsuspend<T>
where
    T: AddAnyAttr + 'static,
{
    type Output<SomeNewAttr: Attribute> =
        Unsuspend<T::Output<SomeNewAttr::CloneableOwned>>;

    fn add_any_attr<NewAttr: Attribute>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        let attr = attr.into_cloneable_owned();
        Unsuspend::new(move || (self.0)().add_any_attr(attr))
    }
}

impl<T> RenderHtml for Unsuspend<T>
where
    T: RenderHtml + 'static,
{
    type AsyncOutput = Self;
    type Owned = Self;

    const MIN_LENGTH: usize = T::MIN_LENGTH;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        (self.0)().to_html_with_buf(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) where
        Self: Sized,
    {
        (self.0)().to_html_async_with_buf::<OUT_OF_ORDER>(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        (self.0)().hydrate::<FROM_SERVER>(cursor, position)
    }

    fn into_owned(self) -> Self::Owned {
        self
    }
}



================================================
FILE: leptos/src/text_prop.rs
================================================
use oco_ref::Oco;
use std::sync::Arc;
use tachys::prelude::IntoAttributeValue;

/// Describes a value that is either a static or a reactive string, i.e.,
/// a [`String`], a [`&str`], a `Signal` or a reactive `Fn() -> String`.
#[derive(Clone)]
pub struct TextProp(Arc<dyn Fn() -> Oco<'static, str> + Send + Sync>);

impl TextProp {
    /// Accesses the current value of the property.
    #[inline(always)]
    pub fn get(&self) -> Oco<'static, str> {
        (self.0)()
    }
}

impl core::fmt::Debug for TextProp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextProp").finish()
    }
}

impl From<String> for TextProp {
    fn from(s: String) -> Self {
        let s: Oco<'_, str> = Oco::Counted(Arc::from(s));
        TextProp(Arc::new(move || s.clone()))
    }
}

impl From<&'static str> for TextProp {
    fn from(s: &'static str) -> Self {
        let s: Oco<'_, str> = s.into();
        TextProp(Arc::new(move || s.clone()))
    }
}

impl From<Arc<str>> for TextProp {
    fn from(s: Arc<str>) -> Self {
        let s: Oco<'_, str> = s.into();
        TextProp(Arc::new(move || s.clone()))
    }
}

impl From<Oco<'static, str>> for TextProp {
    fn from(s: Oco<'static, str>) -> Self {
        TextProp(Arc::new(move || s.clone()))
    }
}

// TODO
/*impl<T> From<T> for MaybeProp<TextProp>
where
    T: Into<Oco<'static, str>>,
{
    fn from(s: T) -> Self {
        Self(Some(MaybeSignal::from(Some(s.into().into()))))
    }
}*/

impl<F, S> From<F> for TextProp
where
    F: Fn() -> S + 'static + Send + Sync,
    S: Into<Oco<'static, str>>,
{
    #[inline(always)]
    fn from(s: F) -> Self {
        TextProp(Arc::new(move || s().into()))
    }
}

impl Default for TextProp {
    fn default() -> Self {
        Self(Arc::new(|| Oco::Borrowed("")))
    }
}

impl IntoAttributeValue for TextProp {
    type Output = Arc<dyn Fn() -> Oco<'static, str> + Send + Sync>;

    fn into_attribute_value(self) -> Self::Output {
        self.0
    }
}

macro_rules! textprop_reactive {
    ($name:ident, <$($gen:ident),*>, $v:ty, $( $where_clause:tt )*) =>
    {
        #[allow(deprecated)]
        impl<$($gen),*> From<$name<$($gen),*>> for TextProp
        where
            $v: Into<Oco<'static, str>>  + Clone + Send + Sync + 'static,
            $($where_clause)*
        {
            #[inline(always)]
            fn from(s: $name<$($gen),*>) -> Self {
                TextProp(Arc::new(move || s.get().into()))
            }
        }
    };
}

#[cfg(not(feature = "nightly"))]
mod stable {
    use super::TextProp;
    use oco_ref::Oco;
    #[allow(deprecated)]
    use reactive_graph::wrappers::read::MaybeSignal;
    use reactive_graph::{
        computed::{ArcMemo, Memo},
        owner::Storage,
        signal::{ArcReadSignal, ArcRwSignal, ReadSignal, RwSignal},
        traits::Get,
        wrappers::read::{ArcSignal, Signal},
    };
    use std::sync::Arc;

    textprop_reactive!(
        RwSignal,
        <V, S>,
        V,
        RwSignal<V, S>: Get<Value = V>,
        S: Storage<V> + Storage<Option<V>>,
        S: Send + Sync + 'static,
    );
    textprop_reactive!(
        ReadSignal,
        <V, S>,
        V,
        ReadSignal<V, S>: Get<Value = V>,
        S: Storage<V> + Storage<Option<V>>,
        S: Send + Sync + 'static,
    );
    textprop_reactive!(
        Memo,
        <V, S>,
        V,
        Memo<V, S>: Get<Value = V>,
        S: Storage<V> + Storage<Option<V>>,
        S: Send + Sync + 'static,
    );
    textprop_reactive!(
        Signal,
        <V, S>,
        V,
        Signal<V, S>: Get<Value = V>,
        S: Storage<V> + Storage<Option<V>>,
        S: Send + Sync + 'static,
    );
    textprop_reactive!(
        MaybeSignal,
        <V, S>,
        V,
        MaybeSignal<V, S>: Get<Value = V>,
        S: Storage<V> + Storage<Option<V>>,
        S: Send + Sync + 'static,
    );
    textprop_reactive!(ArcRwSignal, <V>, V, ArcRwSignal<V>: Get<Value = V>);
    textprop_reactive!(ArcReadSignal, <V>, V, ArcReadSignal<V>: Get<Value = V>);
    textprop_reactive!(ArcMemo, <V>, V, ArcMemo<V>: Get<Value = V>);
    textprop_reactive!(ArcSignal, <V>, V, ArcSignal<V>: Get<Value = V>);
}

/// Extension trait for `Option<TextProp>`
pub trait OptionTextPropExt {
    /// Accesses the current value of the `Option<TextProp>` as an `Option<Oco<'static, str>>`.
    fn get(&self) -> Option<Oco<'static, str>>;
}

impl OptionTextPropExt for Option<TextProp> {
    fn get(&self) -> Option<Oco<'static, str>> {
        self.as_ref().map(|text_prop| text_prop.get())
    }
}



================================================
FILE: leptos/src/transition.rs
================================================
use crate::{
    children::{TypedChildren, ViewFnOnce},
    error::ErrorBoundarySuspendedChildren,
    suspense_component::SuspenseBoundary,
    IntoView,
};
use leptos_macro::component;
use reactive_graph::{
    computed::{suspense::SuspenseContext, ArcMemo},
    effect::Effect,
    owner::{provide_context, use_context, Owner},
    signal::ArcRwSignal,
    traits::{Get, Set, Track, With},
    wrappers::write::SignalSetter,
};
use slotmap::{DefaultKey, SlotMap};
use tachys::reactive_graph::OwnedView;

/// If any [`Resource`](leptos_reactive::Resource) is read in the `children` of this
/// component, it will show the `fallback` while they are loading. Once all are resolved,
/// it will render the `children`.
///
/// Unlike [`Suspense`](crate::Suspense), this will not fall
/// back to the `fallback` state if there are further changes after the initial load.
///
/// Note that the `children` will be rendered initially (in order to capture the fact that
/// those resources are read under the suspense), so you cannot assume that resources read
/// synchronously have
/// `Some` value in `children`. However, you can read resources asynchronously by using
/// [Suspend](crate::prelude::Suspend).
///
/// ```
/// # use leptos::prelude::*;
/// # if false { // don't run in doctests
/// async fn fetch_cats(how_many: u32) -> Vec<String> { vec![] }
///
/// let (cat_count, set_cat_count) = signal::<u32>(1);
///
/// let cats = Resource::new(move || cat_count.get(), |count| fetch_cats(count));
///
/// view! {
///   <div>
///     <Transition fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
///       // you can access a resource synchronously
///       {move || {
///           cats.get().map(|data| {
///             data
///               .into_iter()
///               .map(|src| {
///                   view! {
///                     <img src={src}/>
///                   }
///               })
///               .collect_view()
///           })
///         }
///       }
///       // or you can use `Suspend` to read resources asynchronously
///       {move || Suspend::new(async move {
///         cats.await
///               .into_iter()
///               .map(|src| {
///                   view! {
///                     <img src={src}/>
///                   }
///               })
///               .collect_view()
///       })}
///     </Transition>
///   </div>
/// }
/// # ;}
/// ```
#[component]
pub fn Transition<Chil>(
    /// Will be displayed while resources are pending. By default this is the empty view.
    #[prop(optional, into)]
    fallback: ViewFnOnce,
    /// A function that will be called when the component transitions into or out of
    /// the `pending` state, with its argument indicating whether it is pending (`true`)
    /// or not pending (`false`).
    #[prop(optional, into)]
    set_pending: Option<SignalSetter<bool>>,
    children: TypedChildren<Chil>,
) -> impl IntoView
where
    Chil: IntoView + Send + 'static,
{
    let error_boundary_parent = use_context::<ErrorBoundarySuspendedChildren>();

    let owner = Owner::new();
    owner.with(|| {
        let (starts_local, id) = {
            Owner::current_shared_context()
                .map(|sc| {
                    let id = sc.next_id();
                    (sc.get_incomplete_chunk(&id), id)
                })
                .unwrap_or_else(|| (false, Default::default()))
        };
        let fallback = fallback.run();
        let children = children.into_inner()();
        let tasks = ArcRwSignal::new(SlotMap::<DefaultKey, ()>::new());
        provide_context(SuspenseContext {
            tasks: tasks.clone(),
        });
        let none_pending = ArcMemo::new(move |prev: Option<&bool>| {
            tasks.track();
            if prev.is_none() && starts_local {
                false
            } else {
                tasks.with(SlotMap::is_empty)
            }
        });
        if let Some(set_pending) = set_pending {
            Effect::new_isomorphic({
                let none_pending = none_pending.clone();
                move |_| {
                    set_pending.set(!none_pending.get());
                }
            });
        }

        OwnedView::new(SuspenseBoundary::<true, _, _> {
            id,
            none_pending,
            fallback,
            children,
            error_boundary_parent,
        })
    })
}



================================================
FILE: leptos/src/hydration/hydration_script.js
================================================
(function (root, pkg_path, output_name, wasm_output_name) {
	import(`${root}/${pkg_path}/${output_name}.js`)
		.then(mod => {
			mod.default({module_or_path: `${root}/${pkg_path}/${wasm_output_name}.wasm`}).then(() => {
				mod.hydrate();
			});
		})
})



================================================
FILE: leptos/src/hydration/island_script.js
================================================
((root, pkg_path, output_name, wasm_output_name) => {
	let MOST_RECENT_CHILDREN_CB = [];

	function idle(c) {
		if ("requestIdleCallback" in window) {
			window.requestIdleCallback(c);
		} else {
			c();
		}
	}
	async function hydrateIslands(rootNode, mod) {
		async function traverse(node) {
			if (node.nodeType === Node.ELEMENT_NODE) {
				const tag = node.tagName.toLowerCase();
				if(tag === 'leptos-island') {
					const children = [];
					const id = node.dataset.component || null;

					await hydrateIsland(node, id, mod);
					
					for(const child of node.children) {
						await traverse(child, children);
					}
				} else {
					if (tag === 'leptos-children') {
						MOST_RECENT_CHILDREN_CB.push(node.$$on_hydrate);
						for(const child of node.children) {
							await traverse(child);
						};
						// un-set the "most recent children"
						MOST_RECENT_CHILDREN_CB.pop();
					} else {
						for(const child of node.children) {
							await traverse(child);
						};
					}
				}
			}
		}

		await traverse(rootNode);
	}
	async function hydrateIsland(el, id, mod) {
		const islandFn = mod[id];
		if (islandFn) {
			const children_cb = MOST_RECENT_CHILDREN_CB[MOST_RECENT_CHILDREN_CB.length-1];
			if (children_cb) {
				children_cb();
			}
			const res = islandFn(el);
			if (res && res.then) {
				await res;
			}
		} else {
			console.warn(`Could not find WASM function for the island ${id}.`);
		}
	}
	idle(() => {
		import(`${root}/${pkg_path}/${output_name}.js`)
			.then(mod => {
				mod.default({module_or_path: `${root}/${pkg_path}/${wasm_output_name}.wasm`}).then(() => {
					mod.hydrate();
					hydrateIslands(document.body, mod);
				});

				window.__hydrateIsland = (el, id) => hydrateIsland(el, id, mod);
			})
	});
})



================================================
FILE: leptos/src/hydration/islands_routing.js
================================================
let NAVIGATION = 0;

window.addEventListener("click", async (ev) => {
	const req = clickToReq(ev);
	if(!req) {
		return;
	}

	ev.preventDefault();
	await navigateToPage(req, true);
});

window.addEventListener("popstate", async (ev) => {
	const req = new Request(window.location);
	ev.preventDefault();
	await navigateToPage(req, true, true);
});

window.addEventListener("submit", async (ev) => {
	if (ev.defaultPrevented) {
		return;
	}
		
	const req = submitToReq(ev);
	if(!req) {
		return;
	}

	ev.preventDefault();
	await navigateToPage(req, true);
});

async function navigateToPage(
	/** @type Request */
	req, 
	/** @type bool */
	useViewTransition, 
	/** @type bool */
	replace
) {
	NAVIGATION += 1;
	const currentNav = NAVIGATION;

	// add a custom header to indicate that we're on a subsequent navigation 
	req.headers.append("Islands-Router", "true");

	// fetch the new page
	const resp = await fetch(req);
	const redirected = resp.redirected;
	const htmlString = await resp.text();

	if(NAVIGATION === currentNav) {
		// The 'doc' variable now contains the parsed DOM
		const transition = async () => {
			try {
				diffPages(htmlString);
				for(const island of document.querySelectorAll("leptos-island")) {
					if(!island.$$hydrated) {
						__hydrateIsland(island, island.dataset.component);
						island.$$hydrated = true;
					}
				}
			} catch(e) {
				console.error(e);
			}
		};
		// Not all browsers support startViewTransition; see https://caniuse.com/?search=startViewTransition
		if (useViewTransition && document.startViewTransition) {
			await document.startViewTransition(transition);
		} else {
			await transition()
		}

		const url = redirected ? resp.url : req.url;

		if(replace) {
			window.history.replaceState(undefined, null, url);
		} else {
			window.history.pushState(undefined, null, url);
		}
	}
}

function clickToReq(ev) {
	// confirm that this is an <a> that meets our requirements
	if (
		ev.defaultPrevented ||
		ev.button !== 0 ||
		ev.metaKey ||
		ev.altKey ||
		ev.ctrlKey ||
		ev.shiftKey
	      )
        return;

      /** @type HTMLAnchorElement | undefined;*/
      const a = ev
        .composedPath()
        .find(el => el instanceof Node && el.nodeName.toUpperCase() === "A");

	if (!a) return;

     const svg = a.namespaceURI === "http://www.w3.org/2000/svg";
	const href = svg ? a.href.baseVal : a.href;
      const target = svg ? a.target.baseVal : a.target;
      if (target || (!href && !a.hasAttribute("state"))) return;

      const rel = (a.getAttribute("rel") || "").split(/\s+/);
      if (a.hasAttribute("download") || (rel?.includes("external"))) return;

      const url = svg ? new URL(href, document.baseURI) : new URL(href);
      if (
        url.origin !== window.location.origin // ||  
	      // TODO base
        //(basePath && url.pathname && !url.pathname.toLowerCase().startsWith(basePath.toLowerCase()))
      )
        return; 

      return new Request(url);
}

function submitToReq(ev) {
	event.preventDefault();

	const target = ev.target;
	/** @type HTMLFormElement */
	let form;
	if(target instanceof HTMLFormElement) {
		form = target;
	} else {
		if(!target.form) {
			return;
		}
		form = target.form;
	}

	const method = form.method.toUpperCase();
	if(method !== "GET" && method !== "POST") {
		return;
	}

	const url = new URL(form.action);
	let path = url.pathname;
	const requestInit = {};
	const data = new FormData(form);

	const params = new URLSearchParams();
	for (const [key, value] of data.entries()) {
		params.append(key, value);
	}

	requestInit.headers = { 
		Accept: "text/html"
	};
	if(method === "GET") {
		path += `?${params.toString()}`;
	}
	else {
		requestInit.method = "POST";
		requestInit.body = params; 
	}

	return new Request(
		path,
		requestInit
	);
}


function diffPages(htmlString) {
	// Use DOMParser to parse the HTML string
	const parser = new DOMParser();
	const doc = parser.parseFromString(htmlString, 'text/html');

	diffRange(document, document, doc, doc);
}

function diffRange(oldDocument, oldRoot, newDocument, newRoot, oldEnd, newEnd) {
	const oldDocWalker = oldDocument.createTreeWalker(oldRoot);
	const newDocWalker = newDocument.createTreeWalker(newRoot);
	let oldNode = oldDocWalker.currentNode;
	let newNode = newDocWalker.currentNode;

	while (oldDocWalker.nextNode() && newDocWalker.nextNode()) {
		oldNode = oldDocWalker.currentNode;
		newNode = newDocWalker.currentNode;

		if (oldNode == oldEnd || newNode == newEnd) {
			break;
		}

		// if the nodes are different, we need to replace the old with the new
		// because of the typed view tree, this should never actually happen
		if (oldNode.nodeType !== newNode.nodeType) {
			oldNode.replaceWith(newNode);
		}
		// if it's a text node, just update the text with the new text
		else if (oldNode.nodeType === Node.TEXT_NODE) {
			oldNode.textContent = newNode.textContent;
		}
		// islands should not be diffed on the client, because we do not want to overwrite client-side state 
		// but their children should be diffed still, because they could contain new server content
		else if (oldNode.nodeType === Node.ELEMENT_NODE && oldNode.tagName === "LEPTOS-ISLAND") {
			// TODO: diff the leptos-children 

			// skip over leptos-island otherwise
			oldDocWalker.nextSibling();
			newDocWalker.nextSibling();
		}
		// if it's an element, replace if it's a different tag, or update attributes
		else if (oldNode.nodeType === Node.ELEMENT_NODE) {
			diffElement(oldNode, newNode);
		}
		// we use comment "branch marker" nodes to distinguish between different branches in the statically-typed view tree
		// if one of these marker is hit, then there are two options
		// 1) it's the same branch, and we just keep walking until the end 
		// 2) it's a different branch, in which case the old can be replaced with the new wholesale
		else if (oldNode.nodeType === Node.COMMENT_NODE) {
			const oldText = oldNode.textContent;
			const newText = newNode.textContent;
			if(oldText.startsWith("bo-for")) {
				replaceFor(oldDocument, oldDocWalker, newDocument, newDocWalker, oldNode, newNode);
			}
			else if (oldText.startsWith("bo-item")) {
				// skip, this means we're diffing a new item within a For
			}
			else if(oldText.startsWith("bo") && newText !== oldText) {
				replaceBranch(oldDocWalker, newDocWalker, oldNode, newNode);
			}
		}
	}
}

function replaceFor(oldDocument, oldDocWalker, newDocument, newDocWalker, oldNode, newNode) {
	oldDocWalker.nextNode();
	newDocWalker.nextNode();
	const oldRange = new Range();
	const newRange = new Range();
	let oldBranches = 1;
	let newBranches = 1;

	const oldKeys = {};
	const newKeys = {};

	while(oldBranches > 0) {
		const c = oldDocWalker.currentNode;
		if(c.nodeType === Node.COMMENT_NODE) {
			const t = c.textContent;
			if(t.startsWith("bo-for")) {
				oldBranches += 1;
			} else if(t.startsWith("bc-for")) {

				oldBranches -= 1;
			} else if (t.startsWith("bo-item")) {
				const k = t.replace("bo-item-", "");
				oldKeys[k] = { open: c, close: null };
			} else if (t.startsWith("bc-item")) {
				const k = t.replace("bc-item-", "");
				oldKeys[k].close = c;
			}
		}
		oldDocWalker.nextNode();
	}
	while(newBranches > 0) {
		const c = newDocWalker.currentNode;
		if(c.nodeType === Node.COMMENT_NODE) {
			const t = c.textContent;
			if(t.startsWith("bo-for")) {
				newBranches += 1;
			} else if(t.startsWith("bc-for")) {

				newBranches -= 1;
			} else if (t.startsWith("bo-item")) {
				const k = t.replace("bo-item-", "");
				newKeys[k] = { open: c, close: null };
			} else if (t.startsWith("bc-item")) {
				const k = t.replace("bc-item-", "");
				newKeys[k].close = c;
			}
		}
		newDocWalker.nextNode();
	}

	for(const key in oldKeys) {
		if(newKeys[key]) {
			const oldOne = oldKeys[key];
			const newOne = newKeys[key];
			const oldRange = new Range();
			const newRange = new Range();

			// then replace the item in the *new* list with the *old* DOM elements 
			oldRange.setStartAfter(oldOne.open);
			oldRange.setEndBefore(oldOne.close);
			newRange.setStartAfter(newOne.open);
			newRange.setEndBefore(newOne.close);
			const oldContents = oldRange.extractContents();
			const newContents = newRange.extractContents();

			// patch the *old* DOM elements with the new ones
			diffRange(oldDocument, oldContents, newDocument, newContents, oldOne.close, newOne.close);

			// then insert the old DOM elements into the new tree 
			// this means you'll end up with any new attributes or content from the server, 
			// but with any old DOM state (because they are the old elements)
			newRange.insertNode(oldContents);
			newOne.open.replaceWith(oldOne.open);
			newOne.close.replaceWith(oldOne.close);
		}
	}

	try {
		oldRange.setStartAfter(oldNode);
		oldRange.setEndBefore(oldDocWalker.currentNode);
		newRange.setStartAfter(newNode);
		newRange.setEndAfter(newDocWalker.currentNode);
		const newContents = newRange.extractContents();
		oldRange.deleteContents();
		oldRange.insertNode(newContents);
		oldNode.replaceWith(newNode);
		oldDocWalker.currentNode.replaceWith(newDocWalker.currentNode);
	} catch (e) {
		console.error(e);
	}
}

function replaceBranch(oldDocWalker, newDocWalker, oldNode, newNode) {
	oldDocWalker.nextNode();
	newDocWalker.nextNode();
	const oldRange = new Range();
	const newRange = new Range();
	let oldBranches = 1;
	let newBranches = 1;
	while(oldBranches > 0) {
		if(oldDocWalker.nextNode()) {
			if(oldDocWalker.currentNode.nodeType === Node.COMMENT_NODE) {
				if(oldDocWalker.currentNode.textContent.startsWith("bo")) {
					oldBranches += 1;
				} else if(oldDocWalker.currentNode.textContent.startsWith("bc")) {

					oldBranches -= 1;
				}
			}
		}
	}
	while(newBranches > 0) {
		if(newDocWalker.nextNode()) {
			if(newDocWalker.currentNode.nodeType === Node.COMMENT_NODE) {
				if(newDocWalker.currentNode.textContent.startsWith("bo")) {
					newBranches += 1;
				} else if(newDocWalker.currentNode.textContent.startsWith("bc")) {

					newBranches -= 1;
				}
			}
		}
	}

	try {
		oldRange.setStartAfter(oldNode);
		oldRange.setEndBefore(oldDocWalker.currentNode);
		newRange.setStartAfter(newNode);
		newRange.setEndAfter(newDocWalker.currentNode);
		const newContents = newRange.extractContents();
		oldRange.deleteContents();
		oldRange.insertNode(newContents);
		oldNode.replaceWith(newNode);
		oldDocWalker.currentNode.replaceWith(newDocWalker.currentNode);
	} catch (e) {
		console.error(e);
	}
}

function diffElement(oldNode, newNode) {
	/** @type Element */
	const oldEl = oldNode;
	/** @type Element */
	const newEl = newNode;
	if (oldEl.tagName !== newEl.tagName) {
		oldEl.replaceWith(newEl);

	}
	else {
		for(const attr of newEl.attributes) {
			oldEl.setAttribute(attr.name, attr.value);
		}
	}
}

for(const island of document.querySelectorAll("leptos-island")) {
	island.$$hydrated = true;
}



================================================
FILE: leptos/src/hydration/mod.rs
================================================
#![allow(clippy::needless_lifetimes)]

use crate::{prelude::*, WasmSplitManifest};
use leptos_config::LeptosOptions;
use leptos_macro::{component, view};
use std::{path::PathBuf, sync::OnceLock};

/// Inserts auto-reloading code used in `cargo-leptos`.
///
/// This should be included in the `<head>` of your application shell during development.
#[component]
pub fn AutoReload(
    /// Whether the file-watching feature should be disabled.
    #[prop(optional)]
    disable_watch: bool,
    /// Configuration options for this project.
    options: LeptosOptions,
) -> impl IntoView {
    (!disable_watch && std::env::var("LEPTOS_WATCH").is_ok()).then(|| {
        #[cfg(feature = "nonce")]
        let nonce = crate::nonce::use_nonce();
        #[cfg(not(feature = "nonce"))]
        let nonce = None::<()>;

        let reload_port = match options.reload_external_port {
            Some(val) => val,
            None => options.reload_port,
        };
        let protocol = match options.reload_ws_protocol {
            leptos_config::ReloadWSProtocol::WS => "'ws://'",
            leptos_config::ReloadWSProtocol::WSS => "'wss://'",
        };

        let script = format!(
            "(function (reload_port, protocol) {{ {} {} }})({reload_port:?}, \
             {protocol})",
            leptos_hot_reload::HOT_RELOAD_JS,
            include_str!("reload_script.js")
        );
        view! { <script nonce=nonce>{script}</script> }
    })
}

/// Inserts hydration scripts that add interactivity to your server-rendered HTML.
///
/// This should be included in the `<head>` of your application shell.
#[component]
pub fn HydrationScripts(
    /// Configuration options for this project.
    options: LeptosOptions,
    /// Should be `true` to hydrate in `islands` mode.
    #[prop(optional)]
    islands: bool,
    /// Should be `true` to add the “islands router,” which enables limited client-side routing
    /// when running in islands mode.
    #[prop(optional)]
    islands_router: bool,
    /// A base url, not including a trailing slash
    #[prop(optional, into)]
    root: Option<String>,
) -> impl IntoView {
    static SPLIT_MANIFEST: OnceLock<Option<WasmSplitManifest>> =
        OnceLock::new();

    if let Some(splits) = SPLIT_MANIFEST.get_or_init(|| {
        let root = root.clone().unwrap_or_default();

        let site_dir = &options.site_root;
        let pkg_dir = &options.site_pkg_dir;
        let path = PathBuf::from(site_dir.to_string());
        let path = path
            .join(pkg_dir.to_string())
            .join("__wasm_split_manifest.json");
        let file = std::fs::read_to_string(path).ok()?;
        let manifest = WasmSplitManifest(ArcStoredValue::new((
            format!("{root}/{pkg_dir}"),
            serde_json::from_str(&file).expect("could not read manifest file"),
        )));

        Some(manifest)
    }) {
        provide_context(splits.clone());
    }

    let mut js_file_name = options.output_name.to_string();
    let mut wasm_file_name = options.output_name.to_string();
    if options.hash_files {
        let hash_path = std::env::current_exe()
            .map(|path| {
                path.parent().map(|p| p.to_path_buf()).unwrap_or_default()
            })
            .unwrap_or_default()
            .join(options.hash_file.as_ref());
        if hash_path.exists() {
            let hashes = std::fs::read_to_string(&hash_path)
                .expect("failed to read hash file");
            for line in hashes.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    if let Some((file, hash)) = line.split_once(':') {
                        if file == "js" {
                            js_file_name.push_str(&format!(".{}", hash.trim()));
                        } else if file == "wasm" {
                            wasm_file_name
                                .push_str(&format!(".{}", hash.trim()));
                        }
                    }
                }
            }
        } else {
            leptos::logging::error!(
                "File hashing is active but no hash file was found"
            );
        }
    } else if std::option_env!("LEPTOS_OUTPUT_NAME").is_none() {
        wasm_file_name.push_str("_bg");
    }

    let pkg_path = &options.site_pkg_dir;
    #[cfg(feature = "nonce")]
    let nonce = crate::nonce::use_nonce();
    #[cfg(not(feature = "nonce"))]
    let nonce = None::<String>;
    let script = if islands {
        if let Some(sc) = Owner::current_shared_context() {
            sc.set_is_hydrating(false);
        }
        include_str!("./island_script.js")
    } else {
        include_str!("./hydration_script.js")
    };

    let islands_router = islands_router
        .then_some(include_str!("./islands_routing.js"))
        .unwrap_or_default();

    let root = root.unwrap_or_default();
    view! {
        <link rel="modulepreload" href=format!("{root}/{pkg_path}/{js_file_name}.js") crossorigin=nonce.clone()/>
        <link
            rel="preload"
            href=format!("{root}/{pkg_path}/{wasm_file_name}.wasm")
            r#as="fetch"
            r#type="application/wasm"
            crossorigin=nonce.clone().unwrap_or_default()
        />
        <script type="module" nonce=nonce>
            {format!("{script}({root:?}, {pkg_path:?}, {js_file_name:?}, {wasm_file_name:?});{islands_router}")}
        </script>
    }
}

/// If this is provided via context, it means that you are using the islands router and
/// this is a subsequent navigation, made from the client.
///
/// This should be provided automatically by a server integration if it detects that the
/// header `Islands-Router` is present in the request.
///
/// This is used to determine how much of the hydration script to include in the page.
/// If it is present, then the contents of the `<HydrationScripts>` component will not be
/// included, as they only need to be sent to the client once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IslandsRouterNavigation;



================================================
FILE: leptos/src/hydration/reload_script.js
================================================
let host = window.location.hostname;
let ws = new WebSocket(`${protocol}${host}:${reload_port}/live_reload`);
ws.onmessage = (ev) => {
	let msg = JSON.parse(ev.data);
	if (msg.all) window.location.reload();
	if (msg.css) {
		let found = false;
		document.querySelectorAll("link").forEach((link) => {
			if (link.getAttribute('href').includes(msg.css)) {
				let newHref = '/' + msg.css + '?version=' + Date.now();
				link.setAttribute('href', newHref);
				found = true;
			}
		});
		if (!found) console.warn(`CSS hot-reload: Could not find a <link href=/\"${msg.css}\"> element`);
	};
	if(msg.view) {
		patch(msg.view);
	}
};
ws.onclose = () => console.warn('Live-reload stopped. Manual reload necessary.');

