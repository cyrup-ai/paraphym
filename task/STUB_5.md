# STUB_5: Resource Tracking Implementation

**Priority:** 🟡 MEDIUM  
**Severity:** Incomplete Feature  
**Estimated Effort:** 1 session

## OBJECTIVE

Replace incomplete resource tracking in Landlock backend with production-quality implementation that accurately measures CPU time, process count, disk I/O, and memory usage using Linux /proc filesystem.

## BACKGROUND

Current implementation has comment "Would need to implement actual resource tracking" with fabricated data:
- `cpu_time_ms`: Uses wall clock time (incorrect - should use actual CPU time)
- `process_count`: Hardcoded to 1 (ignores child processes)
- `disk_bytes_written`: Hardcoded to 0 (not tracked)

## LOCATION

**File:** `packages/cylo/src/backends/landlock.rs`  
**Line:** 443-446

## SUBTASK 1: Implement CPU Time Tracking

**What:** Read actual CPU time from `/proc/[pid]/stat`  
**Where:** New `get_process_cpu_time()` function

**Why:** Current implementation uses wall clock Duration, not actual CPU usage

**Implementation:**
```rust
use std::fs;

fn get_process_cpu_time(pid: libc::pid_t) -> Result<u64, CyloError> {
    // Read from /proc/[pid]/stat
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(&stat_path)
        .map_err(|e| CyloError::ResourceTracking(
            format!("Failed to read {}: {}", stat_path, e)
        ))?;
    
    // Parse stat file (space-separated fields)
    let fields: Vec<&str> = stat_content.split_whitespace().collect();
    if fields.len() < 15 {
        return Err(CyloError::ResourceTracking("Invalid stat format".to_string()));
    }
    
    // Field 13 = utime (user mode jiffies)
    // Field 14 = stime (kernel mode jiffies)
    let utime: u64 = fields[13].parse()
        .map_err(|e| CyloError::ResourceTracking(format!("Invalid utime: {}", e)))?;
    let stime: u64 = fields[14].parse()
        .map_err(|e| CyloError::ResourceTracking(format!("Invalid stime: {}", e)))?;
    
    // Convert clock ticks to milliseconds
    let clock_ticks_per_sec = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as u64;
    let total_ticks = utime + stime;
    let cpu_time_ms = (total_ticks * 1000) / clock_ticks_per_sec;
    
    Ok(cpu_time_ms)
}
```

## SUBTASK 2: Implement Process Tree Counting

**What:** Count process and all its children recursively  
**Where:** New `count_process_tree()` function

**Why:** Current hardcoded "1" ignores forked children and threads

**Implementation:**
```rust
fn count_process_tree(pid: libc::pid_t) -> Result<usize, CyloError> {
    let mut count = 1; // Count the process itself
    
    // Count threads via /proc/[pid]/task directory
    let task_dir = format!("/proc/{}/task", pid);
    if let Ok(entries) = fs::read_dir(&task_dir) {
        // Each entry is a thread; main thread is already counted
        let thread_count = entries.count();
        if thread_count > 0 {
            count += thread_count - 1; // Don't double-count main thread
        }
    }
    
    // Find child processes from /proc/[pid]/task/[pid]/children
    let children_path = format!("/proc/{}/task/{}/children", pid, pid);
    if let Ok(children_content) = fs::read_to_string(&children_path) {
        for child_pid_str in children_content.split_whitespace() {
            if let Ok(child_pid) = child_pid_str.parse::<libc::pid_t>() {
                // Recursively count child's subtree
                count += count_process_tree(child_pid)?;
            }
        }
    }
    
    Ok(count)
}
```

## SUBTASK 3: Implement Disk I/O Tracking

**What:** Read disk write statistics from `/proc/[pid]/io`  
**Where:** New `get_disk_io_stats()` function

**Why:** Current hardcoded 0 provides no useful information

