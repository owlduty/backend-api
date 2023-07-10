use super::TOKIO_RUNTIME;
use async_trait::async_trait;
use rand::distributions::{Alphanumeric, DistString};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub(crate) fn random_email<'a>() -> String {
    let rand_part = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    format!("john.doe+{rand_part}@owlduty.com")
}

#[async_trait]
pub(crate) trait TestContext {
    async fn setup(&self);
    async fn teardown(&self);
    fn run_test<F>(&self, test: F) -> ()
    where
        F: std::future::Future,
    {
        TOKIO_RUNTIME.block_on(async { self.setup().await });

        let assert = AssertUnwindSafe(async { test.await });
        let result = catch_unwind(|| {
            TOKIO_RUNTIME.block_on(assert);
        });

        TOKIO_RUNTIME.block_on(async { self.teardown().await });

        assert!(result.is_ok());
    }
}
