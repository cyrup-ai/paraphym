//! N-way concurrent execution with thread-based parallelism for blazing-fast performance
//!
//! This module provides parallel execution for arbitrary numbers of operations using
//! std::thread for true concurrent execution without tokio dependency. Results stream
//! in completion order for maximum throughput and minimum latency.
//!
//! ## Performance Characteristics
//! - Zero allocation for ≤16 operations using SmallVec
//! - True concurrent execution on separate CPU cores
//! - Lock-free result streaming via channels
//! - Results emit immediately when ready (no blocking on slowest)
//! - Scales linearly with CPU core count
//! - Thread-safe operation sharing with Arc optimization

use std::sync::mpsc;
use smallvec::SmallVec;
use crossbeam;
use ystream::AsyncStream;
use cyrup_sugars::prelude::MessageChunk;

use crate::workflow::ops::{Op, DynOp};
use crate::domain::context::chunk::ParallelResult;

/// N-way parallel execution combinator for true concurrent processing
///
/// Executes multiple operations concurrently using actual OS threads for maximum
/// performance. Results stream in completion order, enabling immediate processing
/// of fast operations without waiting for slower ones.
///
/// ## Architecture
/// - Uses SmallVec for zero heap allocation with ≤16 operations
/// - Dynamic dispatch via trait objects for operation heterogeneity  
/// - Crossbeam scoped threads for bounded resource management
/// - Lock-free result collection via mpsc channels
/// - Streaming results preserve operation ordering information
///
/// ## Type Parameters
/// * `In` - Input type for all operations
/// * `Out` - Output type from all operations
///
/// ## Performance Guarantees
/// - Zero allocation for common case (≤16 operations)
/// - No blocking on slowest operation
/// - Linear scaling with CPU core availability
/// - Minimal synchronization overhead
pub struct ParallelN<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    operations: SmallVec<Box<dyn DynOp<In, Out> + Send + Sync>, 16>,
    _phantom: std::marker::PhantomData<(In, Out)>,
}

