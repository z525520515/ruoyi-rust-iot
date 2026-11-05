<template>
   <div class="app-container">
      <el-alert
         v-if="isDemo"
         type="info"
         :closable="false"
         title="当前为前端 Mock 演示模式：数据来自浏览器内存，刷新后重置，不连接后端。"
         class="mb8"
      />
      <el-row :gutter="20" class="stat-row">
         <el-col :span="8">
            <el-card shadow="hover" class="stat-card">
               <div class="stat-title">设备总数</div>
               <div class="stat-value">{{ stats.total }}</div>
            </el-card>
         </el-col>
         <el-col :span="8">
            <el-card shadow="hover" class="stat-card stat-online">
               <div class="stat-title">在线设备</div>
               <div class="stat-value">{{ stats.online }}</div>
            </el-card>
         </el-col>
         <el-col :span="8">
            <el-card shadow="hover" class="stat-card stat-offline">
               <div class="stat-title">离线设备</div>
               <div class="stat-value">{{ stats.offline }}</div>
            </el-card>
         </el-col>
      </el-row>

      <el-row :gutter="20">
         <!--部门数据-->
         <el-col :span="4" :xs="24">
            <div class="head-container">
               <el-input
                  v-model="deptName"
                  placeholder="请输入部门名称"
                  clearable
                  prefix-icon="Search"
                  style="margin-bottom: 20px"
               />
            </div>
            <div class="head-container">
               <el-tree
                  :data="deptOptions"
                  :props="{ label: 'label', children: 'children' }"
                  :expand-on-click-node="false"
                  :filter-node-method="filterNode"
                  ref="deptTreeRef"
                  node-key="id"
                  highlight-current
                  default-expand-all
                  @node-click="handleNodeClick"
               />
            </div>
         </el-col>
         <!--设备数据-->
         <el-col :span="20" :xs="24">
            <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="80px">
               <el-form-item label="设备编号" prop="equipmentNumber">
                  <el-input
                     v-model="queryParams.equipmentNumber"
                     placeholder="请输入设备编号"
                     clearable
                     style="width: 200px"
                     @keyup.enter="handleQuery"
                  />
               </el-form-item>
               <el-form-item label="设备名称" prop="equipmentName">
                  <el-input
                     v-model="queryParams.equipmentName"
                     placeholder="请输入设备名称"
                     clearable
                     style="width: 200px"
                     @keyup.enter="handleQuery"
                  />
               </el-form-item>
               <el-form-item label="设备分类" prop="deviceCategory">
                  <el-select v-model="queryParams.deviceCategory" placeholder="设备分类" clearable style="width: 200px">
                     <el-option
                        v-for="dict in device_category"
                        :key="dict.value"
                        :label="dict.label"
                        :value="dict.value"
                     />
                  </el-select>
               </el-form-item>
               <el-form-item label="报警状态" prop="alarmStatus">
                  <el-select v-model="queryParams.alarmStatus" placeholder="报警状态" clearable style="width: 200px">
                     <el-option
                        v-for="dict in alarm_status"
                        :key="dict.value"
                        :label="dict.label"
                        :value="dict.value"
                     />
                  </el-select>
               </el-form-item>
               <el-form-item>
                  <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
                  <el-button icon="Refresh" @click="resetQuery">重置</el-button>
               </el-form-item>
            </el-form>

            <el-row :gutter="10" class="mb8">
               <el-col :span="1.5">
                  <el-button
                     type="primary"
                     plain
                     icon="Plus"
                     @click="handleAdd"
                     v-hasPermi="['equipment_data:daum:add']"
                  >新增</el-button>
               </el-col>
               <el-col :span="1.5">
                  <el-button
                     type="success"
                     plain
                     icon="Edit"
                     :disabled="single"
                     @click="handleUpdate"
                     v-hasPermi="['equipment_data:daum:edit']"
                  >修改</el-button>
               </el-col>
               <el-col :span="1.5">
                  <el-button
                     type="danger"
                     plain
                     icon="Delete"
                     :disabled="multiple"
                     @click="handleDelete"
                     v-hasPermi="['equipment_data:daum:remove']"
                  >删除</el-button>
               </el-col>
               <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
            </el-row>

            <el-table v-loading="loading" :data="dataList" @selection-change="handleSelectionChange">
               <el-table-column type="selection" width="50" align="center" />
               <el-table-column label="设备编号" align="center" prop="equipmentNumber" :show-overflow-tooltip="true" />
               <el-table-column label="设备名称" align="center" prop="equipmentName" :show-overflow-tooltip="true" />
               <el-table-column label="归属部门" align="center" prop="deptName" :show-overflow-tooltip="true" />
               <el-table-column label="设备分类" align="center" prop="deviceCategory">
                  <template #default="scope">
                     <dict-tag :options="device_category" :value="scope.row.deviceCategory" />
                  </template>
               </el-table-column>
               <el-table-column label="电量" align="center" prop="batteryLevel" width="90" />
               <el-table-column label="信号强度" align="center" prop="signalStrength" width="100" />
               <el-table-column label="报警状态" align="center" prop="alarmStatus">
                  <template #default="scope">
                     <dict-tag :options="alarm_status" :value="String(scope.row.alarmStatus)" />
                  </template>
               </el-table-column>
               <el-table-column label="上报时间" align="center" prop="uploadTime" width="160">
                  <template #default="scope">
                     <span>{{ parseTime(scope.row.uploadTime) }}</span>
                  </template>
               </el-table-column>
               <el-table-column label="操作" align="center" width="200" class-name="small-padding fixed-width">
                  <template #default="scope">
                     <el-button link type="primary" icon="View" @click="handleDetail(scope.row)">详情</el-button>
                     <el-button
                        link
                        type="primary"
                        icon="Edit"
                        @click="handleUpdate(scope.row)"
                        v-hasPermi="['equipment_data:daum:edit']"
                     >修改</el-button>
                     <el-button
                        link
                        type="primary"
                        icon="Delete"
                        @click="handleDelete(scope.row)"
                        v-hasPermi="['equipment_data:daum:remove']"
                     >删除</el-button>
                  </template>
               </el-table-column>
            </el-table>

            <pagination
               v-show="total > 0"
               :total="total"
               v-model:page="queryParams.pageNum"
               v-model:limit="queryParams.pageSize"
               @pagination="getList"
            />
         </el-col>
      </el-row>

      <!-- 添加或修改设备数据对话框 -->
      <el-dialog :title="title" v-model="open" width="720px" append-to-body>
         <el-form ref="dataRef" :model="form" :rules="rules" label-width="90px">
            <el-row>
               <el-col :span="12">
                  <el-form-item label="设备编号" prop="equipmentNumber">
                     <el-input v-model="form.equipmentNumber" placeholder="请输入设备编号" maxlength="100" />
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="设备名称" prop="equipmentName">
                     <el-input v-model="form.equipmentName" placeholder="请输入设备名称" maxlength="200" />
                  </el-form-item>
               </el-col>
            </el-row>
            <el-row>
               <el-col :span="12">
                  <el-form-item label="归属部门" prop="deptId">
                     <el-tree-select
                        v-model="form.deptId"
                        :data="deptOptions"
                        :props="{ value: 'id', label: 'label', children: 'children' }"
                        value-key="id"
                        placeholder="请选择归属部门"
                        check-strictly
                        clearable
                        style="width: 100%"
                     />
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="设备分类" prop="deviceCategory">
                     <el-select v-model="form.deviceCategory" placeholder="请选择设备分类" clearable style="width: 100%">
                        <el-option
                           v-for="dict in device_category"
                           :key="dict.value"
                           :label="dict.label"
                           :value="dict.value"
                        />
                     </el-select>
                  </el-form-item>
               </el-col>
            </el-row>
            <el-row>
               <el-col :span="12">
                  <el-form-item label="电量" prop="batteryLevel">
                     <el-input-number v-model="form.batteryLevel" :min="0" :max="100" :precision="1" controls-position="right" style="width: 100%" />
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="信号强度" prop="signalStrength">
                     <el-input-number v-model="form.signalStrength" :min="-200" :max="100" controls-position="right" style="width: 100%" />
                  </el-form-item>
               </el-col>
            </el-row>
            <el-form-item label="报警状态" prop="alarmStatus">
               <el-radio-group v-model="form.alarmStatus">
                  <el-radio
                     v-for="dict in alarm_status"
                     :key="dict.value"
                     :label="Number(dict.value)"
                  >{{ dict.label }}</el-radio>
               </el-radio-group>
            </el-form-item>
            <el-form-item label="数据内容" prop="payload">
               <el-input
                  v-model="form.payload"
                  type="textarea"
                  :rows="8"
                  placeholder='请输入 JSON 数据，例如 {"temperature": 25.6}'
               />
            </el-form-item>
            <el-form-item label="备注" prop="remark">
               <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" maxlength="500" />
            </el-form-item>
         </el-form>
         <template #footer>
            <div class="dialog-footer">
               <el-button type="primary" @click="submitForm">确 定</el-button>
               <el-button @click="cancel">取 消</el-button>
            </div>
         </template>
      </el-dialog>

      <!-- 设备数据详情 -->
      <el-dialog title="设备数据详情" v-model="detailOpen" width="720px" append-to-body>
         <el-descriptions :column="2" border>
            <el-descriptions-item label="设备编号">{{ detailRow.equipmentNumber }}</el-descriptions-item>
            <el-descriptions-item label="设备名称">{{ detailRow.equipmentName }}</el-descriptions-item>
            <el-descriptions-item label="归属部门">{{ detailRow.deptName }}</el-descriptions-item>
            <el-descriptions-item label="设备分类">
               <dict-tag :options="device_category" :value="detailRow.deviceCategory" />
            </el-descriptions-item>
            <el-descriptions-item label="电量">{{ detailRow.batteryLevel }}</el-descriptions-item>
            <el-descriptions-item label="信号强度">{{ detailRow.signalStrength }}</el-descriptions-item>
            <el-descriptions-item label="报警状态">
               <dict-tag :options="alarm_status" :value="String(detailRow.alarmStatus)" />
            </el-descriptions-item>
            <el-descriptions-item label="上报时间">{{ parseTime(detailRow.uploadTime) }}</el-descriptions-item>
         </el-descriptions>
         <div class="payload-title">数据内容</div>
         <pre class="payload-box">{{ formatPayload(detailRow.payload) }}</pre>
         <template #footer>
            <div class="dialog-footer">
               <el-button @click="detailOpen = false">关 闭</el-button>
            </div>
         </template>
      </el-dialog>
   </div>
