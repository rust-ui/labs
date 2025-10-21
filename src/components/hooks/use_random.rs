use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn use_random_id() -> String {
    let mut hasher = DefaultHasher::new();

    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    counter.hash(&mut hasher);

    format!("_gen_id_{}", hasher.finish())
}

pub fn use_random_transition_name() -> String {
    let random_id = use_random_id();
    format!("view-transition-name: {random_id}")
}
