<template>
   <div class="app-container">
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
         <!--设备台账数据-->
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
               <el-form-item label="设备类型" prop="equipmentType">
                  <el-select v-model="queryParams.equipmentType" placeholder="设备类型" clearable style="width: 200px">
                     <el-option
                        v-for="dict in equipment_type"
                        :key="dict.value"
                        :label="dict.label"
                        :value="dict.value"
                     />
                  </el-select>
               </el-form-item>
               <el-form-item label="设备状态" prop="equipmentStatus">
                  <el-select v-model="queryParams.equipmentStatus" placeholder="设备状态" clearable style="width: 200px">
                     <el-option
                        v-for="dict in equipment_status"
                        :key="dict.value"
                        :label="dict.label"
                        :value="dict.value"
                     />
                  </el-select>
               </el-form-item>
               <el-form-item label="通信方式" prop="communicationMeans">
                  <el-select v-model="queryParams.communicationMeans" placeholder="通信方式" clearable style="width: 200px">
                     <el-option
                        v-for="dict in communication_means"
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
                     v-hasPermi="['equipment:installation:add']"
                  >新增</el-button>
               </el-col>
               <el-col :span="1.5">
                  <el-button
                     type="success"
                     plain
                     icon="Edit"
                     :disabled="single"
                     @click="handleUpdate"
                     v-hasPermi="['equipment:installation:edit']"
                  >修改</el-button>
               </el-col>
               <el-col :span="1.5">
                  <el-button
                     type="danger"
                     plain
                     icon="Delete"
                     :disabled="multiple"
                     @click="handleDelete"
                     v-hasPermi="['equipment:installation:remove']"
                  >删除</el-button>
               </el-col>
               <el-col :span="1.5">
                  <el-button
                     type="info"
                     plain
                     icon="Upload"
                     @click="handleImport"
                     v-hasPermi="['equipment:installation:import']"
                  >导入</el-button>
               </el-col>
               <el-col :span="1.5">
                  <el-button
                     type="warning"
                     plain
                     icon="Download"
                     @click="handleExport"
                     v-hasPermi="['equipment:installation:export']"
                  >下载模板</el-button>
               </el-col>
               <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
            </el-row>

            <el-table v-loading="loading" :data="installationList" @selection-change="handleSelectionChange">
               <el-table-column type="selection" width="50" align="center" />
               <el-table-column label="设备编号" align="center" prop="equipmentNumber" :show-overflow-tooltip="true" />
               <el-table-column label="设备名称" align="center" prop="equipmentName" :show-overflow-tooltip="true" />
               <el-table-column label="归属部门" align="center" prop="deptName" :show-overflow-tooltip="true" />
               <el-table-column label="设备类型" align="center" prop="equipmentType">
                  <template #default="scope">
                     <dict-tag :options="equipment_type" :value="scope.row.equipmentType" />
                  </template>
               </el-table-column>
               <el-table-column label="型号标签" align="center" prop="equipmentModelLabel" :show-overflow-tooltip="true" />
               <el-table-column label="设备状态" align="center" prop="equipmentStatus">
                  <template #default="scope">
                     <dict-tag :options="equipment_status" :value="scope.row.equipmentStatus" />
                  </template>
               </el-table-column>
               <el-table-column label="通信方式" align="center" prop="communicationMeans">
                  <template #default="scope">
                     <dict-tag :options="communication_means" :value="scope.row.communicationMeans" />
                  </template>
               </el-table-column>
               <el-table-column label="项目编号" align="center" prop="projectNum" :show-overflow-tooltip="true" />
               <el-table-column label="创建时间" align="center" prop="createTime" width="160">
                  <template #default="scope">
                     <span>{{ parseTime(scope.row.createTime) }}</span>
                  </template>
               </el-table-column>
               <el-table-column label="操作" align="center" width="150" class-name="small-padding fixed-width">
                  <template #default="scope">
                     <el-button
                        link
                        type="primary"
                        icon="Edit"
                        @click="handleUpdate(scope.row)"
                        v-hasPermi="['equipment:installation:edit']"
                     >修改</el-button>
                     <el-button
                        link
                        type="primary"
                        icon="Delete"
                        @click="handleDelete(scope.row)"
                        v-hasPermi="['equipment:installation:remove']"
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

      <!-- 添加或修改设备台账对话框 -->
      <el-dialog :title="title" v-model="open" width="720px" append-to-body>
         <el-form ref="installationRef" :model="form" :rules="rules" label-width="90px">
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
                  <el-form-item label="设备类型" prop="equipmentType">
                     <el-select v-model="form.equipmentType" placeholder="请选择设备类型" clearable style="width: 100%">
                        <el-option
                           v-for="dict in equipment_type"
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
                  <el-form-item label="型号标签" prop="equipmentModelLabel">
                     <el-input v-model="form.equipmentModelLabel" placeholder="请输入设备型号标签" maxlength="100" />
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="设备状态" prop="equipmentStatus">
                     <el-select v-model="form.equipmentStatus" placeholder="请选择设备状态" clearable style="width: 100%">
                        <el-option
                           v-for="dict in equipment_status"
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
                  <el-form-item label="通信方式" prop="communicationMeans">
                     <el-select v-model="form.communicationMeans" placeholder="请选择通信方式" clearable style="width: 100%">
                        <el-option
                           v-for="dict in communication_means"
                           :key="dict.value"
                           :label="dict.label"
                           :value="dict.value"
                        />
                     </el-select>
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="安装图纸" prop="installationDrawing">
                     <el-input v-model="form.installationDrawing" placeholder="请输入图纸地址" maxlength="500" />
                  </el-form-item>
               </el-col>
            </el-row>
            <el-row>
               <el-col :span="12">
                  <el-form-item label="安装地址" prop="addr">
                     <el-input v-model="form.addr" placeholder="请输入安装地址" maxlength="255" />
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="位置ID" prop="locationId">
                     <el-input v-model="form.locationId" placeholder="请输入位置ID" maxlength="100" />
                  </el-form-item>
               </el-col>
            </el-row>
            <el-row>
               <el-col :span="12">
                  <el-form-item label="下级设备" prop="lowerEquipment">
                     <el-input v-model="form.lowerEquipment" placeholder="请输入下级设备" maxlength="100" />
                  </el-form-item>
               </el-col>
               <el-col :span="12">
                  <el-form-item label="项目编号" prop="projectNum">
                     <el-input v-model="form.projectNum" placeholder="请输入项目编号" maxlength="100" />
                  </el-form-item>
               </el-col>
            </el-row>
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

      <!-- 设备导入对话框 -->
      <el-dialog :title="upload.title" v-model="upload.open" width="400px" append-to-body>
         <el-upload
            ref="uploadRef"
            :limit="1"
            accept=".xlsx, .xls"
            :headers="upload.headers"
            :action="upload.url"
            :disabled="upload.isUploading"
            :on-progress="handleFileUploadProgress"
            :on-success="handleFileImportSuccess"
            :on-error="handleFileImportError"
            :auto-upload="false"
            drag
         >
            <div class="el-upload__text">将文件拖到此处，或<em>点击上传</em></div>
            <template #tip>
               <div class="el-upload__tip text-center">
                  <span>仅允许导入 xls、xlsx 格式文件。</span>
                  <el-link type="primary" :underline="false" style="font-size: 12px; vertical-align: baseline" @click="handleExport">下载模板</el-link>
               </div>
            </template>
         </el-upload>
         <template #footer>
            <div class="dialog-footer">
               <el-button type="primary" @click="submitFileForm">确 定</el-button>
               <el-button @click="upload.open = false">取 消</el-button>
            </div>
         </template>
      </el-dialog>
   </div>
