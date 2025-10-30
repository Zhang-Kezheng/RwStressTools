<script setup lang="ts">
import {nextTick, onMounted, onUnmounted, Reactive, reactive, Ref, ref} from 'vue'
import {invoke} from "@tauri-apps/api/core";
import * as ip from 'ip';
import TagList from "./TagList.vue";
type Ipaddr = {
  name: string,
  ipaddr: string
}
const network_interfaces = async () => {
  await invoke<string[][]>("network_interfaces").then((result) => {
    result.map((item) => {
      const ipaddr: Ipaddr = {
        name: item[0],
        ipaddr: item[1],
      }
      return ipaddr
    }).filter((item) => {
      return ip.isV4Format(item.ipaddr)&&is_physical_interface(item.name)
    }).forEach((item) => {
      ipaddrList.push(item)
    })
    ipaddrList.push({name: "loopback", ipaddr: "127.0.0.1"})
    ipaddrList.push({name: "all", ipaddr: "0.0.0.0"})
  }, (error) => {
    throw error
  })
}
function is_physical_interface(name: string){
  return name.startsWith("en") &&
  !name.startsWith("bridge") &&
  !name.startsWith("utun") &&
  !name.startsWith("lima")
}

import {AoaGatewayVo, AoaTagVo, PageResponse} from "../vo/vo";
let gatewayTableList = ref([])
onMounted(async () => {
  await network_interfaces()
});
onUnmounted(async () => {
  if (option.running) {
    await stop()
  }
})
async function start() {
  invoke("receive_start", {protocol: option.transform_protocol, ip: option.ip, port: Number(option.port)}).then(res=>{
    time.value = 0
    option.running =true
    fetchGateway()
    timer = setInterval(() => {
      time.value += 1000
      fetchGateway()
    }, 1000)
  })
}

let time = ref(0)
let timer: NodeJS.Timeout = null

async function stop() {
  clearInterval(timer)
  await invoke("receive_stop")
  option.running = false
}

const run = async () => {
  if (option.running) {
    await stop()
  } else {
    await start()
  }
}
const pageChange = (pageIndex: number) => {
  page.pageIndex = pageIndex
}
const page = reactive({
  pageSize: 100,
  pageIndex: 1,
  total: 0
})


function fetchGateway() {
  invoke<PageResponse<AoaGatewayVo>>("fetch_gateway", {index: page.pageIndex, size: page.pageSize,mac:option.mac}).then((result) => {
    gatewayTableList.value = result.data
    page.total = result.total;
  })
}

const formatTime = (time: number): string => {
  let date = new Date(time);
  let hour = date.getUTCHours();
  let minute = date.getMinutes();
  let second = date.getSeconds();
  return `${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}:${second.toString().padStart(2, '0')}`;

}
const ipaddrList = reactive(new Array<Ipaddr>())
const option = reactive({
  transform_protocol: 0,
  port: 32500,
  ip: "0.0.0.0",
  protocol_type: "IOT_BOX",
  frequency: 0,
  running: false,
  mac: ''
})
const mac = ref('')
const tagsDetailShow = ref(false)
let selectedGatewayMac = ref("")
const diubaolv=(row:AoaGatewayVo)=>{
  const yingshou=time.value*option.frequency*row.tag_count/1000
  if (yingshou===0){
    return "0%"
  }
  return ((yingshou-row.total)*100/yingshou).toFixed(2)+"%"
}
</script>

<template>
  <div style="display: flex;flex-direction: column;height: 100%">

    <el-form class="demo-form-inline" label-position="right">
      <el-row :gutter="20">
        <el-col :span="4">
          <el-form-item label="传输协议">
            <el-select v-model="option.transform_protocol" placeholder="传输协议">
              <el-option
                  key="UDP"
                  label="UDP"
                  :value="0"
              />
              <el-option
                  key="TCP"
                  label="TCP"
                  :value="1"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="4">
          <el-form-item label="协议类型">
            <el-select v-model="option.protocol_type" placeholder="协议类型">
              <el-option
                  key="IOT_BOX"
                  label="IOT_BOX"
                  value="IOT_BOX"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="5">
          <el-form-item label="本机地址">
            <el-select v-model="option.ip" placeholder="本机地址">
              <el-option
                  v-for="ipaddr in ipaddrList"
                  :key="ipaddr.name"
                  :label="ipaddr.ipaddr"
                  :value="ipaddr.ipaddr"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="4">
          <el-form-item label="端口">
            <el-input v-model="option.port" placeholder="端口" oninput="value=value.replace(/[^\d.]/g,'')">
            </el-input>
          </el-form-item>
        </el-col>
        <el-col :span="4">
          <el-form-item label="发包频率">
            <el-input v-model="option.frequency" placeholder="请输入发包频率"
                      oninput="value=value.replace(/[^\d.]/g,'')"/>
          </el-form-item>
        </el-col>
        <el-col :span="3">
          <el-row justify="space-evenly">
            <el-button :type="option.running?'danger':'primary'" @click="run">
              <span v-if="option.running">停止</span>
              <span v-else>开始</span>
            </el-button>
            <el-text>{{ formatTime(time) }}</el-text>
          </el-row>
        </el-col>
      </el-row>
      <el-row :gutter="20">
        <el-col :span="20">
          <el-form-item label="Mac地址">
            <el-input v-model="mac" placeholder="请输入Mac地址"/>
          </el-form-item>
        </el-col>
        <el-col :span="4">
          <el-button type="primary" @click="option.mac=mac">
            搜索
          </el-button>
          <el-button @click="option.mac='';mac='' ">
            重置
          </el-button>
        </el-col>
      </el-row>
    </el-form>
    <el-table border :data="gatewayTableList"
              style="width: 100%;flex: 1">
      <el-table-column label="网关Mac" align="center" prop="mac"/>
      <el-table-column label="总包数" align="center" prop="total">
        <template #default="scope">
          {{ scope.row.total }}
        </template>
      </el-table-column>
      <el-table-column label="丢包率" align="center" prop="packet_loss_rate">
        <template #default="scope">
          {{diubaolv(scope.row)}}
        </template>
      </el-table-column>
      <el-table-column label="收包速率(Byte/S)" align="center" prop="packet_receive_rate">
      </el-table-column>
      <el-table-column label="标签数" align="center">
        <template #default="scope">
          <el-link type="primary" @click="tagsDetailShow=true;selectedGatewayMac=scope.row.mac">
            {{ scope.row.tag_count }}
          </el-link>
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
  </div>
  <el-drawer v-model="tagsDetailShow" :with-header="false" :size="'80%'" :destroy-on-close="true" resizable>
    <TagList :gateway-mac="selectedGatewayMac" :frequency="option.frequency" :run_time="time"/>
  </el-drawer>
</template>

<style scoped>

:deep() .el-table .cell {
  padding: unset;
}
</style>

<style>
</style>