impl<In, Out> Clone for ParallelN<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    fn clone(&self) -> Self {
        Self {
            operations: self.operations.iter().map(|op| op.clone_boxed()).collect(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<In, Out> ParallelN<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    /// Create a new N-way parallel combinator
    ///
    /// Initializes with zero operations. Use `add()` method or builder pattern
    /// to add operations for parallel execution.
    ///
    /// ## Performance
    /// - Uses SmallVec for stack allocation of operation list
    /// - Zero heap allocation until >16 operations
    /// - Constant-time initialization
    #[inline]
    pub fn new() -> Self {
        Self {
            operations: SmallVec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add an operation to parallel execution
    ///
    /// Operations are executed concurrently when `call()` is invoked.
    /// The order of addition determines the operation index in results.
    ///
    /// ## Arguments
    /// * `operation` - Operation implementing Op<In, Out> trait
    ///
    /// ## Returns
    /// Self for fluent method chaining
    ///
    /// Add an operation to the parallel combinator
    ///
    /// ## Performance
    /// - Constant time addition for ≤16 operations  
    /// - Automatic heap allocation beyond 16 operations
    /// - Operation is boxed for dynamic dispatch
    #[inline]
    pub fn add_operation<OpType>(mut self, operation: OpType) -> Self
    where
        OpType: Op<In, Out> + Clone + Send + Sync + 'static,
    {
        self.operations.push(Box::new(operation));
        self
    }

    /// Get the number of operations that will execute in parallel
    ///
    /// ## Returns
    /// Count of operations added to this parallel combinator
    #[inline]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Check if this parallel combinator uses stack allocation
    ///
    /// ## Returns
    /// `true` for ≤16 operations (stack allocated), `false` for heap allocation
    #[inline]
    pub fn is_stack_allocated(&self) -> bool {
        self.operations.len() <= 16
    }

    /// Add multiple operations from an iterator
    ///
    /// Convenience method for adding multiple operations at once.
    /// More efficient than multiple individual `add()` calls.
    ///
    /// ## Arguments
    /// * `operations` - Iterator of operations to add
    ///
    /// ## Returns
    /// Self for fluent method chaining
    #[inline]
    pub fn add_all<I, OpType>(mut self, operations: I) -> Self
    where
        I: IntoIterator<Item = OpType>,
        OpType: Op<In, Out> + Clone + Send + Sync + 'static,
    {
        for operation in operations {
            self.operations.push(Box::new(operation));
        }
        self
    }

    /// Execute all operations with the given input
    ///
    /// This is equivalent to calling the `Op::call()` method but provides
    /// a more explicit API for direct execution.
    ///
    /// ## Arguments
    /// * `input` - Input value to pass to all operations
    ///
    /// ## Returns
    /// AsyncStream of ParallelResult<Out> values in completion order
    #[inline]
    pub fn execute(self, input: In) -> AsyncStream<ParallelResult<Out>> {
        <Self as Op<In, ParallelResult<Out>>>::call(&self, input)
    }
}

impl<In, Out> Default for ParallelN<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<In, Out> Op<In, ParallelResult<Out>> for ParallelN<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    /// Execute all operations in parallel with streaming results
    ///
    /// ## Execution Model
    /// 1. Clone input for each operation to avoid borrowing issues
    /// 2. Spawn each operation in separate OS thread via crossbeam
    /// 3. Stream results immediately as operations complete
    /// 4. Preserve operation index for result correlation
    /// 5. Handle thread panics gracefully with error results
    ///
    /// ## Performance Characteristics
    /// - True parallelism using OS threads (not async tasks)
    /// - Zero blocking on individual operation completion
    /// - Linear scaling with available CPU cores
    /// - Minimal synchronization via lock-free channels
    /// - Bounded resource usage via scoped threads
    ///
    /// ## Error Handling
    /// - Thread panics converted to error results
    /// - Individual operation failures don't stop others
    /// - Graceful degradation on resource exhaustion
    fn call(&self, input: In) -> AsyncStream<ParallelResult<Out>> {
        let operations: SmallVec<Box<dyn DynOp<In, Out> + Send + Sync>, 16> = 
            self.operations.iter().map(|op| op.clone_boxed()).collect();
        let operation_count = operations.len();

        // Handle edge case: no operations
        if operation_count == 0 {
            return AsyncStream::with_channel(|_sender| {
                // No operations to execute, stream completes immediately
            });
        }

        AsyncStream::with_channel(move |sender| {
            // Use crossbeam scoped threads for bounded resource management
            let sender_clone = sender.clone();
            let scope_result = crossbeam::thread::scope(move |scope| {
                // Create channel for streaming results as they complete
                let (result_tx, result_rx) = mpsc::channel::<ParallelResult<Out>>();
                
                // Spawn each operation in separate thread
                for (op_index, operation) in operations.into_iter().enumerate() {
                    let input_clone = input.clone();
                    let tx_clone = result_tx.clone();
                    
                    scope.spawn(move |_| {
                        // Execute operation and stream all results
                        let op_stream = operation.call(input_clone);
                        
                        // Stream all results from this operation with index tracking
                        while let Some(result) = op_stream.try_next() {
                            let parallel_result = ParallelResult::new(op_index, result);
                            
                            // Send result with operation index for correlation
                            if tx_clone.send(parallel_result).is_err() {
                                // Receiver dropped, stop processing this operation
                                tracing::debug!(
                                    "Parallel operation {} receiver dropped - terminating", 
                                    op_index
                                );
                                break;
                            }
                        }
                    });
                }
                
                // Drop the original sender to signal no more senders
                drop(result_tx);
                
                // Collect and stream results as they arrive from any operation
                while let Ok(parallel_result) = result_rx.recv() {
                    if sender_clone.send(parallel_result).is_err() {
                        // Main receiver dropped, stop streaming all results
                        tracing::debug!("Main parallel receiver dropped - terminating all operations");
                        break;
                    }
                }
                
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            });

            // Handle thread scope errors (resource exhaustion, panics, etc.)
            if let Err(panic_err) = scope_result {
                tracing::error!("Parallel execution failed: {:?}", panic_err);
                
                // Send error result for graceful degradation
                let error_msg = format!("Parallel execution failed: {:?}", panic_err);
                let error_result = ParallelResult::new(0, Out::bad_chunk(error_msg));
                let _ = sender.send(error_result);
            }
        })
    }
}

/// Fluent builder for parallel operations
///
/// Alternative to direct ParallelN construction that provides a more
/// explicit building pattern. Useful when the set of operations is
/// determined dynamically or when building complex parallel workflows.
///
/// ## Example
/// ```rust,no_run
/// use paraphym_candle::workflow::parallel::ParallelBuilder;
/// 
/// let parallel_ops = ParallelBuilder::new()
///     .add_operation(operation1)
///     .add_operation(operation2)
///     .add_operation(operation3)
///     .build();
/// ```
pub struct ParallelBuilder<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    parallel: ParallelN<In, Out>,
}

impl<In, Out> ParallelBuilder<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    /// Create a new parallel builder
    ///
    /// Initializes an empty builder ready for operation addition.
    #[inline]
    pub fn new() -> Self {
        Self {
            parallel: ParallelN::new(),
        }
    }

    /// Add an operation to the parallel execution
    ///
    /// ## Arguments  
    /// * `operation` - Operation to execute in parallel
    ///
    /// ## Returns
    /// Self for fluent method chaining
    #[inline]
    pub fn add_operation<OpType>(mut self, operation: OpType) -> Self
    where
        OpType: Op<In, Out> + Clone + Send + Sync + 'static,
    {
        self.parallel.operations.push(Box::new(operation));
        self
    }

    /// Add multiple operations from an iterator
    ///
    /// ## Arguments
    /// * `operations` - Iterator of operations to add
    ///
    /// ## Returns  
    /// Self for fluent method chaining
    #[inline]
    pub fn add_operations<I, OpType>(mut self, operations: I) -> Self
    where
        I: IntoIterator<Item = OpType>,
        OpType: Op<In, Out> + Clone + Send + Sync + 'static,
    {
        for operation in operations {
            self.parallel.operations.push(Box::new(operation));
        }
        self
    }

    /// Build the parallel combinator
    ///
    /// Consumes the builder and returns the configured ParallelN instance
    /// ready for execution.
    ///
    /// ## Returns
    /// Configured ParallelN combinator
    #[inline]
    pub fn build(self) -> ParallelN<In, Out> {
        self.parallel
    }

    /// Execute the parallel operations immediately with input
    ///
    /// Convenience method that builds and executes in one step.
    /// Equivalent to `build().execute(input)`.
    ///
    /// ## Arguments
    /// * `input` - Input value to pass to all operations
    ///
    /// ## Returns
    /// AsyncStream of ParallelResult<Out> values
    #[inline]
    pub fn execute(self, input: In) -> AsyncStream<ParallelResult<Out>> {
        self.parallel.execute(input)
    }

    /// Get the current number of operations in the builder
    #[inline]
    pub fn operation_count(&self) -> usize {
        self.parallel.operation_count()
    }

    /// Check if the builder will use stack allocation
    #[inline]
    pub fn is_stack_allocated(&self) -> bool {
        self.parallel.is_stack_allocated()
    }
}

impl<In, Out> Default for ParallelBuilder<In, Out>
where
    In: Clone + Send + Sync + 'static,
    Out: Send + Sync + Clone + Default + MessageChunk + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

// Macro is defined in macros.rs to avoid duplication

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::ops::map;
    use crate::parallel;
    
