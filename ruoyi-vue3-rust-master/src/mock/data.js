/**
 * GitHub Pages 演示环境使用的 Mock 数据。
 * 仅在 VITE_APP_DEMO=true 时被请求适配器使用，不会影响真实后端联调。
 */

// 演示账号（前端校验）
export const DEMO_ACCOUNT = { username: "admin", password: "admin123" };

export const demoUser = {
  userId: "1",
  deptId: "103",
  userName: "admin",
  nickName: "演示管理员",
  email: "admin@example.com",
  phonenumber: "13800000000",
  sex: "0",
  avatar: "",
  status: "0",
  loginIp: "127.0.0.1",
  loginDate: "2026-11-05 17:00:00",
  remark: "Mock 演示账号",
  admin: true
};

export const demoRoles = ["admin"];
export const demoPermissions = ["*:*:*"];

// ------------------------------------------------------------
// 部门树（与后端 sys_dept 演示数据保持一致，使用数字型部门ID）
// ------------------------------------------------------------
export const deptTree = [
  {
    id: "100",
    label: "若依科技",
    children: [
      {
        id: "101",
        label: "深圳总公司",
        children: [
          { id: "103", label: "研发部门" },
          { id: "104", label: "市场部门" },
          { id: "105", label: "测试部门" },
          { id: "106", label: "财务部门" },
          { id: "107", label: "运维部门" }
        ]
      },
      {
        id: "102",
        label: "长沙分公司",
        children: [
          { id: "108", label: "市场部门" },
          { id: "109", label: "财务部门" }
        ]
      }
    ]
  }
];

function flattenDept(nodes, ancestors = []) {
  const rows = [];
  (nodes || []).forEach(node => {
    rows.push({ id: node.id, label: node.label, ancestors });
    if (node.children && node.children.length) {
      rows.push(...flattenDept(node.children, [...ancestors, node.id]));
    }
  });
  return rows;
}

const deptRows = flattenDept(deptTree);
const deptNameMap = new Map(deptRows.map(d => [d.id, d.label]));
const deptAncestorMap = new Map(deptRows.map(d => [d.id, d.ancestors]));

export function deptName(deptId) {
  if (deptId === undefined || deptId === null) return "";
  return deptNameMap.get(String(deptId)) || "";
}

/** 返回部门自身 + 全部下级部门的数字ID */
function subtreeIds(deptId) {
  const target = String(deptId);
  return deptRows
    .filter(d => d.id === target || (deptAncestorMap.get(d.id) || []).includes(target))
    .map(d => Number(d.id));
}

// ------------------------------------------------------------
// 数据字典（对应后端 sys_dict_data）
// ------------------------------------------------------------
const dict = items =>
  items.map(([dictValue, dictLabel, listClass], index) => ({
    dictCode: `demo-${index}`,
    dictSort: index + 1,
    dictLabel,
    dictValue,
    dictType: "",
    cssClass: "",
    listClass: listClass || "default",
    isDefault: index === 0 ? "Y" : "N",
    status: "0"
  }));

export const dicts = {
  device_category: dict([
    ["electric", "电表", "default"],
    ["water", "水表", "default"],
    ["terminal", "终端", "default"],
    ["alarm", "报警器", "danger"],
    ["wireless", "无线终端", "default"]
  ]),
  alarm_status: dict([
    ["0", "正常", "success"],
    ["1", "报警", "danger"]
  ]),
  equipment_status: dict([
    ["0", "在线", "success"],
    ["1", "离线", "info"],
    ["2", "故障", "danger"],
    ["3", "停用", "warning"]
  ]),
  equipment_type: dict([
    ["01", "压力表", "default"],
    ["02", "水表", "default"],
    ["03", "电表", "default"],
    ["04", "温度计", "default"],
    ["05", "流量计", "default"],
    ["99", "其他", "info"]
  ]),
  communication_means: dict([
    ["MQTT", "MQTT", "default"],
    ["TCP", "TCP", "default"],
    ["HTTP", "HTTP", "default"],
    ["SERIAL", "串口", "default"],
    ["PLC", "PLC", "default"],
    ["OTHER", "其他", "info"]
  ])
};

