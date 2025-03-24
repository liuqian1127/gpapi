<script setup>
const handleNodeClick = data => {
  console.log(data)
}

const data = [
  {
    label: "Default",
    children: []
  }
]

const defaultProps = {
  children: "children",
  label: "label",
}
</script>

<template>
  <el-tree :data="data" :props="defaultProps" class="aside-container" @node-click="handleNodeClick"/>
</template>

<style scoped>
.aside-container {
  height: 100%;
  max-width: 600px;
}
</style>
