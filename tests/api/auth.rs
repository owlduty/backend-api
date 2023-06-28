use crate::api::GRILLON;
use grillon::{dsl::is, json};

#[tokio::test]
async fn signup_success() {
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
    // .json_path("" is(json!({})));
}