// ------------------------------------------------------------
// 动态菜单（模拟后端 /getRouters 返回，component 对应 src/views 下的页面）
// ------------------------------------------------------------
export const demoRouters = [
  {
    name: "Device",
    path: "/device",
    hidden: false,
    redirect: "noRedirect",
    component: "Layout",
    alwaysShow: true,
    meta: { title: "设备管理", icon: "server", noCache: false, link: null },
    children: [
      {
        name: "Installation",
        path: "installation",
        hidden: false,
        component: "device/installation/index",
        meta: { title: "设备台账", icon: "list", noCache: false, link: null }
      },
      {
        name: "DeviceData",
        path: "deviceData",
        hidden: false,
        component: "device/data/index",
        meta: { title: "设备数据", icon: "chart", noCache: false, link: null }
      }
    ]
  }
];

// ------------------------------------------------------------
// 设备台账 Mock 数据
// ------------------------------------------------------------
export const devices = [
  {
    id: 9001,
    deptId: 103,
    equipmentNumber: "DEV-1001",
    equipmentName: "1号车间压力表",
    equipmentType: "01",
    equipmentModelLabel: "S909",
    equipmentStatus: "0",
    communicationMeans: "MQTT",
    installationDrawing: "",
    addr: "1号车间东侧",
    locationId: "A-01",
    lowerEquipment: "",
    projectNum: "PRJ-2026-001",
    createTime: "2026-10-20 09:12:33",
    remark: "压力监测"
  },
  {
    id: 9002,
    deptId: 103,
    equipmentNumber: "DEV-1002",
    equipmentName: "2号车间压力表",
    equipmentType: "01",
    equipmentModelLabel: "S909",
    equipmentStatus: "1",
    communicationMeans: "MQTT",
    installationDrawing: "",
    addr: "2号车间西侧",
    locationId: "A-02",
    lowerEquipment: "",
    projectNum: "PRJ-2026-001",
    createTime: "2026-10-20 10:02:11",
    remark: ""
  },
  {
    id: 9003,
    deptId: 104,
    equipmentNumber: "DEV-2001",
    equipmentName: "办公区总水表",
    equipmentType: "02",
    equipmentModelLabel: "水表-01",
    equipmentStatus: "0",
    communicationMeans: "TCP",
    installationDrawing: "",
    addr: "办公楼一层",
    locationId: "B-01",
    lowerEquipment: "",
    projectNum: "PRJ-2026-002",
    createTime: "2026-10-21 08:30:00",
    remark: "用水统计"
  },
  {
    id: 9004,
    deptId: 101,
    equipmentNumber: "DEV-2002",
    equipmentName: "园区总电表",
    equipmentType: "03",
    equipmentModelLabel: "电表-01",
    equipmentStatus: "0",
    communicationMeans: "HTTP",
    installationDrawing: "",
    addr: "配电房",
    locationId: "C-01",
    lowerEquipment: "",
    projectNum: "PRJ-2026-002",
    createTime: "2026-10-21 09:15:20",
    remark: ""
  },
  {
    id: 9005,
    deptId: 108,
    equipmentNumber: "DEV-3001",
    equipmentName: "长沙仓库温湿度计",
    equipmentType: "04",
    equipmentModelLabel: "TH-01",
    equipmentStatus: "2",
    communicationMeans: "TCP",
    installationDrawing: "",
    addr: "长沙仓库",
    locationId: "D-01",
    lowerEquipment: "DEV-3002",
    projectNum: "PRJ-2026-003",
    createTime: "2026-10-22 14:06:45",
    remark: "设备故障待维修"
  },
  {
    id: 9006,
    deptId: 109,
    equipmentNumber: "DEV-3002",
    equipmentName: "长沙仓库水表",
    equipmentType: "02",
    equipmentModelLabel: "水表-02",
    equipmentStatus: "1",
    communicationMeans: "MQTT",
    installationDrawing: "",
    addr: "长沙仓库",
    locationId: "D-02",
    lowerEquipment: "",
    projectNum: "PRJ-2026-003",
    createTime: "2026-10-22 14:20:12",
    remark: ""
  },
  {
    id: 9007,
    deptId: 103,
    equipmentNumber: "DEV-1003",
    equipmentName: "3号车间流量计",
    equipmentType: "05",
    equipmentModelLabel: "FLOW-01",
    equipmentStatus: "0",
    communicationMeans: "MQTT",
    installationDrawing: "",
    addr: "3号车间",
    locationId: "A-03",
    lowerEquipment: "",
    projectNum: "PRJ-2026-001",
    createTime: "2026-10-23 11:11:11",
    remark: ""
  },
  {
    id: 9008,
    deptId: 104,
    equipmentNumber: "DEV-2003",
    equipmentName: "市场部报警器",
    equipmentType: "99",
    equipmentModelLabel: "ALARM-01",
    equipmentStatus: "0",
    communicationMeans: "OTHER",
    installationDrawing: "",
    addr: "市场部办公室",
    locationId: "B-02",
    lowerEquipment: "",
    projectNum: "PRJ-2026-002",
    createTime: "2026-10-24 16:40:00",
    remark: ""
  },
  {
    id: 9009,
    deptId: 108,
    equipmentNumber: "DEV-3003",
    equipmentName: "长沙市场部电表",
    equipmentType: "03",
    equipmentModelLabel: "电表-02",
    equipmentStatus: "3",
    communicationMeans: "SERIAL",
    installationDrawing: "",
    addr: "长沙市场部",
    locationId: "D-03",
    lowerEquipment: "",
    projectNum: "PRJ-2026-003",
    createTime: "2026-10-25 09:00:00",
    remark: "已停用"
  },
  {
    id: 9010,
    deptId: 101,
    equipmentNumber: "DEV-2004",
    equipmentName: "园区压力表",
    equipmentType: "01",
    equipmentModelLabel: "S909",
    equipmentStatus: "0",
    communicationMeans: "PLC",
    installationDrawing: "",
    addr: "园区泵房",
    locationId: "C-02",
    lowerEquipment: "",
    projectNum: "PRJ-2026-002",
    createTime: "2026-10-26 10:10:10",
    remark: ""
  },
  {
    id: 9011,
    deptId: 103,
    equipmentNumber: "DEV-1004",
    equipmentName: "研发部温湿度计",
    equipmentType: "04",
    equipmentModelLabel: "TH-02",
    equipmentStatus: "0",
    communicationMeans: "MQTT",
    installationDrawing: "",
    addr: "研发部实验室",
    locationId: "A-04",
    lowerEquipment: "",
    projectNum: "PRJ-2026-001",
    createTime: "2026-10-27 15:30:22",
    remark: ""
  },
  {
    id: 9012,
    deptId: 102,
    equipmentNumber: "DEV-4001",
    equipmentName: "长沙分公司总表",
    equipmentType: "03",
    equipmentModelLabel: "电表-03",
    equipmentStatus: "1",
    communicationMeans: "HTTP",
    installationDrawing: "",
    addr: "长沙分公司",
    locationId: "E-01",
    lowerEquipment: "",
    projectNum: "PRJ-2026-004",
    createTime: "2026-10-28 08:00:00",
    remark: ""
  }
];

