use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: String,
    pub team_id: Option<Uuid>,
}

impl User {
    pub fn new(email: &str, name: &str, password: &str, team_id: Option<Uuid>) -> Self {
        Self {
            email: email.into(),
            name: name.into(),
            password: password.into(),
            team_id,
        }
    }
}
