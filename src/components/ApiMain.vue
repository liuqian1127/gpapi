<script setup>
import {ref} from "vue"
import {invoke} from "@tauri-apps/api/core"

import {Right} from "@element-plus/icons-vue"
import {ElMessage} from "element-plus";

const loading = ref(false)

const method = ref("GET")
const url = ref("")
const header = ref("Content-Type: application/json;charset=UTF-8\nAccept: application/json, text/plain, */*\n")
const input = ref("")
const output = ref("")

async function sendRequest() {
  loading.value = true

  if (url.value === "") {
    ElMessage.error("请输入接口地址")
    loading.value = false
    return
  }

  output.value = ""

  const args = {method: method.value, url: url.value, header: header.value, input: input.value}
  invoke("do_request", args)
      .then(resp => output.value = resp)
      .catch(err => ElMessage.error(err))
      .finally(() => loading.value = false)
}
</script>

<template>
  <div class="main-container">
    <el-row>
      <el-col>
        <el-input v-model="url" clearable placeholder="请输入接口地址">
          <template #prepend>
            <el-select v-model="method" style="width:110px;">
              <el-option label="GET" value="GET"/>
              <el-option label="POST" value="POST"/>
              <el-option label="PUT" value="PUT"/>
              <el-option label="PATCH" value="PATCH"/>
              <el-option label="DELETE" value="DELETE"/>
            </el-select>
          </template>
          <template #append>
            <el-button :icon="Right" :loading="loading" @click="sendRequest"/>
          </template>
        </el-input>
      </el-col>
    </el-row>
    <el-row :gutter="20" style="margin-top:20px">
      <el-col :span="12">
        <el-text type="primary">请求头：</el-text>
        <el-input v-model="header" resize="none" rows="10" type="textarea"/>
      </el-col>
      <el-col :span="12">
        <el-text type="primary">请求体：</el-text>
        <el-input v-model="input" :disabled="method==='GET'||method==='DELETE'" resize="none" rows="10" type="textarea"/>
      </el-col>
    </el-row>
    <el-row style="margin-top:20px">
      <el-col>
        <el-text type="primary">响应体：</el-text>
      </el-col>
    </el-row>
    <el-row>
      <el-col :span="24">
        <el-input v-model="output" readonly resize="none" rows="30" type="textarea"/>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped>
.main-container {
  height: 100%;
}
</style>
