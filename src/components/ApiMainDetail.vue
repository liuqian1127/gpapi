<script setup>
import { computed, onMounted, ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage } from "element-plus"

const { path } = defineProps(["path"])

const sendLoading = ref(false)
const saveLoading = ref(false)

const method = ref("GET")
const url = ref("")
const header = ref("Content-Type: application/json;charset=UTF-8\nAccept: application/json, text/plain, */*\n")
const input = ref("")
const output = ref("")

onMounted(() => {
  invoke("read", { path })
    .then(resp => {
      if (resp === "") {
        return
      }
      resp = JSON.parse(resp)
      method.value = resp.method
      url.value = resp.url
      header.value = resp.header
      input.value = resp.input
      output.value = resp.output
    })
    .catch(err => ElMessage.error(err))
})

function send() {
  sendLoading.value = true

  if (url.value === "") {
    ElMessage.error("请输入接口地址")
    sendLoading.value = false
    return
  }

  output.value = ""

  const args = { method: method.value, url: url.value, header: header.value, input: input.value }
  invoke("do_request", args)
    .then(resp => output.value = resp)
    .catch(err => ElMessage.error(err))
    .finally(() => sendLoading.value = false)
}

function save() {
  saveLoading.value = true

  const args = { method: method.value, url: url.value, header: header.value, input: input.value, output: output.value }

  invoke("write", { path, content: JSON.stringify(args) })
    .then(_ => ElMessage.success("保存成功"))
    .catch(_ => ElMessage.error("保存失败"))
    .finally(() => saveLoading.value = false)
}

const placeholder = computed(() => {
  if (method.value === "GET" || method.value === "DELETE") {
    return "请提供类似foo=a&foo=b格式的参数"
  } else {
    return '请根据【请求头】中【Content-Type】提供，具体如下：\n' +
      '1、application/json：需传JSON格式的请求体，如{"lang":"rust","body":"json"}\n' +
      '2、application/x-www-form-urlencoded：From表单格式请求体，如foo=a&foo=b\n' +
      '3、multipart/form-data：文件上传，如file=path/to/test.txt\n' +
      '4、未指定：作为原始内容传递给接口'
  }
})
</script>

<template>
  <el-row :gutter="20">
    <el-col :span="12">
      <el-input v-model="url" clearable placeholder="请输入接口地址">
        <template #prepend>
          <el-select v-model="method" style="width:100px;">
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
            <el-option label="PATCH" value="PATCH" />
          </el-select>
        </template>
      </el-input>
    </el-col>
    <el-col :span="12">
      <el-button :loading="sendLoading" type="primary" @click="send">发送</el-button>
      <el-button :loading="saveLoading" type="success" @click="save">保存</el-button>
    </el-col>
  </el-row>
  <el-row :gutter="20" style="margin-top:20px">
    <el-col :span="12">
      <el-text type="primary">请求头：</el-text>
      <el-input v-model="header" :rows="10" resize="none" type="textarea" />
    </el-col>
    <el-col :span="12">
      <el-text type="primary">请求体：</el-text>
      <el-input v-model="input" :rows="10" resize="none" type="textarea" :placeholder="placeholder" />
    </el-col>
  </el-row>
  <el-row style="margin-top:20px">
    <el-col>
      <el-text type="primary">响应体：</el-text>
      <el-input v-model="output" :rows="28" readonly resize="none" type="textarea" />
    </el-col>
  </el-row>
</template>
