use std::time::Duration;

use reqwest::Client;
use serde::Deserialize;

const CHAIN_ID: &str = "3c4bcd4aacdf3bc5a483e277f4e445f81f4ab92951ed10d560af6a1db0365e6a";
const APPLICATION_ID: &str = "1bb12f5f29013c979fa313aa441af6b3b4fa24027976473a487579642dc3dad7";

const ID: &str = "6a300ad5-15c5-4ac6-be27-b8a4d1d972ee";
const HASH: &str = "903b1c65f8ad53b2acf8704cf2ae766eae37eac0b5196321f34c3e07df3ecf30";
const SIGNATURE: &str = "2c546b2937ea6452c1f381d0c20077f02f63af72f77c1ed76edd4e220e2f59733fada263eab046c55d89a1ca8a6e3504d65eb4d6f40205c3cc7ac2c4603ffb04";

#[derive(Deserialize, Debug)]
struct Response {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    value: Option<Telemetry>,
}

#[derive(Deserialize, Debug)]
struct Telemetry {
    hash: String,
    signature: String,
}

#[tokio::test]
async fn test_graphql() {
    let httpclient = reqwest::Client::new();

    create_telemetry(&httpclient).await;

    let body = format!(
        r#"{{"query": "{{ value(id: \"{}\") {{ hash signature }} }}"}}"#,
        ID
    );
    let res = httpclient
        .post(format!(
            "http://localhost:7070/chains/{}/applications/{}",
            CHAIN_ID, APPLICATION_ID
        ))
        .body(body)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(1))
        .send()
        .await
        .unwrap();
    assert_eq!(reqwest::StatusCode::OK, res.status());

    let data: Response = res.json().await.unwrap();
    let data = data.data.value.unwrap();
    assert_eq!(HASH, data.hash);
    assert_eq!(SIGNATURE, data.signature);
}

async fn create_telemetry(httpclient: &Client) {
    let body: String = format!(
        r#"{{"query": "mutation {{ land(id: \"{}\" hash: \"{}\" signature: \"{}\") }}"}}"#,
        ID, HASH, SIGNATURE,
    );
    let res = httpclient
        .post(format!(
            "http://localhost:7070/chains/{}/applications/{}",
            CHAIN_ID, APPLICATION_ID
        ))
        .body(body)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(1))
        .send()
        .await
        .unwrap();
    assert_eq!(reqwest::StatusCode::OK, res.status());
}
