use serde::{Deserialize, Serialize};
use std::{fs, io, path};

#[tauri::command]
pub fn ls(path: &str) -> String {
    let path = path::Path::new(path);

    let data = ls_r(path).unwrap();

    serde_json::to_string_pretty(&data).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct TreeNode {
    label: String,
    children: Vec<TreeNode>,
}

fn ls_r(path: &path::Path) -> io::Result<TreeNode> {
    let mut node = TreeNode {
        label: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        children: Vec::new(),
    };

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            let child_node = ls_r(&child_path)?;
            node.children.push(child_node);
        }
    }

    Ok(node)
}

#[cfg(test)]
mod fs_tests {
    use crate::fs::ls;

    #[test]
    fn ls_test() {
        let path = "C:\\dev";
        let data = ls(path);
        println!("{}", data);
    }
}
