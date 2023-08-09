use super::ctx::{random_email, TestCtx};
use crate::api::{jwt::jwt_is_valid, GRILLON, PG_POOL, TOKIO_RUNTIME};
use async_trait::async_trait;
use grillon::{dsl::is, header, json, Assert};
use owlduty_domain::user::User;
use std::future::Future;
use uuid::Uuid;

pub(crate) struct SignupCtx<'a> {
    user: &'a User,
}

impl<'a> SignupCtx<'a> {
    fn new(user: &'a User) -> Self {
        Self { user }
    }
}

#[async_trait]
impl TestCtx for SignupCtx<'_> {
    async fn setup(&self) {}

    async fn teardown(&self) {
        let query_res = sqlx::query(
            r#"
            WITH user_with_team AS (
                SELECT team_id FROM auth.users
                JOIN auth.teams ON team_id = auth.teams.id
                WHERE email = $1::TEXT::CITEXT
            ), delete_user AS (
                DELETE FROM auth.users WHERE email = $1::TEXT::CITEXT
                AND team_id IN (SELECT team_id FROM user_with_team)
            )
            DELETE FROM auth.teams WHERE id IN (SELECT team_id FROM user_with_team);
            "#,
        )
        .bind(self.user.email.clone())
        .execute(&*PG_POOL)
        .await;

        match query_res {
            Ok(res) if res.rows_affected() != 1 => {
                panic!(
                    "[TEARDOWN] Expected 1 row to be removed but got: {}",
                    res.rows_affected()
                )
            }
            Ok(res) => println!("Result of the delete: {:#?} ", res),
            Err(err) => panic!("[TEARDOWN][QUERY FAILED] {}", err),
        };
    }
}

pub(crate) fn user_signup(user: &User) -> impl Future<Output = Assert> {
    let payload = json!(user);

    GRILLON
        .post("rpc/signup")
        .headers(vec![(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        )])
        .payload(payload)
        .assert()
}

#[test]
fn signup_without_team() {
    let user = User::new(&random_email(), "john.doe", "testpass", None);

    let assertion = async {
        user_signup(&user)
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

    SignupCtx::new(&user).run_test(assertion);
}

#[test]
fn user_already_exists() {
    let user = User::new(&random_email(), "john.doe", "testpass", None);
    let assertion = async {
        user_signup(&user).await;
        user_signup(&user).await.status(is(409));
    };

    SignupCtx::new(&user).run_test(assertion);
}

#[tokio::test]
async fn invalid_email() {
    let user = User::new("invalidATemail.com", "john.doe", "testpass", None);
    user_signup(&user).await.status(is(400));
}

#[test]
fn signup_with_team() {
    let team_id = async {
        sqlx::query!(
            r#"
            INSERT INTO auth.teams
            DEFAULT VALUES
            RETURNING id"#
        )
        .fetch_one(&*PG_POOL)
        .await
        .expect("Failed to create the team stub.")
        .id
    };

    let team_id = TOKIO_RUNTIME.block_on(team_id);
    let user = User::new(&random_email(), "john.doe", "testpass", Some(team_id));

    let assertion = async {
        // Signup user with existing team
        user_signup(&user).await.status(is(200));

        // Check the user is explicitly associated to a team. The Uuid
        // mapping will validate the ID type.
        let fetch_team_id: Uuid = sqlx::query!(
            "SELECT team_id from auth.users WHERE email = $1::TEXT::CITEXT",
            user.email.clone()
        )
        .fetch_one(&*PG_POOL)
        .await
        .expect("")
        .team_id;

        assert_eq!(
            fetch_team_id, team_id,
            "The team id from signup doesn't match the one in database"
        );
    };

    SignupCtx::new(&user).run_test(assertion);
}
