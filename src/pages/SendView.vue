<script setup lang="ts">
import {reactive, ref} from "vue";

const option = reactive({
  transform_protocol: "UDP",
  port: 32500,
  ip: "0.0.0.0",
  protocol_type: "AOA",
  frequency: 0,
  protocol_id: '',
  mac: ''
})
let time = ref(0)
const formatTime = (time: number): string => {
  let date = new Date(time);
  let hour = date.getUTCHours();
  let minute = date.getMinutes();
  let second = date.getSeconds();
  return `${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}:${second.toString().padStart(2, '0')}`;

}
const run = async () => {
  if (option.protocol_id !== '') {
    // await stop()
  } else {
    // await start()
  }
}
</script>

<template>
<!--  <div style="display: flex;flex-direction: column;height: 100%">-->
<!--    <el-form class="demo-form-inline" label-position="right">-->
<!--      <el-row :gutter="20">-->
<!--        <el-col :span="4">-->
<!--          <el-form-item label="传输协议">-->
<!--            <el-select v-model="option.transform_protocol" placeholder="传输协议">-->
<!--              <el-option-->
<!--                  key="UDP"-->
<!--                  label="UDP"-->
<!--                  value="UDP"-->
<!--              />-->
<!--              <el-option-->
<!--                  key="TCP"-->
<!--                  label="TCP"-->
<!--                  value="TCP"-->
<!--              />-->
<!--            </el-select>-->
<!--          </el-form-item>-->
<!--        </el-col>-->
<!--        <el-col :span="4">-->
<!--          <el-form-item label="协议类型">-->
<!--            <el-select v-model="option.protocol_type" placeholder="协议类型">-->
<!--              <el-option-->
<!--                  key="AOA"-->
<!--                  label="AOA"-->
<!--                  value="AOA"-->
<!--              />-->
<!--            </el-select>-->
<!--          </el-form-item>-->
<!--        </el-col>-->
<!--        <el-col :span="4">-->
<!--          <el-form-item label="目标地址">-->
<!--            <el-input v-model="option.ip" placeholder="目标地址" >-->
<!--            </el-input>-->
<!--          </el-form-item>-->
<!--        </el-col>-->
<!--        <el-col :span="3">-->
<!--          <el-form-item label="端口">-->
<!--            <el-input v-model="option.port" placeholder="端口" oninput="value=value.replace(/[^\d.]/g,'')">-->
<!--            </el-input>-->
<!--          </el-form-item>-->
<!--        </el-col>-->
<!--        <el-col :span="3">-->
<!--          <el-form-item label="发包速率">-->
<!--            <el-input v-model="option.frequency" placeholder="请输入发包速率"-->
<!--                      oninput="value=value.replace(/[^\d.]/g,'')"/>-->
<!--          </el-form-item>-->
<!--        </el-col>-->
<!--        <el-col :span="3">-->
<!--          <el-form-item label="发包频率">-->
<!--            <el-input v-model="option.frequency" placeholder="请输入发包频率"-->
<!--                      oninput="value=value.replace(/[^\d.]/g,'')"/>-->
<!--          </el-form-item>-->
<!--        </el-col>-->
<!--        <el-col :span="3">-->
<!--          <el-row justify="space-evenly">-->
<!--            <el-button :type="option.protocol_id===''?'primary':'danger'" @click="run">-->
<!--              <span v-if="option.protocol_id===''">开始</span>-->
<!--              <span v-else>停止</span>-->
<!--            </el-button>-->
<!--            <el-text>{{ formatTime(time) }}</el-text>-->
<!--          </el-row>-->
<!--        </el-col>-->
<!--      </el-row>-->
<!--    </el-form>-->
<!--  </div>-->
  SendView.vue
</template>

<style scoped>

</style>