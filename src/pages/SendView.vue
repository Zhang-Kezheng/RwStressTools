<script setup lang="ts">
import {onUnmounted, reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const option = reactive({
  transform_protocol: "UDP",
  port: 32500,
  ip: "127.0.0.1",
  protocol_type: "AOA",
  rate: 1000,
  protocol_id: '',
  thread_count: 1,
  mac: ''
})
const formatTime = (time: number): string => {
  let date = new Date(time);
  let hour = date.getUTCHours();
  let minute = date.getMinutes();
  let second = date.getSeconds();
  return `${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}:${second.toString().padStart(2, '0')}`;

}
const run = async () => {
  if (option.protocol_id !== '') {
    await stop()
  } else {
    await start()
  }
}

const run_time=ref(0)
let interval_id=null

const logs=reactive([])
async function stop() {
  option.protocol_id='';
  await invoke("send_stop");
  if (interval_id)clearInterval(interval_id);
}
onUnmounted(()=>{
  if (interval_id)clearInterval(interval_id);
})
async function start() {
  run_time.value=0
  option.protocol_id=option.ip+":"+option.port;
  logs.push(`${format_date(new Date())} 启动`)
  await invoke("send_start",{target:option.protocol_id,threadCount:Number(option.thread_count),rate:Number(option.rate)});
  interval_id=Number(setInterval(()=>{
        run_time.value+=1000;
      },1000)
  )
  logs.push(`${format_date(new Date())} 启动成功`)
}
function format_date(now:Date):string{
  const year = now.getFullYear();
  const month = ('0' + (now.getMonth() + 1)).slice(-2);
  const day = ('0' + now.getDate()).slice(-2);
  const hours = ('0' + now.getHours()).slice(-2);
  const minutes = ('0' + now.getMinutes()).slice(-2);
  const seconds = ('0' + now.getSeconds()).slice(-2);

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}` ;
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
        <el-col :span="4">
          <el-form-item label="协议类型">
            <el-select v-model="option.protocol_type" placeholder="协议类型">
              <el-option
                  key="AOA"
                  label="AOA"
                  value="AOA"
              />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="4">
          <el-form-item label="目标地址">
            <el-input v-model="option.ip" placeholder="目标地址" >
            </el-input>
          </el-form-item>
        </el-col>
        <el-col :span="3">
          <el-form-item label="端口">
            <el-input v-model="option.port" placeholder="端口" oninput="value=value.replace(/[^\d.]/g,'')">
            </el-input>
          </el-form-item>
        </el-col>
        <el-col :span="3">
          <el-form-item label="发包频率">
            <el-input v-model="option.rate" placeholder="请输入发包频率"
                      oninput="value=value.replace(/[^\d.]/g,'')"/>
          </el-form-item>
        </el-col>
        <el-col :span="3">
          <el-form-item label="线程数">
            <el-input v-model="option.thread_count" placeholder="请输入线程数"
                      oninput="value=value.replace(/[^\d.]/g,'')"/>
          </el-form-item>
        </el-col>
        <el-col :span="3">
          <el-row justify="space-evenly">
            <el-button :type="option.protocol_id===''?'primary':'danger'" @click="run">
              <span v-if="option.protocol_id===''">开始</span>
              <span v-else>停止</span>
            </el-button>
            <el-text>{{ formatTime(run_time) }}</el-text>
          </el-row>
        </el-col>
      </el-row>
    </el-form>
    <el-card style="flex: 1;display: flex;flex-direction: column" body-class="el-body">
      <template #header>
        <div class="card-header">
          <span>操作日志</span>
        </div>
      </template>
      <el-scrollbar height="400px">
        <p v-for="log in logs"  class="text item">{{log }}</p>
      </el-scrollbar>
    </el-card>
  </div>
</template>

<style scoped>
.el-body{
  flex: 1;
  overflow: auto;
}
</style>