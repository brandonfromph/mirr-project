use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        System.realloc(ptr, old_layout, new_size)
    }
}

#[global_allocator]
static A: CountingAllocator = CountingAllocator;

#[test]
fn allocation_audit_hot_path_no_new_allocs_after_init() {
    use nasa_rust_project::mirr_executor::{drive_lexer_with_interpreter, set_alloc_hook};

    // install hook so that the interpreter will notify us of checkpoints
    set_alloc_hook(|label| {
        let c = ALLOC_COUNT.load(Ordering::SeqCst);
        eprintln!("[alloc-test] {} -> {}", label, c);
    });

    // Warm-up runs (allows init-time allocations to occur).  Execute twice
    // with "init" and once with the eventual hot-path input to exercise any
    // branches that would otherwise allocate later.
    let baseline_pre = ALLOC_COUNT.load(Ordering::SeqCst);
    let alloc1 = {
        let before = ALLOC_COUNT.load(Ordering::SeqCst);
        let _ = drive_lexer_with_interpreter(b"init");
        ALLOC_COUNT.load(Ordering::SeqCst).saturating_sub(before)
    };
    let alloc2 = {
        let before = ALLOC_COUNT.load(Ordering::SeqCst);
        let _ = drive_lexer_with_interpreter(b"init");
        ALLOC_COUNT.load(Ordering::SeqCst).saturating_sub(before)
    };
    let alloc3 = {
        let before = ALLOC_COUNT.load(Ordering::SeqCst);
        let _ = drive_lexer_with_interpreter(b"guard true");
        ALLOC_COUNT.load(Ordering::SeqCst).saturating_sub(before)
    };
    let baseline_post = ALLOC_COUNT.load(Ordering::SeqCst);
    eprintln!(
        "warmup allocs: init1={} init2={} guard={} total {}->{}",
        alloc1, alloc2, alloc3, baseline_pre, baseline_post
    );

    // Switch to a no-op hook before the measurement window so that the
    // eprintln! inside the diagnostic hook does not contribute allocations
    // to the hot-path measurement (on Windows, eprintln! can allocate a
    // UTF-16 transcoding buffer via the I/O subsystem).
    set_alloc_hook(|_| {});

    // Representative hot-path invocation — should not allocate after init.
    let pre = ALLOC_COUNT.load(Ordering::SeqCst);
    let _out = drive_lexer_with_interpreter(b"guard true");
    let post = ALLOC_COUNT.load(Ordering::SeqCst);
    let hot_alloc = post.saturating_sub(pre);
    eprintln!("hot-path allocs: {} ({} -> {})", hot_alloc, pre, post);
    assert_eq!(
        post, pre,
        "Hot-path emitted unexpected heap allocations after init ({} -> {})",
        pre, post
    );

    // Sanity: ensure warm-up did allocate at least once.
    assert!(baseline_post >= baseline_pre, "Warm-up did not record any allocations");
}
