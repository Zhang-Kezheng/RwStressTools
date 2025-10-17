<script setup >
import {onMounted,onActivated,onDeactivated, reactive, ref, onUnmounted} from 'vue'
import { invoke } from "@tauri-apps/api/core";
import ipaddr from 'ipaddr.js'
import {bind, send, unbind} from "@kuyoonjo/tauri-plugin-udp";
import { listen } from "@tauri-apps/api/event";
import {AoA} from "../protocol/AoA.js"
// 全局错误捕获（浏览器端）
window.addEventListener('unhandledrejection', function(event) {
  console.error(event)
});
const network_interfaces=async ()=>{
  await invoke("network_interfaces").then((result)=>{
    result.map((item)=>{
      return {
        name:item[0],
        ipaddr:ipaddr.parse(item[1])
      }
    }).filter((item)=>{
      return item.ipaddr.kind()==="ipv4"
    }).forEach((item)=>{
      ipaddrList.push(item)
    })
    ipaddrList.push({name:"all",ipaddr:ipaddr.parse("0.0.0.0")})
  },(error)=>{
    throw error
  })
}
onMounted(async () => {
  await network_interfaces()
  await listen("plugin://udp", (x) => {
    if (x.payload.id!==protocol_id.value)return;
    let aoa=AoA.getInstance(x.payload.data)
    if (aoa!=null){
      console.log(aoa)
    }
  });
});
onUnmounted(async () => {
  if (protocol_id.value!=='') {
    await unbind(protocol_id.value)
    protocol_id.value = ''
  }
})
const run = async () => {
  if (protocol_id.value!=='') {
    await unbind(protocol_id.value)
    protocol_id.value = ''
  } else {
    protocol_id.value=receive.ip+":"+receive.port
    await bind(protocol_id.value, protocol_id.value)
  }
}
const ipaddrList = reactive([])
const receive=reactive({
  transform_protocol:"UDP",
  port:32500,
  ip:"0.0.0.0",
  protocol_type:"AOA",
  frequency:1
})
let protocol_id = ref('');
</script>

<template>
  <el-row>
    <el-form  class="demo-form-inline" label-position="right">
      <el-row :gutter="20">
        <el-col :span="5">
          <el-form-item label="传输协议" v-model="receive">
            <el-select v-model="receive.transform_protocol" placeholder="传输协议" >
              <el-option
                  key="UDP"
                  label="UDP"
                  value="UDP"
              />
              <el-option
                  key="TCP"
                  label="TCP"
                  value="TCP"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="5">
          <el-form-item label="协议类型"  >
            <el-select v-model="receive.protocol_type" placeholder="协议类型" >
              <el-option
                  key="AOA"
                  label="AOA"
                  value="AOA"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="5">
          <el-form-item label="本机地址"  >
            <el-select v-model="receive.ip" placeholder="本机地址" >
              <el-option
                  v-for="ipaddr in ipaddrList"
                  :key="ipaddr.name"
                  :label="ipaddr.ipaddr.toString()"
                  :value="ipaddr.ipaddr.toString()"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="5">
          <el-form-item label="端口"  >
            <el-input v-model="receive.port" placeholder="端口" oninput="value=value.replace(/[^\d.]/g,'')">
            </el-input>
          </el-form-item>
        </el-col>
        <el-col :span="4">
          <el-button @click="run">
            <span v-if="protocol_id===''">开始</span>
            <span v-else>停止</span>
          </el-button>
        </el-col>
      </el-row>
    </el-form>
    <el-table :data="tableData" style="width: 100%" border>
      <el-table-column prop="date" label="Mac"  align="center"/>
      <el-table-column prop="name" label="电压" align="center"/>
      <el-table-column prop="address" label="防拆" align="center" min-width="60"/>
      <el-table-column prop="address" label="按钮" align="center"/>
      <el-table-column prop="address" label="振动" align="center"/>
      <el-table-column prop="address" label="心率" align="center"/>
      <el-table-column prop="address" label="收缩压" align="center"/>
      <el-table-column prop="address" label="舒张压" align="center"/>
      <el-table-column prop="address" label="血氧" align="center"/>
      <el-table-column prop="address" label="体温" align="center"/>
      <el-table-column prop="address" label="计步" align="center"/>
      <el-table-column prop="address" label="睡眠状态" align="center"/>
      <el-table-column prop="address" label="深睡眠时间" align="center"/>
      <el-table-column prop="address" label="浅睡眠时间" align="center"/>
      <el-table-column prop="address" label="rssi" align="center"/>
    </el-table>
  </el-row>
</template>

<style scoped>

.demo-form-inline .el-select {
  --el-select-width: 220px;
}
.demo-form-inline .el-input {
  --el-input-width: 220px;
}
.el-table .cell {
  padding: unset;
}
</style>