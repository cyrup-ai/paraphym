# Paraphym Candle

High-performance AI framework with fluent API for building agents using Candle ML.

## Architecture

### Async Streaming (100% Tokio)

This codebase uses **pure tokio async streaming** throughout:

- ✅ **tokio_stream** for all stream operations (89+ usages)
- ✅ **async_stream::spawn_stream** for stream construction (187+ usages)
- ✅ **tokio::sync::mpsc** for async channels
- ✅ **tokio::spawn** for async task spawning

### Migration Notes

**Completed Migration (v0.1.0):**
- Removed all `ystream` dependencies
- Removed all `crossbeam-channel`, `crossbeam-queue`, `crossbeam-utils` dependencies
- Kept only `crossbeam-skiplist` for lock-free data structures (different purpose)
- Converted all sync/async bridges to pure tokio async patterns

**Benefits:**
- **Better Performance**: No sync/async bridging overhead
- **Simpler Code**: Single async runtime (tokio) throughout
- **Better Error Handling**: Proper async error propagation
- **Better Cancellation**: Native tokio task cancellation support

### Dependencies

**Async Runtime:**
- `tokio` - Main async runtime with full features
- `tokio-stream` - Stream utilities
- `async-stream` - Stream construction helpers

**Lock-free Data Structures:**
- `crossbeam-skiplist` - For O(log n) concurrent map operations (SkipMap)

### Performance

The migration to pure tokio async provides:
- Zero-allocation channel operations
- Better memory usage (no thread spawning overhead)
- Native async backpressure handling
- Efficient task scheduling with tokio runtime

## Usage

See examples in `examples/` directory for usage patterns.

## License

MIT
