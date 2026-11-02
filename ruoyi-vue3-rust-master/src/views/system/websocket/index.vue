<template>
  <div class="app-container">

      <el-form ref="refForm" :model="form" label-width="150px">
        <el-form-item label="userId" prop="userId">
          <el-input v-model="form.userId"></el-input>
        </el-form-item>
        <el-form-item label="message" prop="message">
          <el-input v-model="form.message"></el-input>
        </el-form-item>



    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button
            type="primary"
            plain
            icon="Plus"
            @click="handleSend()"

        >发送</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
            type="success"
            plain
            icon="Edit"

            @click="handleHeartbeat"

        >发送ping</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button
            type="danger"
            plain
            icon="Delete"

            @click="reset()"

        >重置</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>


  </div>
</template>

<script setup name="websocket">

import useWebSocket from '@/utils/useWebSocket';
const { proxy } = getCurrentInstance();
const hostName = window.location.hostname;
const currentPath = window.location.pathname;
const port = window.location.port;
console.log('hostName : ',hostName,port,currentPath);
const user_id = ref("1");
//const ws_url = 'ws://'+hostName+import.meta.env.VITE_APP_BASE_API+'/ws/'+user_id.value;
const ws_url = 'ws://127.0.0.1:60800' +import.meta.env.VITE_APP_BASE_API+'/ws/'+user_id.value;
//console.log('ws_url : ',ws_url);
const {connect, messages, isConnected, send } = useWebSocket(ws_url)


const data = reactive({
  form: {},
});

const {form } = toRefs(data);
/** 表单重置 */
function reset() {
  form.value = {
    userId: 0,
    message: '',

  };
  proxy.resetForm("refForm");
}
// 组件挂载时连接
onMounted(() => {
  console.log('ws_url ',ws_url);
  connect({
    onOpen: () => {
      console.log('WebSocket opened');
    },
    onMessage: (data) => {
      console.log('onMessage ',data);
    },
    onClose: (event) => {
      console.log('WebSocket closed event',event);
    },
    onError: (event) => {
      console.error('WebSocket error:', event);
    }}
  )
})

// 发送示例
const handleSend = () => {
  send({message_type: 'broadcast', content: 'Hello',recipient_id: 1})
}
const handleHeartbeat = () => {
  send({message_type: 'heartbeat', content: '0'})
}
// // 组件挂载时连接
// onMounted(() => {
//   connectWebSocket(user_id.value)
//   proxy.$globalData.$Socket.eventPatch.onMsg((msg, sk) => {    //监听是否接受消息
//     console.log('from server : ',msg)
//   });
// })
//
// // 发送示例
// const handleSend = (message,to_user) => {
//   if(proxy.$globalData.$Socket!==undefined){
//     proxy.$globalData.$Socket.nsend(`{message_type:'message',content:'${message}',recipient_id:${to_user}}`)
//     //SOCKET.value.send({ message_type: 'message', content: 'Hello',recipient_id: to_user})
//   }
//
// }
function submitForm()
{
  let id = form.value.userId|undefined?? 0 ;
  let message = form.value.message|undefined?? '' ;
  handleSend(message,id);
}
// 上传图片
</script>
