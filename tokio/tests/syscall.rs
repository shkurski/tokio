#[cfg(all(feature = "test-util", tokio_unstable))]
mod unstable {
    use std::future::Future;
    use std::io;
    use std::pin::Pin;
    use std::sync::Arc;
    use tokio::runtime::Builder;
    use tokio::Syscalls;

    struct TestSyscalls;

    impl Syscalls for TestSyscalls {
        fn spawn(&self, future: Pin<Box<dyn Future<Output = ()>>>) {
            todo!("spawn")
        }

        fn spawn_blocking(&self, task: Box<dyn FnOnce()>) {
            todo!("spawn_blocking")
        }
        fn park(&self) -> Result<(), io::Error> {
            todo!("park")
        }

        fn park_timeout(&self, duration: std::time::Duration) -> Result<(), io::Error> {
            todo!("park_timeout")
        }

        fn unpark(&self) {
            todo!("unpark")
        }
    }

    #[test]
    fn syscall_spawn() {
        let syscalls = Arc::new(TestSyscalls);
        let mut runtime = Builder::new()
            .enable_all()
            .basic_scheduler()
            .syscalls(syscalls)
            .build()
            .unwrap();

        let result = runtime.block_on(async { 1 + 1 });
        assert_eq!(result, 2);
    }
}
