use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print_no_body().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({"username" : "demo1", "password": "welcome"}),
    );

    req_login.await?.print().await?;

    Ok(())
}
