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

    <api-aside-tree :data="treeData"/>
  </div>
</template>

<style scoped>
.aside-container {
  height: 100%;
  max-width: 600px;
}
</style>
