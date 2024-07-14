pub mod environment;
pub mod tests;
pub mod compile;
pub mod deploy;



pub async fn stage_daemon(
    build_id: &str,
    repo_name: &str,
    repo_url: &str,
    webhook_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    environment::rust_environment(build_id, repo_name, repo_url, webhook_url).await?;
    tests::rust_tests(build_id, repo_name, repo_url, webhook_url).await?;
    compile::rust_build(build_id, repo_name, repo_url, webhook_url).await?;
    deploy::rust_deploy(build_id, repo_name, repo_url, webhook_url).await?;
    Ok(())

}
