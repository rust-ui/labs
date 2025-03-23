pub fn generate_random_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Get a somewhat random number based on the memory address of a new object
    let ptr_value = Box::new(());
    let addr = &*ptr_value as *const () as usize;

    // Add some extra randomness with the current call counter
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);

    // Hash the combined values
    let mut hasher = DefaultHasher::new();
    addr.hash(&mut hasher);
    count.hash(&mut hasher);

    format!("_gen_id_{:x}", hasher.finish())
}
