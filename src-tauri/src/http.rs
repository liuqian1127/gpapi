use std::fs::File;
use std::io::Read;
use std::path;
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
        Some(v) => match v.to_str() {
            Ok(v) => v,
            Err(e) => return Err(format!("提取Content-Type错误 {e}")),
        },
        None => return Err("未提供Content-Type".into()),
    };

    let client = reqwest::Client::new();
    let response;
    if content_type.contains("application/json") {
        let json: serde_json::Value = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => return Err(format!("JSON字符串错误 {e}")),
        };
        response = client.post(url).json(&json).send().await;
    } else if content_type.contains("application/x-www-form-urlencoded") {
        response = client.post(url).form(input).send().await;
    } else if content_type.contains("multipart/form-data") {
        // 上传文件，此时input的格式为：file=path/to/file.txt
        let input = input.replace(" ", "").replace("\r", "");
        let parts: Vec<&str> = input.splitn(2, "=").collect();
        if parts.len() != 2 {
            return Err("请求体格式错误，参考：file=/path/to/test.txt".to_string());
        }

        let file_form = parts[0];
        let file_path = parts[1];
        let file_path = path::Path::new(file_path);
        if !file_path.exists() {
            return Err(format!("{}不存在", file_path.display()));
        }
        if !file_path.is_file() {
            return Err(format!("{}不是文件", file_path.display()));
        }

        // 打开文件
        let result = File::open(file_path);
        let mut file = match result {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("打开{}文件失败 {}", file_path.display(), e));
            }
        };
        let mut file_content = Vec::new();
        let result = file.read_to_end(&mut file_content);
        match result {
            Ok(s) => {
                if s == 0 {
                    return Err(format!("{}内容为空", file_path.display()));
                }
            }
            Err(e) => {
                return Err(format!("{}读取失败 {}", file_path.display(), e));
            }
        }

        // 创建 multipart 表单
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_name = file_name.to_string().clone(); // 避免生命周期问题
        let part = reqwest::multipart::Part::bytes(file_content).file_name(file_name);
        let file_form = file_form.to_string().clone(); // 避免生命周期问题
        let form = reqwest::multipart::Form::new().part(file_form, part);

        response = client.post(url).multipart(form).send().await;
    } else {
        response = client.post(url).body(input.to_string()).send().await;
    }

    parse_response(response).await
}

async fn put(url: &str, header: &str, input: &str) -> Result<String, String> {
    let headers = parse_header(header);
    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(v) => match v.to_str() {
            Ok(v) => v,
            Err(e) => return Err(format!("提取Content-Type错误 {e}")),
        },
        None => return Err("未提供Content-Type".into()),
    };

    let client = reqwest::Client::new();
    let response;
    if content_type.contains("application/x-www-form-urlencoded") {
        response = client.put(url).form(input).send().await;
    } else if content_type.contains("application/json") {
        let json: serde_json::Value = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => return Err(format!("JSON字符串错误 {e}")),
        };
        response = client.put(url).json(&json).send().await;
    } else {
        response = client.put(url).body(input.to_string()).send().await;
    }

    parse_response(response).await
}

async fn patch(url: &str, header: &str, input: &str) -> Result<String, String> {
    let headers = parse_header(header);
    let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(v) => match v.to_str() {
            Ok(v) => v,
            Err(e) => return Err(format!("提取Content-Type错误 {e}")),
        },
        None => return Err("未提供Content-Type".into()),
    };

    let client = reqwest::Client::new();
    let response;
    if content_type.contains("application/x-www-form-urlencoded") {
        response = client.patch(url).form(input).send().await;
    } else if content_type.contains("application/json") {
        let json: serde_json::Value = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => return Err(format!("JSON字符串错误 {e}")),
        };
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
        Ok(resp) => Ok(resp.text().await.unwrap()),
        Err(err) => {
            if err.is_connect() {
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
mod tests {
    use crate::http;

    #[test]
    fn parse_header_test() {
        let header = "Content-Type: application/json;charset=UTF-8\nAccept: application/json, text/plain, */*\n";
        let header_map = http::parse_header(header);
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

    #[tokio::test]
    async fn get_test() {
        let url = "https://tauri.app";
        let header = "Content-Type: application/json;charset=UTF-8\nAccept: application/json, text/plain, */*\n";
        let resp = http::get(url, header).await;
        match resp {
            Ok(output) => {
                println!("{output}");
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
}
