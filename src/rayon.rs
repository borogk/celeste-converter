use rayon::ThreadPoolBuilder;

pub fn init_rayon() {
    ThreadPoolBuilder::new()
        .thread_name(|i| format!("thread-{}", i))
        .build_global()
        .unwrap();
}