</template>

<script setup name="DeviceData">
import { listData, getData, addData, updateData, delData, getDataStats } from "@/api/device/data";
import { deptTreeSelect } from "@/api/system/dept";

const { proxy } = getCurrentInstance();
const isDemo = import.meta.env.VITE_APP_DEMO === "true";
const { device_category, alarm_status } = proxy.useDict("device_category", "alarm_status");

const dataList = ref([]);
const open = ref(false);
const detailOpen = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const deptName = ref("");
const deptOptions = ref([]);
const detailRow = ref({});
const stats = ref({ total: 0, online: 0, offline: 0 });

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    equipmentNumber: undefined,
    equipmentName: undefined,
    deviceCategory: undefined,
    alarmStatus: undefined,
    deptId: undefined
  },
  rules: {
    equipmentNumber: [{ required: true, message: "设备编号不能为空", trigger: "blur" }]
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 通过条件过滤节点 */
const filterNode = (value, data) => {
  if (!value) return true;
  return data.label.indexOf(value) !== -1;
};
/** 根据名称筛选部门树 */
watch(deptName, val => {
  proxy.$refs["deptTreeRef"].filter(val);
});
/** 查询部门下拉树结构 */
function getDeptTree() {
  deptTreeSelect().then(response => {
    deptOptions.value = response.data;
  });
}
/** 查询设备数据列表 */
function getList() {
  loading.value = true;
  const params = { ...queryParams.value };
  // 字典值默认是字符串，后端 alarmStatus 为整型，这里做一次转换
  if (params.alarmStatus !== undefined && params.alarmStatus !== null && params.alarmStatus !== "") {
    params.alarmStatus = Number(params.alarmStatus);
  }
  listData(params).then(response => {
    loading.value = false;
    dataList.value = response.rows;
    total.value = response.total;
  });
}
/** 查询设备状态统计 */
function getStats() {
  getDataStats().then(response => {
    stats.value = response.data || { total: 0, online: 0, offline: 0 };
  });
}
/** 部门树节点单击事件 */
function handleNodeClick(data) {
  const deptId = Number(data.id);
  queryParams.value.deptId = Number.isNaN(deptId) ? undefined : deptId;
  handleQuery();
}
/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}
/** 重置按钮操作 */
function resetQuery() {
  proxy.resetForm("queryRef");
  queryParams.value.deptId = undefined;
  proxy.$refs["deptTreeRef"].setCurrentKey(null);
  handleQuery();
}
/** 多选框选中数据 */
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.id);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}
/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加设备数据";
}
/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const id = row.id || ids.value[0];
  getData(id).then(response => {
    form.value = normalizeForm(response.data);
    open.value = true;
    title.value = "修改设备数据";
  });
}
/** 详情按钮操作 */
function handleDetail(row) {
  detailRow.value = row;
  detailOpen.value = true;
}
/** 表单重置 */
function reset() {
  form.value = {
    id: undefined,
    equipmentNumber: undefined,
    equipmentName: undefined,
    deptId: undefined,
    deviceCategory: undefined,
    batteryLevel: 0,
    signalStrength: 0,
    alarmStatus: 0,
    payload: undefined,
    remark: undefined
  };
  proxy.resetForm("dataRef");
}
/** 后端部门ID转成前端树的字符串ID；payload 对象转 JSON 文本 */
function normalizeForm(row) {
  return {
    ...row,
    deptId: row.deptId != null ? String(row.deptId) : undefined,
    payload: row.payload != null
      ? (typeof row.payload === "string" ? row.payload : JSON.stringify(row.payload, null, 2))
      : undefined
  };
}
/** 提交按钮 */
function submitForm() {
  proxy.$refs["dataRef"].validate(valid => {
    if (!valid) return;
    // 只提交后端 DTO 需要的字段（避免把 uploadTime/createTime 等回传）
    const payload = {
      id: form.value.id,
      equipmentNumber: form.value.equipmentNumber,
      equipmentName: form.value.equipmentName,
      deviceCategory: form.value.deviceCategory,
      batteryLevel: form.value.batteryLevel,
      signalStrength: form.value.signalStrength,
      alarmStatus: form.value.alarmStatus,
      payload: form.value.payload,
      remark: form.value.remark
    };
    if (form.value.deptId !== undefined && form.value.deptId !== null && form.value.deptId !== "") {
      const deptId = Number(form.value.deptId);
      payload.deptId = Number.isNaN(deptId) ? null : deptId;
    } else {
      payload.deptId = null;
    }
    // 数据内容为 JSON 文本，转成对象提交
    if (payload.payload !== undefined && payload.payload !== null && String(payload.payload).trim() !== "") {
      try {
        payload.payload = JSON.parse(payload.payload);
      } catch (e) {
        proxy.$modal.msgError("数据内容不是合法的 JSON");
        return;
      }
    } else {
      payload.payload = null;
    }
    const request = payload.id != undefined ? updateData(payload) : addData(payload);
    request.then(() => {
      proxy.$modal.msgSuccess(payload.id != undefined ? "修改成功" : "新增成功");
      open.value = false;
      getList();
      getStats();
    });
  });
}
/** 删除按钮操作 */
function handleDelete(row) {
  const deleteIds = row.id || ids.value;
  proxy.$modal.confirm('是否确认删除设备数据编号为"' + deleteIds + '"的数据项？').then(function () {
    return delData(deleteIds);
  }).then(() => {
    getList();
    getStats();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => {});
}
/** 格式化数据内容 */
function formatPayload(payload) {
  if (payload === undefined || payload === null || payload === "") return "无";
  try {
    const obj = typeof payload === "string" ? JSON.parse(payload) : payload;
    return JSON.stringify(obj, null, 2);
  } catch (e) {
    return String(payload);
  }
}

getDeptTree();
getList();
getStats();
</script>

<style scoped>
.stat-row {
  margin-bottom: 8px;
}
.stat-card {
  text-align: center;
  border-radius: 8px;
}
.stat-title {
  color: #909399;
  font-size: 14px;
}
.stat-value {
  margin-top: 8px;
  font-size: 28px;
  font-weight: 600;
  color: #303133;
}
.stat-online .stat-value {
  color: #67c23a;
}
.stat-offline .stat-value {
  color: #f56c6c;
}
.payload-title {
  margin: 16px 0 8px;
  font-weight: 600;
  color: #303133;
}
.payload-box {
  max-height: 320px;
  overflow: auto;
  padding: 12px;
  border-radius: 4px;
  background: #f5f7fa;
  color: #303133;
  font-size: 12px;
  line-height: 1.6;
}
</style>
