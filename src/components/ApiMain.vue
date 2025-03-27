<script setup>
import {useTabStore} from "@/store/tab"
import {storeToRefs} from "pinia"
import ApiMainDetail from "@/components/ApiMainDetail.vue"

const tabStore = useTabStore()
const {tabs, currentTab} = storeToRefs(tabStore)
const {removeTab} = tabStore
</script>

<template>
  <div class="main-container">
    <el-tabs v-if="tabs.length>0" v-model="currentTab" closable type="card" @tab-remove="removeTab">
      <el-tab-pane v-for="(item, index) in tabs" :key="index" :label="item.label" :name="item.name">
        <api-main-detail :key="index" :path="item.name"></api-main-detail>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.main-container {
  height: 100%;
}
</style>
