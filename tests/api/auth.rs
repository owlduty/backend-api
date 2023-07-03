use super::TestContext;
use crate::api::GRILLON;
use grillon::{dsl::is, json};

struct AuthContext;
impl TestContext for AuthContext {
    fn setup() {
        println!("Setup Auth context");
    }

    fn teardown() {
        println!("teardown auth context");
    }
}

#[test]
fn signup_success() {
    let assert = async {
        GRILLON
            .post("/rpc/signup")
            .payload(json!({
                "name": "john doe",
                "email": "john.die@owlduty.com",
                "password": "testpass"
            }))
            .assert()
            .await
            .status(is(201));
    };

    AuthContext::run_test(assert);
}