**Implementation:**
```rust
fn get_disk_io_stats(pid: libc::pid_t) -> Result<u64, CyloError> {
    // Read from /proc/[pid]/io
    let io_path = format!("/proc/{}/io", pid);
    let io_content = fs::read_to_string(&io_path)
        .map_err(|e| CyloError::ResourceTracking(
            format!("Failed to read {}: {}", io_path, e)
        ))?;
    
    // Parse for write_bytes line
    let mut bytes_written = 0u64;
    for line in io_content.lines() {
        if line.starts_with("write_bytes:") {
            bytes_written = line.split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| CyloError::ResourceTracking(
                    "Invalid write_bytes format".to_string()
                ))?;
            break;
        }
    }
    
    Ok(bytes_written)
}

fn get_disk_read_stats(pid: libc::pid_t) -> Result<u64, CyloError> {
    let io_path = format!("/proc/{}/io", pid);
    let io_content = fs::read_to_string(&io_path)
        .map_err(|e| CyloError::ResourceTracking(
            format!("Failed to read {}: {}", io_path, e)
        ))?;
    
    let mut bytes_read = 0u64;
    for line in io_content.lines() {
        if line.starts_with("read_bytes:") {
            bytes_read = line.split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| CyloError::ResourceTracking(
                    "Invalid read_bytes format".to_string()
                ))?;
            break;
        }
    }
    
    Ok(bytes_read)
}
```

## SUBTASK 4: Implement Memory Usage Tracking

**What:** Read RSS memory from `/proc/[pid]/status`  
**Where:** New `get_memory_usage()` function

**Why:** Need to track actual memory consumption

**Implementation:**
```rust
fn get_memory_usage(pid: libc::pid_t) -> Result<u64, CyloError> {
    // Read from /proc/[pid]/status
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(&status_path)
        .map_err(|e| CyloError::ResourceTracking(
            format!("Failed to read {}: {}", status_path, e)
        ))?;
    
    // Parse for VmRSS (Resident Set Size)
    let mut rss_kb = 0u64;
    for line in status_content.lines() {
        if line.starts_with("VmRSS:") {
            rss_kb = line.split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| CyloError::ResourceTracking(
                    "Invalid VmRSS format".to_string()
                ))?;
            break;
        }
    }
    
    // Convert kilobytes to bytes
    Ok(rss_kb * 1024)
}
```

## SUBTASK 5: Add CyloError Variant

**What:** Add ResourceTracking error variant if not exists  
**Where:** `CyloError` enum definition

**Why:** Need proper error type for resource tracking failures

