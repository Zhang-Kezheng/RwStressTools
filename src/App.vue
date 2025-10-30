<script setup lang="ts">
import {useRouter} from "vue-router";
import {Document, Download, Setting, Suitcase, Upload} from "@element-plus/icons-vue";
import { Window } from '@tauri-apps/api/window';
import {getCurrentInstance, onMounted, reactive, ref} from "vue";
import {ExitEnum,appConfig} from "./stores/LocalStorage";
import {defaultWindowIcon, hide, show} from "@tauri-apps/api/app";
import {attachConsole } from '@tauri-apps/plugin-log';
import { platform } from '@tauri-apps/plugin-os';
attachConsole()
const router = useRouter()
const menuRoutes=['/SendView','/ReceiveView','/HistoryView','/SettingView']
router.push(menuRoutes[0])
Window.getCurrent().onCloseRequested(async (event) => {
  if (appConfig.exit===ExitEnum.MIN){
    event.preventDefault();
    if (platform()==="windows"){
      await Window.getCurrent().hide()
    }else {
      await hide()
    }
  }
});
const tips=reactive({
  exit:appConfig.exit,
  notTips:false
})
window.addEventListener("unhandledrejection", event => {
  console.error(event)
  ElNotification({
    title: 'Error',
    message:  event.reason,
    type: 'error',
    position: 'bottom-right',
  })
});
const tipsConfirm=()=>{
  tipsOnExitShow.value=false
  appConfig.exit=tips.exit
  if (appConfig.exit==ExitEnum.EXIT){
    Window.getCurrent().destroy()
  }else {
     hide()
  }
}
const tipsOnExitShow=ref(false)
const handleSelect=(index:number)=>{
  router.push(menuRoutes[index])
}
</script>

<template>
  <el-container style="height:100vh" >
    <el-aside style="width: unset">
      <el-menu
          default-active="0"
          :collapse="true"
          style="height: 100%;"
          @select="handleSelect">
        <el-menu-item index="0">
          <el-icon><upload /></el-icon>
          <template #title>发送</template>
        </el-menu-item>
        <el-menu-item index="1" >
          <el-icon><download /></el-icon>
          <template #title>接收</template>
        </el-menu-item>
        <el-menu-item index="2" >
          <el-icon><document /></el-icon>
          <template #title>历史记录</template>
        </el-menu-item>
        <el-menu-item index="3">
          <el-icon><setting /></el-icon>
          <template #title>设置</template>
        </el-menu-item>
      </el-menu>
    </el-aside>
    <el-main>
        <RouterView v-slot="{Component}">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </RouterView>
    </el-main>
  </el-container>
  <el-dialog
      v-model="tipsOnExitShow"
      title="点击关闭按钮以后"
      center
      width="400">
    <div style="display: flex;flex-direction: column;align-items: center">
      <el-radio-group v-model="tips.exit">
        <el-radio :value="ExitEnum.MIN" style="display: block">最小化到系统托盘</el-radio>
        <el-radio :value="ExitEnum.EXIT" style="display: block">退出程序</el-radio>
      </el-radio-group>
      <el-checkbox v-model="tips.notTips" >不再提示</el-checkbox>
    </div>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="tipsOnExitShow = false">取消</el-button>
        <el-button type="primary" @click="tipsConfirm">
          确定
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style>

body{
  margin: 0;
}


</style>
