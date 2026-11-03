import request from '@/utils/request'

// 查询设备台账列表
export function listInstallation(query) {
  return request({
    url: '/equipment/installation/list',
    method: 'get',
    params: query
  })
}

// 查询设备台账详细
export function getInstallation(id) {
  return request({
    url: '/equipment/installation/' + id,
    method: 'get'
  })
}

// 新增设备台账
export function addInstallation(data) {
  return request({
    url: '/equipment/installation',
    method: 'post',
    data: data
  })
}

// 修改设备台账
export function updateInstallation(data) {
  return request({
    url: '/equipment/installation',
    method: 'put',
    data: data
  })
}

// 删除设备台账
export function delInstallation(id) {
  return request({
    url: '/equipment/installation/' + id,
    method: 'delete'
  })
}

// 下载设备导入模板（后端为 GET 二进制接口）
export function exportTemplate() {
  return request({
    url: '/equipment/export',
    method: 'get',
    responseType: 'blob'
  })
}
