<script setup>
import {useTabStore} from "@/store/tab.js"
import {storeToRefs} from "pinia"

const {data} = defineProps(['data'])

const tabStore = useTabStore()
const {currentTab} = storeToRefs(tabStore)
const {addTab} = tabStore

// 点击文件树节点
const handleNodeClick = data => {
  if (data.isDir) {
    return
  }

  const tab = {
    name: data.path,
    label: data.label
  }
  addTab(tab)

  currentTab.value = data.path
}

// 文件树右键菜单
const handleNodeContextMenu = (event, data) => {
  console.log(event)
  console.log(data)
}
</script>

<template>
  <el-tree :data="data" empty-text="无数据" highlight-current @node-click="handleNodeClick" @node-contextmenu="handleNodeContextMenu"/>
</template>

<style scoped>
</style>
