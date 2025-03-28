use std::{fs, io, path};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    label: String,
    children: Vec<TreeNode>,
    is_dir: bool,
    path: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    root_path: String,
}

/// 列举文件树
#[tauri::command]
pub fn tree(path: &str) -> Result<TreeNode, String> {
    let p = path::Path::new(path);

    let result = ls(p);

    match result {
        Ok(n) => {
            // 当构建文件树成功时保存配置
            let setting = Setting {
                root_path: path.to_string(),
            };
            let result = write_setting(setting);
            if let Err(e) = result {
                return Err(e);
            }

            Ok(n)
        }
        Err(e) => Err(format!("获取失败 [{e}]")),
    }
}

/// 读取配置
#[tauri::command]
pub fn read_setting() -> Result<Setting, String> {
    let file = fs::File::open(SETTING_FILEPATH);
    let file = match file {
        Ok(f) => f,
        Err(e) => return Err(format!("读文件失败 [{e}]")),
    };

    let result = serde_json::from_reader(file);
    match result {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("读取配置失败 [{e}]")),
    }
}

/// 覆盖写文件，将[`content`]写入[`path`]文件
#[tauri::command]
pub fn write(path: &str, content: &str) -> Result<String, String> {
    let result = create(path);
    if let Err(e) = result {
        return Err(e);
    }

    let result = fs::write(path, content);
    match result {
        Ok(()) => Ok("写入成功".into()),
        Err(e) => Err(format!("写入失败 {e}")),
    }
}

/// 读文件[`path`]全部内容
#[tauri::command]
pub fn read(path: &str) -> Result<String, String> {
    let result = fs::read_to_string(path);

    match result {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("读取文件失败 {e}")),
    }
}

/// 写入配置
fn write_setting(setting: Setting) -> Result<String, String> {
    let file = create(SETTING_FILEPATH);
    let file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let result = serde_json::to_writer(file, &setting);

    match result {
        Ok(()) => Ok("保存成功".into()),
        Err(e) => Err(format!("保存失败 [{e}]")),
    }
}

/// 创建[`path`]目录
fn mkdir(path: &str) -> Result<String, String> {
    let p = path::Path::new(path);
    if !p.exists() {
        if let Err(e) = fs::create_dir_all(path) {
            return Err(format!("创建目录失败 {e}"));
        }
    }

    Ok(format!("{} 创建成功", path))
}

/// 创建[`path`]文件
fn create(path: &str) -> Result<fs::File, String> {
    // 确保父目录存在
    let p = path::Path::new(path);
    if let Some(parent) = p.parent() {
        let result = mkdir(parent.to_str().unwrap_or_default());
        if let Err(e) = result {
            return Err(format!("创建父目录失败 [{e}]"));
        }
    }

    // 创建文件
    let file = fs::File::create(path);
    match file {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("创建文件失败 {e}")),
    }
}

/// 递归列举[`path`]文件树
fn ls(path: &path::Path) -> Result<TreeNode, io::Error> {
    let mut node = TreeNode {
        // file_stem 不带扩展名
        label: path.file_stem().unwrap_or_default().to_str().unwrap_or_default().to_string(),
        children: Vec::new(),
        is_dir: path.is_dir(),
        path: path.to_str().unwrap_or_default().to_string(),
    };

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            let child_node = ls(&child_path)?;
            node.children.push(child_node);
        }
    }

    Ok(node)
}

/// 设置文件路径
const SETTING_FILEPATH: &str = "../conf/settings.json";

#[cfg(test)]
mod tests {
    use crate::fs;
    use crate::fs::Setting;

    #[test]
    fn tree_test() {
        let path = "C:\\dev";
        let data = fs::tree(path);
        println!("{:?}", data);
    }

    #[test]
    fn mkdir_test() {
        let path = "../conf";
        let ret = fs::mkdir(path);
        println!("{:?}", ret);
    }

    #[test]
    fn create_test() {
        let path = fs::SETTING_FILEPATH;
        let ret = fs::create(path);
        println!("{:?}", ret);
    }

    #[test]
    fn write_test() {
        let path = fs::SETTING_FILEPATH;
        let content = "{\"rootPath\":\"C:/dev\"}";
        let ret = fs::write(path, content);
        println!("{:?}", ret)
    }

    #[test]
    fn read_test() {
        let path = fs::SETTING_FILEPATH;
        let ret = fs::read(path);
        println!("{:?}", ret)
    }

    #[test]
    fn read_setting_test() {
        let result = fs::read_setting();
        println!("{:?}", result)
    }

    #[test]
    fn write_setting_test() {
        let setting = Setting {
            root_path: "C:\\dev".to_string(),
        };

        let result = fs::write_setting(setting);
        println!("{:?}", result)
    }
}
