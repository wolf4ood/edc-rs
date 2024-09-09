use common::setup_provider_client;

mod common;

#[tokio::test]
async fn should_fetch_dataplanes() {
    let client = setup_provider_client();

    let response = client.data_planes().list().await.unwrap();
    assert!(response.len() > 0);
}
