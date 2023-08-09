use crate::api::jwt::jwt_is_valid;

use super::ctx::{random_email, TestCtx};
use super::{GRILLON, PG_POOL};
use async_trait::async_trait;
use grillon::{dsl::is, header, json, Assert};
use std::future::Future;
use uuid::Uuid;

pub(crate) struct SigninCtx<'a> {
    email: &'a str,
    pass: &'a str,
}

impl<'a> SigninCtx<'a> {
    fn new(email: &'a str, pass: &'a str) -> Self {
        Self { email, pass }
    }
}

pub(crate) fn user_signin(email: &str, password: &str) -> impl Future<Output = Assert> {
    GRILLON
        .post("rpc/signin")
        .headers(vec![(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        )])
        .payload(json!({
            "email": email,
            "password": password
        }))
        .assert()
}

#[async_trait]
impl TestCtx for SigninCtx<'_> {
    async fn setup(&self) {
        let query_res =
            sqlx::query("insert into auth.users (name, email, pass) values ($1, $2, $3)")
                .bind("john.doe")
                .bind(self.email)
                .bind(self.pass)
                .execute(&*PG_POOL)
                .await;

        match query_res {
            Ok(res) if res.rows_affected() != 1 => {
                eprintln!(
                    "[SETUP] Expected 1 row to be inserted but got: {}",
                    res.rows_affected()
                )
            }
            Ok(_) => (),
            Err(err) => eprintln!("[SETUP] Query failed: {}", err),
        };
    }

    async fn teardown(&self) {
        let query_res = sqlx::query("DELETE FROM auth.users WHERE email = $1")
            .bind(self.email)
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

#[test]
fn signin_success() {
    let (email, pass) = (&random_email(), "pass");
    let assertion = async {
        user_signin(email, pass)
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
                let token: &str = json_body["token"].as_str().expect("Failed to retrive JWT");
                assert!(jwt_is_valid(token).is_ok(), "Invalid JWT");
            })
    };

    SigninCtx::new(email, pass).run_test(assertion);
}

#[test]
fn invalid_signin_credentials() {
    let good_email = &random_email();
    let (bad_email, good_pass) = ("inexistant@owlduty.com", "pass");
    let assertion = async { user_signin(bad_email, good_pass).await.status(is(400)) };

    // Create a user with valid credentials and test with invalid credentials

    // case 1: bad email
    SigninCtx::new(good_email, good_pass).run_test(assertion);

    // case 2: bad pass
    let assertion = async { user_signin(good_email, "bad pass").await.status(is(400)) };
    SigninCtx::new(good_email, good_pass).run_test(assertion);
}
