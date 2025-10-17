<script setup>
import {useRouter} from "vue-router";
import {Document, Setting, Suitcase} from "@element-plus/icons-vue";


const router = useRouter()
const menuRoutes=['/SendView','/ReceiveView','/SettingView','/HistoryView'
]
router.push(menuRoutes[0])

const handleSelect=(index)=>{
  router.push(menuRoutes[index])
}
</script>

<template>
  <el-container style="height:100vh">
    <el-aside style="width: unset">
      <el-menu
          default-active="0"
          :collapse="true"
          style="height: 100%;"
          @select="handleSelect">
        <el-menu-item index="0">
          <el-icon><Suitcase /></el-icon>
          <template #title>发送</template>
        </el-menu-item>
        <el-menu-item index="1" >
          <el-icon><document /></el-icon>
          <template #title>接收</template>
        </el-menu-item>
        <el-menu-item index="2">
          <el-icon><setting /></el-icon>
          <template #title>设置</template>
        </el-menu-item>
      </el-menu>
    </el-aside>
    <el-main>
      <keep-alive>
        <RouterView v-slot="{Component}">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </RouterView>
      </keep-alive>
    </el-main>
  </el-container>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body{
  margin: 0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

</style>
