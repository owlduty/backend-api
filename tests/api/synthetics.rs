use crate::api::GRILLON;
use grillon::dsl::is;

#[tokio::test]
async fn get_synthetics() {
    GRILLON.get("synthetics").assert().await.status(is(200));
}
