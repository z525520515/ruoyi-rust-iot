import request from '@/utils/request'

// 查询设备数据列表
export function listData(query) {
  return request({
    url: '/equipment_data/daum/list',
    method: 'get',
    params: query
  })
}

// 查询设备数据详细
export function getData(id) {
  return request({
    url: '/equipment_data/daum/' + id,
    method: 'get'
  })
}

// 新增设备数据
export function addData(data) {
  return request({
    url: '/equipment_data/daum',
    method: 'post',
    data: data
  })
}

// 修改设备数据
export function updateData(data) {
  return request({
    url: '/equipment_data/daum',
    method: 'put',
    data: data
  })
}

// 删除设备数据
export function delData(id) {
  return request({
    url: '/equipment_data/daum/' + id,
    method: 'delete'
  })
}

// 设备在线状态统计（总数 / 在线 / 离线）
export function getDataStats() {
  return request({
    url: '/equipment_data/daum/stats',
    method: 'get'
  })
}
