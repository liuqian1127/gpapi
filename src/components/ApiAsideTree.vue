<script setup>
import {useTabStore} from "@/store/tab.js"
import {storeToRefs} from "pinia"
import {ref, watch} from "vue"
import {invoke} from "@tauri-apps/api/core"
import {ElMessage} from "element-plus"

const {rootPath, treeData} = defineProps(["rootPath", "treeData"])
const emit = defineEmits(["refresh"])

const showMenu = ref(false)
const menuPosition = ref({top: 0, left: 0})
const activeNode = ref()

const tree = ref()

const newName = ref("")
const dialogFormVisible = ref(false)

const tabStore = useTabStore()
const {activeTab} = storeToRefs(tabStore)
const {addTab, removeTab, renameTab, updateTab} = tabStore

// 点击文件树节点
const handleNodeClick = data => {
  closeMenu()

  if (data.isDir) {
    return
  }

  const tab = {
    name: data.path,
    label: data.label
  }
  addTab(tab)

  activeTab.value = data.path
}

// 文件树右键菜单
const handleContextMenu = (event, data) => {
  showMenu.value = true
  menuPosition.value = {top: event.clientY, left: event.clientX}
  activeNode.value = data
}

// 关闭菜单
const closeMenu = () => showMenu.value = false

// 新增目录
const addDir = () => {
  closeMenu()

  const path = activeNode.value.path + "/新目录"
  const node = {
    id: Date.now(),
    label: "新目录",
    path,
    children: [],
    isDir: true
  }

  // 新建目录
  invoke("mkdir", {path})
      .then(resp => {
        if (resp === "exist") {
          ElMessage.success("目录已存在")
        } else {
          tree.value.append(node, activeNode.value)
        }
      })
      .catch(err => ElMessage.error(err))
}

// 新增文件
const addFile = () => {
  closeMenu()

  const path = activeNode.value.path + "/新文件.json"
  const node = {
    id: Date.now(),
    label: "新文件",
    path,
    children: [],
    isDir: false
  }

  // 新建文件
  invoke("touch", {path})
      .then(resp => {
        if (resp === "exist") {
          ElMessage.success("文件已存在")
        } else {
          tree.value.append(node, activeNode.value)
        }
      })
      .catch(err => ElMessage.error(err))
}

// 复制节点
const copy = () => {
  closeMenu()

  // 文件树复制 path是文件名
  const path = activeNode.value.path

  const pathParts = path.split(/[\\/]/)
  const dirPath = pathParts.slice(0, -1).join("/")
  const fileName = pathParts[pathParts.length - 1]
  const label = fileName.replaceAll("\.json", "") + " Copy"
  const newPath = dirPath + "/" + label + ".json"

  const node = {
    id: Date.now(),
    label,
    path: newPath,
    children: [],
    isDir: false
  }

  // 新建文件
  invoke("copy", {from: path, to: newPath})
      .then(resp => {
        if (resp === "exist") {
          ElMessage.success("文件已存在")
        } else {
          tree.value.insertAfter(node, activeNode.value)
        }
      })
      .catch(err => ElMessage.error(err))
}

// 删除节点
const remove = () => {
  // 关闭菜单
  closeMenu()

  // 文件路径，具有唯一性
  const path = activeNode.value.path

  // 删除底层文件
  invoke("remove", {path})
      .then(_ => {
        // 删除标签页
        removeTab(path)

        // 删除文件树节点
        tree.value.remove(activeNode.value)
      })
      .catch(err => ElMessage.error(err))
}

// 重命名
const rename = () => {
  // 非空时中止修改
  const name = newName.value
  if (name === "") {
    return
  }

  const fromPath = activeNode.value.path
  const label = activeNode.value.label
  const toPath = fromPath.replaceAll(label, name)

  // 文件系统
  invoke("rename", {from: fromPath, to: toPath})
      .then(resp => {
        if (resp === "exist") {
          ElMessage.error("不能重名")
        } else {
          // 文件树
          activeNode.value.path = toPath
          activeNode.value.label = name

          // 标签页
          if (!activeNode.isDir) {
            renameTab(label, name)
          }

          dialogFormVisible.value = false
        }
      })
      .catch(err => ElMessage.error(err))
}

// 打开重命名对话框
const showRenameDialog = () => {
  newName.value = activeNode.value.label
  dialogFormVisible.value = true
}

// 监听全局点击事件关闭菜单
watch(showMenu, visible => {
  if (visible) {
    document.addEventListener('click', () => showMenu.value = false, {once: true})
  }
})

// 拖拽节点时变更文件系统
const handleDrop = (draggingNode, dropNode) => {
  const label = draggingNode.data.label
  const fromPath = draggingNode.data.path
  const toPath = dropNode.data.path + "/" + label + ".json"

  // 文件系统
  invoke("rename", {from: fromPath, to: toPath})
      .then(resp => {
        if (resp === "exist") {
          ElMessage.error("不能重名")
        } else {
          // 更新标签页
          updateTab(label, toPath)
          activeTab.value = toPath

          // 更新文件树
          draggingNode.data.path = toPath
        }
      })
      .catch(err => ElMessage.error(err))
}
// 仅目录节点可拖入
const allowDrop = (draggingNode, dropNode, type) => {
  return dropNode.data.isDir && type === 'inner'
}
// 仅允许拖拽文件节点
const allowDrag = draggingNode => {
  return !draggingNode.data.isDir
}
</script>

<template>
  <el-tree ref="tree" :data="treeData" empty-text="无数据" node-key="id" highlight-current
           :draggable="true" :allow-drop="allowDrop" :allow-drag="allowDrag" @node-drop="handleDrop"
           @node-click="handleNodeClick" @node-contextmenu="handleContextMenu">
    <template #default="{node, data}">
      <el-text :type="data.isDir?'primary':'success'">{{ node.label }}</el-text>
    </template>
  </el-tree>

  <teleport to="body">
    <div v-if="showMenu" class="context-menu" :style="{ left: menuPosition.left + 'px', top: menuPosition.top + 'px' }">
      <ul>
        <li v-if="activeNode.isDir" @click="addDir">新增目录</li>
        <li v-if="activeNode.isDir" @click="addFile">新增文件</li>
        <li v-if="!activeNode.isDir" @click="copy">复制</li>
        <li v-if="!activeNode.isRoot && (activeNode.isDir && activeNode.children.length===0 || !activeNode.isDir)" @click="remove">删除</li>
        <li v-if="!activeNode.isRoot && activeNode.children.length===0" @click="showRenameDialog">重命名</li>
      </ul>
    </div>
  </teleport>

  <el-dialog v-model="dialogFormVisible" title="重命名" width="300">
    <el-input v-model="newName" @keyup.enter="rename"/>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogFormVisible=false">取消</el-button>
        <el-button type="primary" @click="rename">确定</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped>
.context-menu {
  position: fixed;
  background: #fff;
  border: 1px solid #ebeef5;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  z-index: 9999;

  ul {
    list-style: none;
    padding: 5px 0;
    margin: 0;

    li {
      padding: 5px 15px;
      cursor: pointer;

      &:hover {
        background: #e7e8ea;
      }
    }
  }
}

.el-menu-item {
  height: 30px;
  margin-top: 5px;
  margin-bottom: 5px;
}
</style>
