<script setup>
import {onMounted, ref} from "vue"
import {invoke} from "@tauri-apps/api/core"
import {ElMessage} from "element-plus"
import {Refresh} from "@element-plus/icons-vue"

// 根目录 todo 待改为读写配置文件
const rootPath = ref("")
// 文件树数据
const treeData = ref([])
// 加载文件树控件
const loading = ref(false)

// 读取配置
onMounted(() => {
  invoke("read_setting")
      .then(resp => {
        rootPath.value = resp.rootPath
        handleLoad()
      })
      .catch(err => ElMessage.error(err))
})

// 加载文件树
const handleLoad = () => {
  if (rootPath.value === "") {
    ElMessage.error("请输入工作目录")
    return
  }

  loading.value = true
  const args = {path: rootPath.value}
  invoke("tree", args)
      .then(resp => treeData.value = resp.children)
      .catch(err => ElMessage.error(err))
      .finally(() => loading.value = false)
}

// 清空树
const handleClear = () => treeData.value = []

// 点击文件树节点
const handleNodeClick = data => {
  console.log(data)
}

// 文件树右键菜单
const handleNodeContextMenu = (event, data) => {
  console.log(event)
  console.log(data)
}
</script>

<template>
  <div class="aside-container">
    <el-input v-model="rootPath" placeholder="请输入工作目录" clearable @clear="handleClear">
      <template #append>
        <el-button :icon="Refresh" :disabled="rootPath===''" :loading="loading" @click="handleLoad"/>
      </template>
    </el-input>

    <el-tree :data="treeData" highlight-current empty-text="无数据" @node-click="handleNodeClick" @node-contextmenu="handleNodeContextMenu"/>
  </div>
</template>

<style scoped>
.aside-container {
  height: 100%;
  max-width: 600px;
}
</style>