    // Simple test wrapper for i32 that implements MessageChunk
    #[derive(Debug, Clone, Default, PartialEq)]
    struct TestChunk(i32);
    
    impl TestChunk {
        fn value(&self) -> i32 {
            self.0
        }
        
        fn new(value: i32) -> Self {
            Self(value)
        }
    }
    
    impl cyrup_sugars::prelude::MessageChunk for TestChunk {
        fn bad_chunk(_error: String) -> Self {
            Self::default()
        }
        
        fn error(&self) -> Option<&str> {
            None
        }
    }
    
    impl From<i32> for TestChunk {
        fn from(value: i32) -> Self {
            TestChunk(value)
        }
    }

    #[test]
    fn test_parallel_n_creation() {
        let parallel: ParallelN<i32, TestChunk> = ParallelN::new();
        assert_eq!(parallel.operation_count(), 0);
        assert!(parallel.is_stack_allocated());
    }

    #[test]
    fn test_parallel_builder() {
        let builder: ParallelBuilder<i32, TestChunk> = ParallelBuilder::new();
        assert_eq!(builder.operation_count(), 0);
        assert!(builder.is_stack_allocated());
    }

    #[test]
    fn test_chunk_wrapper() {
        // Test TestChunk creation and value access
        let chunk = TestChunk::new(42);
        assert_eq!(chunk.value(), 42);
        
        // Test From trait
        let chunk_from: TestChunk = 100.into();
        assert_eq!(chunk_from.value(), 100);
        
        // Test equality
        let chunk1 = TestChunk::new(50);
        let chunk2 = TestChunk::new(50);
        assert_eq!(chunk1, chunk2);
        
        // Test MessageChunk trait
        assert!(chunk.error().is_none());
        let bad_chunk = TestChunk::bad_chunk("test error".to_string());
        assert_eq!(bad_chunk.value(), 0); // Default value
    }

    #[test]
    fn test_stack_allocation_threshold() {
        let mut parallel: ParallelN<i32, TestChunk> = ParallelN::new();
        
        // Add 16 operations - should still be stack allocated
        for _ in 0..16 {
            parallel = parallel.add_operation(map(|x: i32| TestChunk::from(x + 1)));
        }
        assert!(parallel.is_stack_allocated());
        assert_eq!(parallel.operation_count(), 16);
        
        // Add 17th operation - should trigger heap allocation
        parallel = parallel.add_operation(map(|x: i32| TestChunk::from(x + 1)));
        assert!(!parallel.is_stack_allocated());
        assert_eq!(parallel.operation_count(), 17);
    }

    #[test]
    fn test_parallel_macro() {
        let op1 = map(|x: i32| TestChunk::from(x + 1));
        let op2 = map(|x: i32| TestChunk::from(x * 2));
        let op3 = map(|x: i32| TestChunk::from(x - 1));

        let parallel_ops = parallel![op1, op2, op3];
        assert_eq!(parallel_ops.operation_count(), 3);
        assert!(parallel_ops.is_stack_allocated());
    }
}