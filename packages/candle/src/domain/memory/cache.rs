use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Cached timestamp for zero-allocation timestamp operations
static CACHED_TIMESTAMP: AtomicU64 = AtomicU64::new(0);

/// Get cached timestamp with zero allocation - updates every ~100ms
#[inline(always)]
#[must_use]
pub fn get_cached_timestamp() -> u64 {
    let cached = CACHED_TIMESTAMP.load(Ordering::Relaxed);
    if cached == 0 {
        // First call or background thread hasn't updated yet
        update_cached_timestamp();
        CACHED_TIMESTAMP.load(Ordering::Relaxed)
    } else {
        cached
    }
}

/// Update cached timestamp (called by background thread or on first access)
#[inline]
fn update_cached_timestamp() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    CACHED_TIMESTAMP.store(now, Ordering::Relaxed);
}

/// Get cached SystemTime for compatibility with existing APIs
#[inline(always)]
#[must_use]
pub fn get_cached_system_time() -> SystemTime {
    let timestamp = get_cached_timestamp();
    UNIX_EPOCH + std::time::Duration::from_secs(timestamp)
}

/// Initialize timestamp caching system (call once at startup)
pub fn initialize_timestamp_cache() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        // Initial timestamp update
        update_cached_timestamp();

        // Start background thread for periodic updates
        std::thread::spawn(|| loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            update_cached_timestamp();
        });
    });
}
