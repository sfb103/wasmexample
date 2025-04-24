#[cfg(feature = "executor")]
pub mod executor {
    use std::future::Future;
    use std::error::Error;
    use std::time::Duration;

    /// A trait representing the ability to spawn asynchronous tasks.
    pub trait Executor: Sync + Send + Copy + 'static {
        /// The handle returned by `spawn`, allowing awaiting the task's completion.
        /// Uses GATs (Generic Associated Types) to be generic over the task's output type `T`.
        type JoinHandle<T: Send + 'static>: Future<Output = Result<T, Self::JoinError>> + Send + 'static;

        /// The error type returned when joining a task fails (e.g., due to panic).
        /// Must implement standard error traits and be Send + Sync + 'static.
        type JoinError: Error + Send + Sync + 'static;

        /// Associated type for the future returned by `sleep`.
        /// Must resolve to () and be Send + 'static.
        type SleepFuture: Future<Output = ()> + Send + 'static;        

        /// Spawns a new asynchronous task to run on this executor.
        ///
        /// - `future`: The task to run async, expressed as a Future. It must be `Send`
        ///             to potentially run on other threads and `'static` because the
        ///             spawned task may outlive the caller.
        /// - `T`: The output type of the future. It must also be `Send + 'static`.
        ///
        /// Returns a `JoinHandle` which can be awaited to get the result of the future.
        fn spawn<F, T>(&self, future: F) -> Self::JoinHandle<T>
        where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static;

        /// Like spawn, but executes the closure on a thread where blocking is acceptable.
        /// 
        /// - `task`: The task to run sync, expressed as an FnOnce (e.g. closure). It
        ///           must be `Send` to potentially run on other threads and `'static`
        ///           because the spawned task may outlive the caller.
        /// - `T`: The return value of the FnOnce (e.g. closure). It must also be
        ///        `Send + 'static`.
        ///
        fn spawn_blocking<F, T>(&self, task: F) -> Self::JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static;

        /// Pauses the current asynchronous task for the specified duration. This can
        /// be used within tasks created either by spawn or spawn_blocking.
        /// 
        /// - `duration`: the amount of time to pause the task
        /// 
        /// Returns a future that pauses the current asynchronous task for the specified
        /// duration when awaited.
        fn sleep(&self, duration: Duration) -> Self::SleepFuture;
    }
}

#[cfg(feature = "tokio_executor")]
pub mod tokio_executor {
    use crate::executor::Executor;

    /// An implementation of the `Executor` trait using the Tokio runtime.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct TokioExecutor;

    impl Executor for TokioExecutor {
        // Tokio's JoinHandle fits the requirements for our associated type.
        type JoinHandle<T: Send + 'static> = tokio::task::JoinHandle<T>;

        // Tokio's JoinError fits the requirements.
        type JoinError = tokio::task::JoinError;

        // Tokio's Sleep fits the requirements.
        type SleepFuture = tokio::time::Sleep;     

        fn spawn<F, T>(&self, future: F) -> Self::JoinHandle<T>
        where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static, {
            // Delegate spawning directly to tokio::spawn.
            tokio::spawn(future)
        }

        fn spawn_blocking<F, T>(&self, task: F) -> Self::JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static, {
            // Delegate spawning directly to tokio::spawn_block.
            tokio::task::spawn_blocking(task)
        }

        fn sleep(&self, duration: std::time::Duration) -> Self::SleepFuture {
            tokio::time::sleep(duration)
        }        
    }
}

#[cfg(test)]
mod tests {
    use crate::executor::Executor;
    use crate::tokio_executor::TokioExecutor;

    use std::time::Duration;

    async fn run_task_on_executor<E: Executor>(executor: E) {
        // Since TokioExecutor is Copy, capturing `executor` by value in async move works.
        // If E was only Clone, use: let exec_clone = executor.clone(); before spawn.        
        let handle = executor.spawn(async move {
            println!("Hello World!");
            let duration = Duration::from_millis(30 as u64);
            executor.sleep(duration).await; // Call sleep on the captured executor instance
        });
        handle.await.unwrap();
        println!("Future is joined.")
    }

    #[tokio::test]
    async fn test_executor() {
        println!("Using TokioExecutor");
        let tokio_executor = TokioExecutor;
        run_task_on_executor(tokio_executor).await;
        println!("Done.")
    }
}
