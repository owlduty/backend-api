use super::TestContext;
use crate::api::{GRILLON, PG_POOL};
use async_trait::async_trait;
use grillon::{dsl::is, header, json, Assert};
use rand::distributions::{Alphanumeric, DistString};
use std::future::Future;
use uuid::Uuid;

pub(crate) struct SignupCtx<'a> {
    user_email: &'a str,
}

impl<'a> SignupCtx<'a> {
    fn new(user_email: &'a str) -> Self {
        Self { user_email }
    }
}

#[async_trait]
impl TestContext for SignupCtx<'_> {
    async fn setup(&self) {}

    async fn teardown(&self) {
        let query_res = sqlx::query("DELETE FROM auth.users WHERE email = $1")
            .bind(self.user_email)
            .execute(&*PG_POOL)
            .await;

        match query_res {
            Ok(res) if res.rows_affected() != 1 => {
                eprintln!(
                    "[TEARDOWN] Expected 1 row to be removed but got: {}",
                    res.rows_affected()
                )
            }
            Ok(_) => (),
            Err(err) => eprintln!("[TEARDOWN] Query failed: {}", err),
        };
    }
}

fn random_email<'a>() -> String {
    let rand_part = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    format!("john.doe+{rand_part}@owlduty.com")
}

fn user_signup(email: &str, name: &str, password: &str) -> impl Future<Output = Assert> {
    GRILLON
        .post("rpc/signup")
        .headers(vec![(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        )])
        .payload(json!({
            "name": name,
            "email": email,
            "password": password
        }))
        .assert()
}

#[test]
fn signup_success() {
    let email = &random_email();
    let assertion = async {
        user_signup(email, "john.doe", "testpass")
            .await
            .status(is(200))
            .assert_fn(|assert| {
                assert!(assert.json.is_some());

                let json_body = assert.json.as_ref().unwrap();
                // check user's id
                assert!(
                    json_body["profile"]["id"].is_string(),
                    "User id should be a string"
                );

                let uuid = &json_body["profile"]["id"].as_str().unwrap();
                assert!(
                    Uuid::parse_str(uuid).is_ok(),
                    "User id should be a valid UUID v4"
                );

                // Check JWT token
                assert!(
                    json_body["token"].is_string(),
                    "JWT token should be a string"
                );
                assert!(
                    json_body["token"].to_string().len() > 1,
                    "JWT token should be longer than 1 character"
                )
            })
    };

    SignupCtx::new(email).run_test(assertion);
}

#[test]
fn user_already_exists() {
    let email = &random_email();
    let assertion = async {
        user_signup(email, "john.doe", "testpass").await;
        user_signup(email, "john.doe", "testpass")
            .await
            .status(is(409));
    };

    SignupCtx::new(email).run_test(assertion);
}

#[tokio::test]
async fn invalid_email() {
    user_signup("invalidATemail.com", "john.doe", "testpass")
        .await
        .status(is(400));
}
