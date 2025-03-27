<script setup>
import ApiAside from "@/components/ApiAside.vue"
import ApiMain from "@/components/ApiMain.vue"
</script>

<template>
  <el-container>
    <el-aside style="border: 1px solid #cfcfc4">
      <api-aside/>
    </el-aside>
    <el-main>
      <api-main/>
    </el-main>
  </el-container>
</template>

<style scoped>
.el-container {
  height: 100%;
}

.el-main {
  --el-main-padding: 10px
}
</style>