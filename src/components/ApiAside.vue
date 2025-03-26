<script setup>
import {ref} from "vue"
import {invoke} from "@tauri-apps/api/core"
import {ElMessage} from "element-plus"
import {Refresh} from "@element-plus/icons-vue"

const path = ref("")
const data = ref([])
const loading = ref(false)

// 加载树
const handleLoad = () => {
  if (path.value === "") {
    ElMessage.error("请输入工作目录")
    return
  }

  loading.value = true
  const args = {path: path.value}
  invoke("ls", args)
      .then(resp => {
        resp = JSON.parse(resp)
        data.value = resp.children
      })
      .catch(() => ElMessage.error("加载文件数异常"))
      .finally(() => loading.value = false)
}

// 清空树
const handleClear = () => data.value = []

// 树节点被点击
const handleNodeClick = data => {
  console.log(data)
}
</script>

<template>
  <div class="aside-container">
    <el-input v-model="path" placeholder="请输入工作目录" clearable @clear="handleClear">
      <template #append>
        <el-button :icon="Refresh" :disabled="path===''" :loading="loading" @click="handleLoad"/>
      </template>
    </el-input>
    <el-tree :data="data" @node-click="handleNodeClick"/>
  </div>
</template>

<style scoped>
.aside-container {
  height: 100%;
  max-width: 600px;
}
</style>
