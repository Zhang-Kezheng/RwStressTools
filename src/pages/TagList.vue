<script setup lang="ts">
import {defineProps, onMounted, onUnmounted, reactive, ref, watch} from 'vue';
import {invoke} from "@tauri-apps/api/core";
import {AoaTagDto, PageResponse} from "../vo/vo";
const props = defineProps({
  gatewayMac: {
    type: String,
    required: true
  },
  run_time: {
    type: Number,
    required: true
  },
  frequency: {
    type: Number,
    required: true
  },
})
const tagList=ref(new Array<AoaTagDto>())
const tag_mac=ref("")
const option= reactive({
  tag_mac:""
})
function fetchTagList() {
  invoke<PageResponse<AoaTagDto>>("fetch_tag_list",{gatewayMac:props.gatewayMac,index:page.pageIndex,size:page.pageSize,tagMac:option.tag_mac}).then(res =>{
    page.total=res.total;
    tagList.value=res.data;
  })
}
let timer=null
onMounted(()=>{
  fetchTagList();
  timer=setInterval(() => {
    fetchTagList();
  },1000)
})
onUnmounted(()=>{
  if (timer) clearInterval(timer)
})
const page=reactive({
  pageSize:20,
  pageIndex:1,
  total:0
})
import Decimal from 'decimal.js';
const diubaolv=(packet_count:number):string=>{
  let yingshou:number=Number(props.frequency)*props.run_time/1000
  if (yingshou==0){
    return "0"
  }else {
    return Number(Decimal(yingshou-packet_count).div(yingshou).mul(100).toFixed(2)).toString()+"%"
  }
}
const pageChange=(pageIndex:number)=>{
  page.pageIndex=pageIndex
  fetchTagList();
}
const exportMode=ref(0);
const centerDialogVisible =ref(false);
const exportTagList=()=>{
  centerDialogVisible.value=true
}
function timestampToTime(timestamp) {
  const date = new Date(timestamp);//时间戳为10位需*1000，时间戳为13位的话不需乘1000
  const Y = date.getFullYear() + '-';
  const M = (date.getMonth()+1 < 10 ? '0'+(date.getMonth()+1):date.getMonth()+1) + '-';
  const D = (date.getDate()< 10 ? '0'+date.getDate():date.getDate())+ ' ';
  const h = (date.getHours() < 10 ? '0'+date.getHours():date.getHours())+ ':';
  const m = (date.getMinutes() < 10 ? '0'+date.getMinutes():date.getMinutes()) + ':';
  const s = date.getSeconds() < 10 ? '0'+date.getSeconds():date.getSeconds();
  return Y+M+D+h+m+s;
}

import {BaseDirectory, resolve} from "@tauri-apps/api/path";
import {appConfig} from "../stores/LocalStorage";
import {writeTextFile} from "@tauri-apps/plugin-fs";
const exportConfirm= async ()=>{
  centerDialogVisible.value = false
  // await writeTextFile(`tag_list_${new Date().getTime()}.csv`, "Mac,电压,防拆,按钮,振动,心率,舒张压,收缩压,时间", {
  //   baseDir: BaseDirectory.AppLocalData,
  // });

  await invoke("export_tag_list",{exportMode:exportMode.value,exportPath:await resolve(appConfig.exportDir,`tag_list_${props.gatewayMac}_${new Date().getTime()}.csv`),gatewayMac:props.gatewayMac,tagMac:option.tag_mac,yingshou:Number(props.frequency)*props.run_time/1000})
  ElMessage({
      message: "导出成功",
      type:"success",
      placement:"bottom"
    })
}
</script>

<template>
  <div style="display: flex; flex-direction: column;height: 100%">
    <el-row :gutter="20">
      <el-col :span="19">
        <el-form-item label="Mac地址">
          <el-input v-model="tag_mac" placeholder="请输入Mac地址"/>
        </el-form-item>
      </el-col>
      <el-col :span="5">
        <el-button type="primary" @click="option.tag_mac=tag_mac;fetchTagList();">
          搜索
        </el-button>
        <el-button @click="option.tag_mac='';tag_mac='';fetchTagList(); ">
          重置
        </el-button>
        <el-button @click="exportTagList ">
          导出
        </el-button>
      </el-col>
    </el-row>
    <el-table :data="tagList"
              style="width: 100%;flex: 1" border>
      <el-table-column prop="mac" label="Mac"  align="center"  min-width="130"/>
      <el-table-column prop="voltage" label="电压" align="center" min-width="70"/>
      <el-table-column prop="tamper" label="防拆" align="center" min-width="70"/>
      <el-table-column prop="button" label="按钮" align="center" min-width="70"/>
      <el-table-column prop="shock" label="振动" align="center" min-width="70"/>
      <el-table-column prop="heart_rate" label="心率" align="center" min-width="70"/>
      <el-table-column prop="blood_pressure_h" label="收缩压" align="center" min-width="70"/>
      <el-table-column prop="blood_pressure_l" label="舒张压" align="center" min-width="70"/>
      <el-table-column prop="blood_oxygen" label="血氧" align="center" min-width="70"/>
      <el-table-column prop="body_temperature" label="体温" align="center" min-width="70"/>
      <el-table-column prop="step_count" label="计步" align="center" min-width="70"/>
      <el-table-column prop="sleep_state" label="睡眠状态" align="center" min-width="100"/>
      <el-table-column prop="deep_sleep_time" label="深睡眠时间" align="center" min-width="100"/>
      <el-table-column prop="light_sleep_time" label="浅睡眠时间" align="center" min-width="100" />
      <el-table-column prop="rssi" label="rssi" align="center" min-width="100"/>
      <el-table-column prop="first_time" label="首次上报时间" align="center" min-width="150">
        <template #default="scope">
          {{timestampToTime(scope.row.first_time)}}
        </template>
      </el-table-column>
      <el-table-column prop="packet_loss_rate" label="丢包率" align="center" min-width="100">
        <template #default="scope">
          {{
          diubaolv(scope.row.packet_count)
          }}
        </template>
      </el-table-column>
      <el-table-column prop="last_time" label="最新时间" align="center" min-width="150">
        <template #default="scope">
          {{timestampToTime(scope.row.last_time)}}
        </template>
      </el-table-column>
    </el-table>
    <el-row justify="center" style="margin-top: 10px">
      <el-pagination background layout="prev, pager, next"
                     :total="page.total"
                     :hide-on-single-page="true"
                     :page-size="page.pageSize"
                     :current-page="page.pageIndex"
                     @update:current-page="pageChange"
      />
    </el-row>
    <el-dialog v-model="centerDialogVisible" title="提示" width="300" center align-center>
      <el-row justify="center">
        <el-radio-group v-model="exportMode" >
          <el-radio :value="0" style="display: block">不合并导出</el-radio>
          <el-radio :value="1" style="display: block">合并导出</el-radio>
        </el-radio-group>
      </el-row>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="centerDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="exportConfirm">
            确定
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>

</style>