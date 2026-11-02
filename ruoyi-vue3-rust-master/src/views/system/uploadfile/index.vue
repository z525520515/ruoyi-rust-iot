<template>
  <div class="app-container">

      <el-form ref="refForm" :model="form" label-width="150px">
        <el-form-item label="name" prop="name">
          <el-input v-model="form.name"></el-input>
        </el-form-item>
        <el-form-item label="category" prop="category">
          <el-input v-model="form.category"></el-input>
        </el-form-item>
        <el-form-item label="description" prop="description">
          <el-input v-model="form.description"></el-input>
        </el-form-item>
        <el-form-item label="tags" prop="tags">
          <el-input v-model="form.tags"></el-input>
        </el-form-item>
        <el-form-item label="导入文件" prop="uploadFile">
          <el-upload v-model="form.document"
                     class="upload-demo"
                     action="#"
                     ref="upload"
                     :auto-upload="false"
                     :file-list="fileList1"
                     :on-change="changeFile"
                     :headers="headers"

          >
            <el-button class="btn"><i class="el-icon-paperclip">                            </i>上传附件</el-button>
          </el-upload>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm">确定</el-button>
        </el-form-item>

    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button
            type="primary"
            plain
            icon="Plus"
            @click="handleAdd"
            v-hasPermi="['system:notice:add']"
        >新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
            type="success"
            plain
            icon="Edit"
            :disabled="single"
            @click="handleUpdate"
            v-hasPermi="['system:notice:edit']"
        >修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
            type="danger"
            plain
            icon="Delete"
            :disabled="multiple"
            @click="handleDelete"
            v-hasPermi="['system:notice:remove']"
        >删除</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>


  </div>
</template>

<script setup name="Notice">
import { uploadFile,uploadForm } from '@/api/system/test'
import { getServer } from '@/api/monitor/server'

const { proxy } = getCurrentInstance();

const title = ref("");

const headers = ref({ "Content-Type": "multipart/form-data" });
const fileList1 = ref([]);
const data = reactive({
  form: {},
});

const {form } = toRefs(data);
/** 表单重置 */
function reset() {
  form.value = {
    noticeId: undefined,
    noticeTitle: undefined,
    noticeType: undefined,
    noticeContent: undefined,
    status: "0"
  };
  proxy.resetForm("refForm");
}
function changeFile(file, fileList) {
  fileList1.value=fileList;
  //console.log('fileList ',fileList1);
  //console.log('file ',file);
}


function submitForm()
{
  if (!(fileList1 === undefined || fileList1 == null)) {
    let formData = new FormData();
    console.log('file1 ',fileList1.value[0]);
    let namef=fileList1.value[0].name;
    formData.append("document", fileList1.value[0].raw,namef);
    formData.append("name", form.value.name);
    formData.append("category", form.value.category);
    formData.append("description", form.value.description);
    formData.append("tags", form.value.tags);
    uploadFile(formData).then(response => {
      console.log(response);
      proxy.$modal.msgSuccess("上传成功");
    });
  }
}
reset();
// 上传图片
</script>