</template>

<script setup name="DeviceInstallation">
import { listInstallation, getInstallation, addInstallation, updateInstallation, delInstallation, exportTemplate } from "@/api/device/installation";
import { deptTreeSelect } from "@/api/system/dept";
import { getToken } from "@/utils/auth";
import { saveAs } from "file-saver";

const { proxy } = getCurrentInstance();
const { equipment_type, equipment_status, communication_means } = proxy.useDict("equipment_type", "equipment_status", "communication_means");

const installationList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const deptName = ref("");
const deptOptions = ref([]);

/*** 设备导入参数 */
const upload = reactive({
  open: false,
  title: "设备导入",
  isUploading: false,
  headers: { Authorization: "Bearer " + getToken() },
  url: import.meta.env.VITE_APP_BASE_API + "/equipment/import"
});

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    equipmentNumber: undefined,
    equipmentName: undefined,
    equipmentType: undefined,
    equipmentStatus: undefined,
    communicationMeans: undefined,
    deptId: undefined
  },
  rules: {
    equipmentNumber: [{ required: true, message: "设备编号不能为空", trigger: "blur" }],
    equipmentName: [{ required: true, message: "设备名称不能为空", trigger: "blur" }],
    deptId: [{ required: true, message: "归属部门不能为空", trigger: "change" }]
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
/** 查询设备台账列表 */
function getList() {
  loading.value = true;
  listInstallation(queryParams.value).then(response => {
    loading.value = false;
    installationList.value = response.rows;
    total.value = response.total;
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
  title.value = "添加设备";
}
/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const id = row.id || ids.value[0];
  getInstallation(id).then(response => {
    form.value = normalizeForm(response.data);
    open.value = true;
    title.value = "修改设备";
  });
}
/** 表单重置 */
function reset() {
  form.value = {
    id: undefined,
    deptId: undefined,
    equipmentNumber: undefined,
    equipmentName: undefined,
    equipmentType: undefined,
    equipmentModelLabel: undefined,
    equipmentStatus: undefined,
    communicationMeans: undefined,
    installationDrawing: undefined,
    addr: undefined,
    locationId: undefined,
    lowerEquipment: undefined,
    projectNum: undefined,
    remark: undefined
  };
  proxy.resetForm("installationRef");
}
/** 后端数字部门ID转成前端树的字符串ID */
function normalizeForm(row) {
  return {
    ...row,
    deptId: row.deptId != null ? String(row.deptId) : undefined
  };
}
/** 提交按钮 */
function submitForm() {
  proxy.$refs["installationRef"].validate(valid => {
    if (valid) {
      const payload = { ...form.value };
      if (payload.deptId !== undefined && payload.deptId !== null && payload.deptId !== "") {
        const deptId = Number(payload.deptId);
        payload.deptId = Number.isNaN(deptId) ? null : deptId;
      }
      const request = payload.id != undefined ? updateInstallation(payload) : addInstallation(payload);
      request.then(() => {
        proxy.$modal.msgSuccess(payload.id != undefined ? "修改成功" : "新增成功");
        open.value = false;
        getList();
      });
    }
  });
}
/** 删除按钮操作 */
function handleDelete(row) {
  const deleteIds = row.id || ids.value;
  proxy.$modal.confirm('是否确认删除设备编号为"' + deleteIds + '"的数据项？').then(function () {
    return delInstallation(deleteIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => {});
}
/** 下载导入模板 */
function handleExport() {
  exportTemplate().then(data => {
    saveAs(new Blob([data]), `设备台账导入模板_${new Date().getTime()}.xlsx`);
    proxy.$modal.msgSuccess("模板下载成功");
  });
}
/** 导入按钮操作 */
function handleImport() {
  upload.open = true;
}
/** 文件上传中处理 */
function handleFileUploadProgress() {
  upload.isUploading = true;
}
/** 文件上传成功处理 */
function handleFileImportSuccess(response) {
  upload.isUploading = false;
  let res = response;
  if (typeof res === "string") {
    try {
      res = JSON.parse(res);
    } catch (e) {
      res = {};
    }
  }
  if (res && res.code === 200) {
    proxy.$modal.msgSuccess(res.msg || "导入成功");
    upload.open = false;
    proxy.$refs["uploadRef"].clearFiles();
    getList();
  } else {
    proxy.$modal.msgError((res && res.msg) || "导入失败");
  }
}
/** 文件上传失败处理 */
function handleFileImportError() {
  upload.isUploading = false;
  proxy.$modal.msgError("导入失败，请检查文件后重试");
}
/** 提交上传文件 */
function submitFileForm() {
  if (!proxy.$refs["uploadRef"].uploadFiles.length) {
    proxy.$modal.msgWarning("请先选择要导入的文件");
    return;
  }
  proxy.$refs["uploadRef"].submit();
}

getDeptTree();
getList();
</script>
