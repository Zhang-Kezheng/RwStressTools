<script setup lang="ts">
import {reactive, onMounted, ref} from "vue";
import {dataBase, TagModel} from "../db";

const tableData = reactive([])
onMounted(async ()=>{
  query()
  let result=await dataBase.select("select count(1) from TagModel")
  console.log(result[0]["count(1)"])
  page.total=result[0]["count(1)"]
})
const pageChange=async (pageIndex:number)=>{
  page.pageIndex=pageIndex
  query()
}
const query= ()=>{
  loading.value=true
  dataBase.select("select * from TagModel limit $1,$2",[(page.pageIndex-1)*page.pageSize,page.pageSize]).then(res=>{
    const result=res as TagModel[]
    console.log(result)
    result.forEach(item=>{
      tableData.push(item)
    })
    loading.value=false
  })
}
const page=reactive({
  pageSize:50,
  pageIndex:1,
  total:0,
})
const loading=ref(false)
</script>

<template>
  <div style="display: flex;flex-direction: column;height: 100%">
    HistoryView.vue
<!--    <el-table v-loading="loading" :data="tableData" style="width: 100%;flex: 1" border>-->
<!--      <el-table-column prop="mac" label="Mac"  align="center" min-width="150"></el-table-column>-->
<!--      <el-table-column prop="name" label="电压" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="防拆" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="按钮" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="振动" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="心率" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="收缩压" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="舒张压" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="血氧" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="体温" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="计步" align="center" min-width="70"/>-->
<!--      <el-table-column prop="address" label="睡眠状态" align="center" min-width="100"/>-->
<!--      <el-table-column prop="address" label="深睡眠时间" align="center" min-width="100"/>-->
<!--      <el-table-column prop="address" label="浅睡眠时间" align="center" min-width="100" />-->
<!--      <el-table-column prop="address" label="rssi" align="center" min-width="100"/>-->
<!--      <el-table-column prop="deviceID" label="网关ID" align="center" min-width="100"/>-->
<!--    </el-table>-->
<!--    <el-row justify="center" style="margin-top: 10px">-->
<!--      <el-pagination background layout="prev, pager, next"-->
<!--                     :total="page.total"-->
<!--                     :page-size="page.pageSize"-->
<!--                     :current-page="page.pageIndex"-->
<!--                     @update:current-page="pageChange"-->
<!--      />-->
<!--    </el-row>-->
  </div>
</template>

<style scoped>

</style>