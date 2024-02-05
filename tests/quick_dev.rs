use anyhow::Result;

#[tokio::test]
pub async fn test_response_of_failed_login() -> Result<()> {
    let params = [("username", "Test"), ("password", "Test")];
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/login")
        .form(&params)
        .send()
        .await?;

    assert_eq!(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        res.status(),
        "Assert failed login returns HTTP Status Code INTERNAL_SERVER_ERROR"
    );
    Ok(())
}

#[tokio::test]
pub async fn test_root() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print_no_body().await?;

    Ok(())
}