**Implementation:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum CyloError {
    // ... existing variants ...
    
    #[error("Resource tracking error: {0}")]
    ResourceTracking(String),
}
```

## SUBTASK 6: Integrate into get_resource_usage

**What:** Replace incomplete implementation with production code  
**Where:** Line 443-446 in landlock.rs

**Current:**
```rust
let resource_usage = ResourceUsage {
    // Would need to implement actual resource tracking
    cpu_time_ms: duration.as_millis() as u64,
    process_count: 1,
    disk_bytes_written: 0,
```

**Replace with:**
```rust
let resource_usage = get_resource_usage(pid)?;

// New helper function:
pub fn get_resource_usage(pid: libc::pid_t) -> Result<ResourceUsage, CyloError> {
    Ok(ResourceUsage {
        cpu_time_ms: get_process_cpu_time(pid)?,
        process_count: count_process_tree(pid)?,
        disk_bytes_written: get_disk_io_stats(pid)?,
        disk_bytes_read: get_disk_read_stats(pid)?,
        memory_bytes: get_memory_usage(pid)?,
        network_bytes_sent: 0, // Leave for future implementation
        network_bytes_received: 0, // Leave for future implementation
    })
}
```

## SUBTASK 7: Update ResourceUsage Structure

**What:** Add missing fields to ResourceUsage if needed  
**Where:** ResourceUsage struct definition

**Additions:**
```rust
pub struct ResourceUsage {
    pub cpu_time_ms: u64,
    pub process_count: usize,
    pub disk_bytes_written: u64,
    pub disk_bytes_read: u64,      // Add if missing
    pub memory_bytes: u64,          // Add if missing
    pub network_bytes_sent: u64,    // Add if missing (TODO)
    pub network_bytes_received: u64, // Add if missing (TODO)
}
```

## SUBTASK 8: Remove TODO Comment

**What:** Remove "Would need to implement" comment  
**Where:** Line 442 comment

**Why:** Implementation is now complete

## DEFINITION OF DONE

- [ ] `get_process_cpu_time()` implemented using /proc/[pid]/stat
- [ ] `count_process_tree()` implemented with recursion
- [ ] `get_disk_io_stats()` and `get_disk_read_stats()` implemented
- [ ] `get_memory_usage()` implemented using VmRSS
- [ ] `CyloError::ResourceTracking` variant added
- [ ] `get_resource_usage()` helper function created
- [ ] Integration complete in landlock.rs
- [ ] ResourceUsage struct has all fields
- [ ] TODO/incomplete comments removed
- [ ] Error handling comprehensive
- [ ] Code compiles without warnings
- [ ] Network fields documented as future work

## REQUIREMENTS

- ❌ **NO TESTS** - Testing team handles test coverage
- ❌ **NO BENCHMARKS** - Performance team handles benchmarking
- ✅ **PRODUCTION CODE ONLY** - Complete implementation, no stubs
- ⚠️ **LINUX ONLY** - This implementation is Linux-specific via /proc filesystem

## RESEARCH NOTES

### /proc Filesystem Documentation

- `/proc/[pid]/stat`: Process statistics (CPU time, state, etc.)
  - Field 13 (utime): CPU time in user mode (clock ticks)
  - Field 14 (stime): CPU time in kernel mode (clock ticks)
  - Clock ticks converted using `sysconf(_SC_CLK_TCK)` (typically 100 Hz)

- `/proc/[pid]/io`: I/O statistics (requires CONFIG_TASK_IO_ACCOUNTING)
  - `read_bytes`: Total bytes read (includes cache)
  - `write_bytes`: Total bytes written (includes cache)
  - May require specific permissions in some configurations

- `/proc/[pid]/status`: Process status (human-readable)
  - `VmRSS`: Resident Set Size in kilobytes
  - `VmSize`: Virtual memory size
  - `Threads`: Thread count

- `/proc/[pid]/task/[pid]/children`: Space-separated list of child PIDs

### Clock Tick Conversion

```rust
// Get system clock tick rate
let hz = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
// Typically 100 on most systems (USER_HZ)

// Convert ticks to milliseconds
let ms = (ticks * 1000) / hz;
```

### Error Handling

- Processes may disappear between stat reads (not an error, return 0)
- /proc/[pid]/io may not exist on older kernels (fallback to 0)
- Permission denied errors should be propagated
- Parse errors indicate malformed /proc data (should be rare)

### Alternative: getrusage()

Could use `getrusage()` syscall instead of /proc:
```rust
use libc::{getrusage, rusage, RUSAGE_CHILDREN};

let mut usage = std::mem::MaybeUninit::<rusage>::uninit();
unsafe {
    getrusage(RUSAGE_CHILDREN, usage.as_mut_ptr());
    let usage = usage.assume_init();
    // usage.ru_utime, usage.ru_stime available
}
```
However, /proc provides more comprehensive data (I/O, memory).

### Process Tree Traversal

Recursive traversal can be expensive for large process trees.
Consider caching or limiting depth if performance becomes an issue.

### Network Tracking (Future Work)

Network bytes would require:
- Reading `/proc/net/dev` (per-interface, not per-process)
- Using eBPF/BPF for per-process network accounting
- Third-party crates like `procfs` with network extensions

Leaving as 0 for now is acceptable; document as future enhancement.

## VERIFICATION

After implementation, verify:
1. CPU time increases when process does work (not wall clock time)
2. Process count correctly includes threads and children
3. Disk I/O tracked for actual file writes
4. Memory usage reflects RSS
5. Short-lived child processes counted
6. Error handling graceful when /proc files unavailable
7. No panics when processes disappear
