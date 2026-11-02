import request from '@/utils/request'
// 新增
export function uploadFile(data) {
    return request({
        url: '/system/uploads',
        method: 'post',
        data: data
    })
}

// 修改菜单
export function uploadForm(data) {
    return request({
        url: '/system/uploadfrm',
        method: 'post',
        data: data
    })
}
