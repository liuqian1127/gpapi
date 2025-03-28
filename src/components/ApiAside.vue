<script setup>
import {onMounted, ref} from "vue"
import {invoke} from "@tauri-apps/api/core"
import {ElMessage} from "element-plus"
import {Refresh} from "@element-plus/icons-vue"
import ApiAsideTree from "@/components/ApiAsideTree.vue";

// 根目录
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
  const path = rootPath.value + ""
  if (path === "") {
    ElMessage.error("请输入工作目录")
    return
  }

  loading.value = true

  // 防止盘符小写
  rootPath.value = path.slice(0, 1).toUpperCase() + path.slice(1)
  const args = {path: rootPath.value}
  invoke("tree", args)
      .then(resp => {
        resp.children[0].isRoot = true
        treeData.value = resp.children
      })
      .catch(err => ElMessage.error(err))
      .finally(() => loading.value = false)
}

// 清空文件树
const handleClear = () => treeData.value = []
</script>

<template>
  <div class="aside-container">
    <el-input v-model="rootPath" clearable placeholder="请输入工作目录" @clear="handleClear">
      <template #append>
        <el-button :disabled="rootPath===''" :icon="Refresh" :loading="loading" @click="handleLoad"/>
      </template>
    </el-input>

    <api-aside-tree :root-path="rootPath" :tree-data="treeData" @refresh="handleLoad"/>
  </div>
</template>

<style scoped>
.aside-container {
  height: 100%;
  max-width: 600px;
}
</style>
