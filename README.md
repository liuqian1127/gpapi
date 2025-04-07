# gpapi

get、post API 的工具

## 创建项目

```bash
npm create tauri-app@latest
```

```text
> npx
> create-tauri-app

✔ Project name · gpapi
✔ Identifier · com.gpapi.app
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)
✔ Choose your package manager · npm
✔ Choose your UI template · Vue - (https://vuejs.org/)
✔ Choose your UI flavor · JavaScript

Template created! To get started run:
  cd gpapi
  npm install
  npm run tauri android init

For Desktop development, run:
  npm run tauri dev

For Android development, run:
  npm run tauri android dev
```

## 依赖

### ElementPlus

```bash
# 组件
npm install element-plus
# 图标
npm install @element-plus/icons-vue
```

### Tauri

```bash
# HTTP客户端
npm run tauri add http
# 文件系统
npm run tauri add fs
```

### Pinia

```bash
npm install pinia
```

## 说明

### 类POST用法

在使用`POST`、`PUT`、`PATCH`时，需提供与`请求头`中`Content-Type`类型对应的`请求体`数据，如：

* `application/json` JSON类型的请求体，如`{"lang":"rust","body":"json"}`
* `application/x-www-form-urlencoded` From 表单请求体，如`foo=bar&baz=quux`
* `multipart/form-data` 文件上传，如`file=path/to/file.txt`
* 未指定内容类型，则作为`RAW`类型传递给接口
