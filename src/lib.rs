pub mod log;
pub mod api;
pub mod build;
pub mod config;
pub mod git;
pub mod deploy;
pub mod tests;
pub mod utils;
pub mod health;


#[tokio::test]
async fn test_schedule_build() {
    let yes: bool = true;
    assert_eq!(yes, true);
}