// ------------------------------------------------------------
// 设备数据 Mock 数据
// ------------------------------------------------------------
function mockPayload(deviceNumber, index) {
  return {
    deviceNumber,
    temperature: Number((20 + (index % 8) + Math.random()).toFixed(2)),
    pressure: Number((0.8 + (index % 5) * 0.05).toFixed(3)),
    battery: 60 + (index % 40),
    rssi: -60 - (index % 30),
    status: index % 7 === 0 ? "alarm" : "normal",
    reportSeq: 1000 + index
  };
}

function buildDeviceData() {
  const categories = ["electric", "water", "terminal", "alarm", "wireless"];
  const rows = [];
  devices.forEach((device, deviceIndex) => {
    for (let i = 0; i < 2; i++) {
      const index = deviceIndex * 2 + i;
      const uploadTime = new Date(Date.parse("2026-11-05T17:20:00+08:00") - index * 5 * 60 * 1000);
      rows.push({
        id: 8001 + index,
        equipmentNumber: device.equipmentNumber,
        equipmentName: device.equipmentName,
        deviceCategory: categories[deviceIndex % categories.length],
        uploadTime: formatDateTime(uploadTime),
        batteryLevel: Math.max(5, 100 - (index % 20) * 4),
        signalStrength: -55 - (index % 35),
        alarmStatus: index % 7 === 0 ? 1 : 0,
        payload: mockPayload(device.equipmentNumber, index),
        deptId: device.deptId,
        remark: "",
        createTime: formatDateTime(uploadTime)
      });
    }
  });
  return rows;
}

function pad(n) {
  return n < 10 ? "0" + n : "" + n;
}

function formatDateTime(date) {
  return (
    date.getFullYear() +
    "-" +
    pad(date.getMonth() + 1) +
    "-" +
    pad(date.getDate()) +
    " " +
    pad(date.getHours()) +
    ":" +
    pad(date.getMinutes()) +
    ":" +
    pad(date.getSeconds())
  );
}

export const deviceData = buildDeviceData();

