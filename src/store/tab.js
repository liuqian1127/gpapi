import {defineStore} from 'pinia'
import {ref} from "vue"

// 右侧主界面Tab标签页状态
export const useTabStore = defineStore('Tab', () => {
    const tabs = ref([])
    const currentTab = ref('')

    function addTab(tab) {
        const find = tabs.value.find(obj => obj.label === tab.label)
        if (!find) {
            tabs.value.push(tab)
        }
    }

    function removeTab(name) {
        const arr = tabs.value
        let activeName = currentTab.value
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

        currentTab.value = activeName
        tabs.value = arr.filter(tab => tab.name !== name)
    }

    return {tabs, currentTab, addTab, removeTab}
})
