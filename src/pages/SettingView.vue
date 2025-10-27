<script setup lang="ts">
import {appConfig, ExitEnum, ThemeEnum} from "../stores/LocalStorage"
import {setTheme} from "@tauri-apps/api/app";
import {open} from "@tauri-apps/plugin-dialog";
import {Folder} from "@element-plus/icons-vue";

const themeChange=()=>{
  if (appConfig.theme==ThemeEnum.DARK){
    setTheme('dark')
  }else if (appConfig.theme==ThemeEnum.LIGHT) {
    setTheme('light')
  }else {
    setTheme(null)
  }
}

const selectDir= async ()=>{
  let dir=await open({
    directory: true, // 指定为选择文件夹
    multiple: false // 是否允许多选
  })
  if (dir==null){
    return
  }
  appConfig.exportDir=dir
}
</script>

<template>
  <h4>常规设置</h4>
  <el-form>
    <el-form-item label="主题颜色:" label-position="top">
      <el-row style="width:100%;">
        <el-col :span="4">
          <el-select v-model="appConfig.theme" placeholder="主题颜色" @change="themeChange">
            <el-option
                key="UDP"
                label="跟随系统"
                :value="ThemeEnum.SYSTEM"
            />
            <el-option
                key="TCP"
                label="深色模式"
                :value="ThemeEnum.DARK"
            />
            <el-option
                key="TCP"
                label="浅色模式"
                :value="ThemeEnum.LIGHT"
            />
          </el-select>
        </el-col>
      </el-row>
    </el-form-item>
    <el-form-item label="关闭主界面时:" label-position="top">
      <div style="display: flex;flex-direction: column">
        <el-radio-group v-model="appConfig.exit">
          <el-radio :value="ExitEnum.MIN">最小化到系统托盘</el-radio>
          <el-radio :value="ExitEnum.EXIT">退出程序</el-radio>
        </el-radio-group>
<!--        <el-checkbox v-model="appConfig.tipsOnExit" >退出时提示</el-checkbox>-->
      </div>
    </el-form-item>
    <el-form-item label="导出目录:" label-position="top">
      <el-row style="width:100%;">
        <el-col :span="6">
          <el-input
              readonly
              v-model="appConfig.exportDir">
            <template #append>
              <el-button :icon="Folder"  @click="selectDir"/>
            </template>
          </el-input>
        </el-col>
      </el-row>
    </el-form-item>
  </el-form>
</template>

<style scoped>

</style>