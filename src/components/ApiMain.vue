<script setup>
import {useTabStore} from "@/store/tab"
import {storeToRefs} from "pinia"
import ApiMainDetail from "@/components/ApiMainDetail.vue"
import {ref, watch} from "vue"

const tabStore = useTabStore()
const {tabs, activeTab} = storeToRefs(tabStore)
const {removeTab} = tabStore

const menuPosition = ref()
const selectedTab = ref()
const showMenu = ref(false)

// 从事件中获取tabId
const getTabId = event => {
  const target = event.target.closest('.el-tabs__item') // 获取当前标签元素
  if (target) {
    // 获取标签页唯一标识：tab-{name}，使用截取的方式，剔除“tab-”等4个字符
    return target.id.substring(4)
  }
}

// 右键菜单
const handleRightClick = event => {
  const tabId = getTabId(event)
  if (!tabId) {
    return
  }
  // 阻止标签页上的默认右键菜单
  event.preventDefault()
  const currentTab = tabs.value.find(tab => tab.name === tabId)

  // 设置菜单位置
  menuPosition.value = {left: event.clientX, top: event.clientY}
  selectedTab.value = currentTab
  showMenu.value = true
}

const handleMiddleClick = event => {
  const tabId = getTabId(event)
  if (!tabId) {
    return
  }
  removeTab(tabId)
}
// 关闭当前标签页
const closeTab = currentTab => {
  removeTab(currentTab.name)
}

// 关闭其他标签页
const closeOtherTabs = currentTab => {
  tabs.value.filter(tab => tab.name !== currentTab.name)
      .forEach(tab => removeTab(tab.name))
}

// 关闭当前标签页左侧所有标签页
const closeLeftTabs = currentTab => {
  const currentIndex = tabs.value.findIndex(t => t.name === currentTab.name)
  tabs.value.filter((_, index) => index < currentIndex)
      .forEach(tab => removeTab(tab.name))
}

// 关闭当前标签页右侧所有标签页
const closeRightTabs = currentTab => {
  const currentIndex = tabs.value.findIndex(t => t.name === currentTab.name)
  tabs.value.filter((_, index) => index > currentIndex)
      .forEach(tab => removeTab(tab.name))
}

// 关闭所有标签页
const closeAllTabs = () => {
  tabs.value.forEach(tab => removeTab(tab.name))
}

// 监听全局点击事件关闭菜单
watch(showMenu, visible => {
  if (visible) {
    document.addEventListener('click', () => showMenu.value = false, {once: true})
  }
})
</script>

<template>
  <div class="main-container">
    <el-tabs v-if="tabs.length>0" v-model="activeTab" closable type="card" @tab-remove="removeTab"
             @contextmenu="handleRightClick" @mousedown.middle="handleMiddleClick">
      <el-tab-pane v-for="(item, index) in tabs" :key="item.name" :label="item.label" :name="item.name">
        <api-main-detail :key="index" :path="item.name"/>
      </el-tab-pane>
    </el-tabs>
  </div>

  <teleport to="body">
    <div v-if="showMenu" class="context-menu" :style="{ left: menuPosition.left + 'px', top: menuPosition.top + 'px' }">
      <ul>
        <li @click="closeTab(selectedTab)">关闭</li>
        <li @click="closeOtherTabs(selectedTab)">关闭其他</li>
        <li @click="closeLeftTabs(selectedTab)">关闭左侧</li>
        <li @click="closeRightTabs(selectedTab)">关闭右侧</li>
        <li @click="closeAllTabs()">关闭全部</li>
      </ul>
    </div>
  </teleport>
</template>

<style scoped>
.main-container {
  height: 100%;
}

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
</style>
