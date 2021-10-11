use std::time::Duration;

use reqwest::Client;

use serde_json::{json, Value};

/// Adds the workshop item to the download queue of steamworkshopdownloader.io
/// and returns the process UUID if everything went right.
pub async fn queue_download(
    client: Client,
    workshop_item_id: u32,
) -> Result<String, reqwest::Error> {
    let endpoint = "https://backend-02-prd.steamworkshopdownloader.io/api/download/request";

    // TODO: Use this instead of the old `map!` macro inside `steam_api.rs`
    let params = json!({
        "publishedFileId": workshop_item_id,
        "collectionId": 0,
        "extract": false,
        "hidden": false,
        "direct": false,
        "autodownload": true
    });

    let response = client.post(endpoint).json(&params).send().await?;
    let content = response.text().await?;
    let json_value: Value = serde_json::from_str(content.as_str()).unwrap();

    Ok(json_value["uuid"].as_str().unwrap().to_string())
}

/// When this returns Ok(true), it means that the download is ready and we can proceed to
/// download the item.
pub async fn check_status(client: Client, uuid: String) -> Result<bool, reqwest::Error> {
    // Wait half a second
    let duration = Duration::from_millis(500);
    async_std::task::sleep(duration).await;

    let endpoint = "https://backend-02-prd.steamworkshopdownloader.io/api/download/status";
    let params = json!({ "uuids": [uuid] });
    let request = client.post(endpoint).json(&params);
    let response = request.send().await?;

    let content = response.text().await?;
    let json_value: Value = serde_json::from_str(content.as_str()).unwrap();

    if json_value[uuid]["status"] == "prepared" {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Downloads the mod from steamworkshopdownloader.io and returns it in `zip` format as a byte
/// vector.
pub async fn download_mod(client: Client, uuid: String) -> Result<Vec<u8>, reqwest::Error> {
    let endpoint = "https://backend-02-prd.steamworkshopdownloader.io/api/download/transmit";
    let params = json!({ "uuid": uuid });
    let response = client.get(endpoint).query(&params).send().await?;
    let content = response.bytes().await?;
    Ok(content.to_vec())
}
