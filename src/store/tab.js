import {defineStore} from "pinia"
import {ref} from "vue"

// 右侧主界面Tab标签页状态
export const useTabStore = defineStore("Tab", () => {
    const tabs = ref([])
    const activeTab = ref("")

    function addTab(tab) {
        const find = tabs.value.find(obj => obj.name === tab.name)
        if (!find) {
            tabs.value.push(tab)
        }
    }

    function removeTab(name) {
        const arr = tabs.value
        let activeName = activeTab.value
        if (activeName === name) {
            arr.forEach((tab, index) => {
                if (tab.name === name) {
                    const nextTab = arr[index + 1] || arr[index - 1]
                    if (nextTab) {
                        activeName = nextTab.name
                    }
                }
            })
        }

        activeTab.value = activeName
        tabs.value = arr.filter(tab => tab.name !== name)
    }

    function renameTab(oldLabel, newLabel) {
        for (const tab of tabs.value) {
            if (tab.label === oldLabel) {
                tab.label = newLabel
                tab.name = tab.name.replaceAll(oldLabel, newLabel)
                activeTab.value = tab.name
                break
            }
        }
    }

    // 根据标签名修改标签路径（name存放的path）
    function updateTab(label, name) {
        console.log(label, name)
        const found = tabs.value.find(obj => obj.label === label)
        if (found) {
            found.name = name
        }
    }

    return {tabs, activeTab, addTab, removeTab, renameTab, updateTab}
})