// ------------------------------------------------------------
// 查询 / 增删改（内存操作，刷新页面后恢复初始数据）
// ------------------------------------------------------------
function includes(value, keyword) {
  return String(value || "").toLowerCase().includes(String(keyword || "").toLowerCase());
}

function paginate(list, query) {
  const pageNum = Number(query.pageNum || 1);
  const pageSize = Number(query.pageSize || 10);
  const start = (pageNum - 1) * pageSize;
  return { rows: list.slice(start, start + pageSize), total: list.length };
}

function withDeptName(row) {
  return { ...row, deptName: deptName(row.deptId) };
}

export function queryDevices(query = {}) {
  let list = devices.slice();
  if (query.equipmentNumber) list = list.filter(d => includes(d.equipmentNumber, query.equipmentNumber));
  if (query.equipmentName) list = list.filter(d => includes(d.equipmentName, query.equipmentName));
  if (query.equipmentType) list = list.filter(d => d.equipmentType === query.equipmentType);
  if (query.equipmentStatus) list = list.filter(d => d.equipmentStatus === query.equipmentStatus);
  if (query.communicationMeans) list = list.filter(d => d.communicationMeans === query.communicationMeans);
  if (query.deptId !== undefined && query.deptId !== null && query.deptId !== "") {
    const ids = subtreeIds(query.deptId);
    list = list.filter(d => ids.includes(Number(d.deptId)));
  }
  list.sort((a, b) => String(b.createTime).localeCompare(String(a.createTime)));
  const page = paginate(list, query);
  return { rows: page.rows.map(withDeptName), total: page.total };
}

export function getDevice(id) {
  const row = devices.find(d => String(d.id) === String(id));
  return row ? withDeptName(row) : null;
}

export function saveDevice(payload) {
  if (payload.id !== undefined && payload.id !== null && payload.id !== "") {
    const index = devices.findIndex(d => String(d.id) === String(payload.id));
    if (index >= 0) {
      devices[index] = { ...devices[index], ...payload, id: devices[index].id };
      return true;
    }
    return false;
  }
  const id = 9000 + devices.length + 1;
  devices.unshift({
    ...payload,
    id,
    createTime: formatDateTime(new Date()),
    createBy: "admin"
  });
  return true;
}

export function removeDevice(id) {
  const index = devices.findIndex(d => String(d.id) === String(id));
  if (index >= 0) {
    devices.splice(index, 1);
    return true;
  }
  return false;
}

export function queryDeviceData(query = {}) {
  let list = deviceData.slice();
  if (query.equipmentNumber) list = list.filter(d => includes(d.equipmentNumber, query.equipmentNumber));
  if (query.equipmentName) list = list.filter(d => includes(d.equipmentName, query.equipmentName));
  if (query.deviceCategory) list = list.filter(d => d.deviceCategory === query.deviceCategory);
  if (query.alarmStatus !== undefined && query.alarmStatus !== null && query.alarmStatus !== "") {
    list = list.filter(d => String(d.alarmStatus) === String(query.alarmStatus));
  }
  if (query.deptId !== undefined && query.deptId !== null && query.deptId !== "") {
    const ids = subtreeIds(query.deptId);
    list = list.filter(d => ids.includes(Number(d.deptId)));
  }
  list.sort((a, b) => String(b.uploadTime).localeCompare(String(a.uploadTime)));
  const page = paginate(list, query);
  return { rows: page.rows.map(withDeptName), total: page.total };
}

export function getDeviceData(id) {
  const row = deviceData.find(d => String(d.id) === String(id));
  return row ? withDeptName(row) : null;
}

export function saveDeviceData(payload) {
  if (payload.id !== undefined && payload.id !== null && payload.id !== "") {
    const index = deviceData.findIndex(d => String(d.id) === String(payload.id));
    if (index >= 0) {
      deviceData[index] = { ...deviceData[index], ...payload, id: deviceData[index].id };
      return true;
    }
    return false;
  }
  const id = 8000 + deviceData.length + 1;
  deviceData.unshift({
    ...payload,
    id,
    uploadTime: payload.uploadTime || formatDateTime(new Date()),
    createTime: formatDateTime(new Date())
  });
  return true;
}

export function removeDeviceData(id) {
  const index = deviceData.findIndex(d => String(d.id) === String(id));
  if (index >= 0) {
    deviceData.splice(index, 1);
    return true;
  }
  return false;
}

export function deviceStats() {
  const total = devices.length;
  const online = devices.filter(d => d.equipmentStatus === "0").length;
  return { total, online, offline: total - online };
}
