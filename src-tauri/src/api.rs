use std::str::FromStr;
use tauri_plugin_http::reqwest;

#[tauri::command]
pub async fn do_request(
    method: &str,
    url: &str,
    header: &str,
    input: &str,
) -> Result<String, String> {
    match method {
        "GET" => get(url, header).await,
        "DELETE" => del(url, header).await,
        "POST" => post(url, header, input).await,
        "PUT" => put(url, header, input).await,
        "PATCH" => patch(url, header, input).await,
        other => Err(format!("暂不支持{other}请求方法")),
    }
}

async fn get(url: &str, header: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client.get(url).headers(parse_header(header)).send().await;
    parse_response(response).await
}

async fn del(url: &str, header: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .delete(url)
        .headers(parse_header(header))
        .send()
        .await;
    parse_response(response).await
}

async fn post(url: &str, header: &str, input: &str) -> Result<String, String> {
    let headers = parse_header(header);
    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(v) => v.to_str().unwrap(),
        None => return Err("未提供Content-Type".into()),
    };

    let client = reqwest::Client::new();
    let response;
    if content_type.contains("application/x-www-form-urlencoded") {
        response = client.post(url).form(input).send().await;
    } else if content_type.contains("application/json") {
        let json: serde_json::Value = serde_json::from_str(input).unwrap();
        response = client.post(url).json(&json).send().await;
    } else {
        response = client.post(url).body(input.to_string()).send().await;
    }

    parse_response(response).await
}

async fn put(url: &str, header: &str, input: &str) -> Result<String, String> {
    let headers = parse_header(header);
    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(v) => v.to_str().unwrap(),
        None => return Err("未提供Content-Type".into()),
    };

    let client = reqwest::Client::new();
    let response;
    if content_type.contains("application/x-www-form-urlencoded") {
        response = client.put(url).form(input).send().await;
    } else if content_type.contains("application/json") {
        let json: serde_json::Value = serde_json::from_str(input).unwrap();
        response = client.put(url).json(&json).send().await;
    } else {
        response = client.put(url).body(input.to_string()).send().await;
    }

    parse_response(response).await
}

async fn patch(url: &str, header: &str, input: &str) -> Result<String, String> {
    let headers = parse_header(header);
    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(v) => v.to_str().unwrap(),
        None => return Err("未提供Content-Type".into()),
    };

    let client = reqwest::Client::new();
    let response;
    if content_type.contains("application/x-www-form-urlencoded") {
        response = client.patch(url).form(input).send().await;
    } else if content_type.contains("application/json") {
        let json: serde_json::Value = serde_json::from_str(input).unwrap();
        response = client.patch(url).json(&json).send().await;
    } else {
        response = client.patch(url).body(input.to_string()).send().await;
    }

    parse_response(response).await
}

async fn parse_response(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<String, String> {
    match response {
        Ok(o) => Ok(o.text().await.unwrap()),
        Err(e) => {
            if e.is_connect() {
                Err("网络不通".into())
            } else {
                Err("其他异常".into())
            }
        }
    }
}

fn parse_header(header: &str) -> reqwest::header::HeaderMap {
    let mut map = reqwest::header::HeaderMap::new();

    // 清理掉空格和\r字符
    let header = header.replace(" ", "").replace("\r", "");
    header.split("\n").for_each(|line| {
        if !line.is_empty() {
            let kv: Vec<&str> = line.split(":").collect();
            let k = reqwest::header::HeaderName::from_str(kv[0]).unwrap();
            let v = kv[1].parse().unwrap();
            map.insert(k, v);
        }
    });

    map
}

#[cfg(test)]
mod api_tests {
    use crate::api::parse_header;

    #[test]
    fn parse_header_test() {
        let header = "Content-Type: application/json;charset=UTF-8\nAccept: application/json, text/plain, */*\n";
        let header_map = parse_header(header);
        assert_eq!(header_map.len(), 2);
        assert_eq!(
            header_map.get("Content-Type").unwrap(),
            "application/json;charset=UTF-8"
        );
        assert_eq!(
            header_map.get("Accept").unwrap(),
            "application/json,text/plain,*/*"
        );
    }
}
