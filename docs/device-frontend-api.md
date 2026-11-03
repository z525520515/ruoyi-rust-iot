# 设备台账 / 设备数据 前后端对接设计

本文档说明新增的「设备管理」模块（设备台账、设备数据）与后端接口的对接方式、字段映射、
部门权限设计与部署步骤。

> 现有前端是 RuoYi-Vue3 的后台框架，菜单由后端 `sys_menu` 动态下发、路由组件按
> `component` 字段从 `src/views/**` 自动加载，因此新增页面只需要：
> 1）在 `src/views/device/**` 下新增页面；2）在 `sys_menu` 增加菜单与按钮权限；
> 3）请求接口与后端保持一致。

## 1. 页面设计

### 1.1 设备台账（`src/views/device/installation/index.vue`）

- 布局：左侧部门树（`/system/dept/treeselect`）+ 右侧列表，沿用系统用户管理页的树 + 列表风格
- 搜索：设备编号、设备名称、设备类型、设备状态、通信方式（`el-form :inline` 查询区）
- 列表：设备编号、设备名称、归属部门（`deptName`）、设备类型、型号标签、设备状态、
  通信方式、项目编号、创建时间、操作列
- 操作：新增 / 修改 / 删除 / 导入 / 下载导入模板
- 表单：设备编号、设备名称、**归属部门（部门树选择）**、设备类型、型号标签、设备状态、
  通信方式、安装图纸、安装地址、位置ID、下级设备、项目编号、备注
- 权限按钮使用 `v-hasPermi`，权限码与后端 `has_permit` 对应

### 1.2 设备数据（`src/views/device/data/index.vue`）

- 顶部统计卡片：设备总数 / 在线 / 离线（`/equipment_data/daum/stats`）
- 布局：左侧部门树 + 右侧列表，与设备台账一致
- 搜索：设备编号、设备名称、设备分类、报警状态
- 列表：设备编号、设备名称、归属部门、设备分类、电量、信号强度、报警状态、上报时间、操作
- 操作：详情（展示原始 JSON 数据）、修改、删除、新增
- 表单：设备编号、设备名称、**归属部门**、设备分类、电量、信号强度、报警状态、
  数据内容（JSON 文本，提交前校验并转对象）、备注

页面样式全部使用现有 Element Plus + RuoYi 样式类（`app-container`、`mb8`、
`head-container`、`dict-tag`、`right-toolbar`、`pagination`），不引入新的 UI 风格。

## 2. 接口清单

统一前缀：`VITE_APP_BASE_API`（开发环境 `/dev-api`，由 Vite 代理到后端 `127.0.0.1:60800`）。
统一响应：分页 `{ code: 200, rows: [], total: n }`，非分页 `{ code: 200, data: ... }`，
失败 `code != 200` 时由前端请求拦截器统一提示。

### 2.1 设备台账

| 方法 | 路径 | 权限码 | 说明 |
| --- | --- | --- | --- |
| GET | `/equipment/installation/list` | `equipment:installation:query` | 分页查询 |
| GET | `/equipment/installation/{id}` | `equipment:installation:query` | 详情 |
| POST | `/equipment/installation` | `equipment:installation:add` | 新增 |
| PUT | `/equipment/installation` | `equipment:installation:edit` | 修改 |
| DELETE | `/equipment/installation/{id}` | `equipment:installation:remove` | 删除 |
| GET | `/equipment/export` | `equipment:installation:export` | 下载导入模板（xlsx 二进制） |
| POST | `/equipment/import` | `equipment:installation:import` | 导入 Excel（multipart，字段 `file`） |

分页参数（camelCase，与后端 `EquipmentInstallationPageDTO` 一致）：

```
pageNum, pageSize,
equipmentNumber(模糊), equipmentName(模糊), equipmentType(精确),
equipmentStatus(精确), communicationMeans(精确), projectNum(模糊),
deptId(数字型部门ID，含下级部门)
```

新增/修改字段（`EquipmentInstallationAddDTO` / `EquipmentInstallationUpdateDTO`）：

```
deptId, remark, expand, spare1,
equipmentNumber, equipmentName, installationDrawing, equipmentType, dictValue,
equipmentModelLabel, equipmentStatus, communicationMeans, addr, locationId,
lowerEquipment, projectNum
```

列表行（`EquipmentInstallationVo`）：

```
id, deptId, deptName, createBy, createTime, updateBy, updateTime,
equipmentNumber, equipmentName, equipmentType, equipmentModelLabel,
equipmentStatus, communicationMeans, installationDrawing, addr, locationId,
lowerEquipment, projectNum, remark
```

### 2.2 设备数据

| 方法 | 路径 | 权限码 | 说明 |
| --- | --- | --- | --- |
| GET | `/equipment_data/daum/list` | `equipment_data:daum:query` | 分页查询 |
| GET | `/equipment_data/daum/{id}` | `equipment_data:daum:query` | 详情 |
| POST | `/equipment_data/daum` | `equipment_data:daum:add` | 新增 |
| PUT | `/equipment_data/daum` | `equipment_data:daum:edit` | 修改 |
| DELETE | `/equipment_data/daum/{id}` | `equipment_data:daum:remove` | 删除 |
| GET | `/equipment_data/daum/stats` | `equipment_data:daum:query` | 总数/在线/离线统计 |

分页参数（`EquipmentDaumPageDTO`）：

```
pageNum, pageSize,
equipmentNumber(模糊), equipmentName(模糊), deviceCategory(精确),
alarmStatus(整型), deptId(数字型部门ID，含下级部门)
```

列表行（`EquipmentDaumVo`）：

```
id, equipmentNumber, equipmentName, deviceCategory, uploadTime, batteryLevel,
signalStrength, alarmStatus, payload, deptId, deptName, remark, createTime
```

> `payload` 为设备原始 JSON 数据；前端详情中对字符串/对象两种形式都做了格式化兼容。

### 2.3 部门树

| 方法 | 路径 | 权限 | 说明 |
| --- | --- | --- | --- |
| GET | `/system/dept/treeselect` | 登录即可 | 返回 `[{ id, label, children }]` |

该接口供设备页面左侧部门树、表单归属部门选择使用，不要求 `system:dept:query` 权限，
避免只具备设备权限的角色打不开部门树。

## 3. 字段映射

| 前端表单字段 | 后端 DTO | 设备台账表字段 |
| --- | --- | --- |
| deptId | deptId（i64） | dept_id（int8） |
| equipmentNumber | equipmentNumber | equipment_number |
| equipmentName | equipmentName | equipment_name |
| equipmentType | equipmentType | equipment_type |
| equipmentModelLabel | equipmentModelLabel | equipment_model_label |
| equipmentStatus | equipmentStatus | equipment_status |
| communicationMeans | communicationMeans | communication_means |
| installationDrawing | installationDrawing | installation_drawing |
| addr / locationId / lowerEquipment / projectNum | 同名 | addr / location_id / lower_equipment / project_num |
| remark | remark | remark |

设备数据字段：`equipmentNumber / equipmentName / deviceCategory / batteryLevel /
signalStrength / alarmStatus / payload / deptId / remark` 一一对应。

## 4. 部门权限设计

### 4.1 页面层

- 归属部门是设备台账/设备数据的必填/可筛选参数，表单通过部门树选择后以数字型
  `deptId` 提交（后端字段为 `i64 / i32`）。
- 左侧部门树点击后，`deptId` 作为查询条件传给后端，后端按「本部门 + 下级部门」过滤。

### 4.2 后端数据权限

后端新增 `backend/src/util/data_scope.rs`，在列表、详情、修改、删除时应用：

| `sys_role.data_scope` | 含义 | 设备模块行为 |
| --- | --- | --- |
| `1` | 全部数据 | 不限制 |
| `2` | 自定义数据 | 按当前用户部门的「本部门及以下」处理（简化实现，见下方限制） |
| `3` | 本部门数据 | 按当前用户部门过滤 |
| `4` | 本部门及以下 | 按当前用户部门 + 全部下级部门过滤 |
| `5` | 仅本人数据 | 设备台账按 `create_by` 过滤；设备数据没有创建人字段，按本部门过滤 |

- `admin` 用户（`user_name = "admin"`）与未配置角色的用户不限制。
- 实现方式：`web_data` 保存登录用户的 `UserCache`（含 `user.dept_id` 与角色
  `data_scope`），`data_scope.rs` 根据 `sys_dept.ancestors` 计算部门子树 ID，
  在分页 SQL 中追加 `and dept_id in (?, ...)`；详情/修改/删除额外做单条记录校验。
- 同时支持前端传入的 `deptId` 精确筛选（两个条件取交集）。

### 4.3 限制与后续

- 设备表 `dept_id` 为数字类型，因此只支持 **数字型部门 ID**（RuoYi 默认演示部门
  `100/101/...` 均满足）；如系统后续使用 ObjectId 型部门 ID，需要把设备表
  `dept_id` 改为 `varchar` 并同步调整 DTO/VO。
- `data_scope = 2`（自定义部门）目前简化为「本部门及以下」；如需精确到
  `sys_role_dept`，可在 `current_scope()` 中增加一次角色-部门查询。
- “仅本人”对设备数据退化为本部门（设备数据由 MQTT 上报，没有创建人字段）。

## 5. 菜单、权限、字典

执行 `backend/sql/device_menu.sql`（可重复执行）：

- 目录：设备管理（`device`，图标 `server`）
- 页面：设备台账（路由 `/device/installation`，组件 `device/installation/index`）、
  设备数据（路由 `/device/deviceData`，组件 `device/data/index`）
- 按钮权限：`equipment:installation:*`、`equipment_data:daum:*`
- 角色：默认给超级管理员角色（role_id = `1`）授权；其他角色请在「角色管理」中勾选
- 字典：`device_category`、`alarm_status`、`equipment_status`、`equipment_type`、
  `communication_means`

> 菜单来自后端缓存（Redis key `sys_menu:all`），执行 SQL 后需重启后端或清除该缓存。

## 6. 部署步骤

```bash
# 1. 初始化/升级数据库（已有表可跳过 schema）
psql -U postgres -d iot_core -f backend/sql/device_schema.sql
psql -U postgres -d iot_core -f backend/sql/device_menu.sql

# 2. 启动后端（会自动加载新的部门树接口与数据权限逻辑）
cd backend && cargo run --bin ruoyi_rust_dev

# 3. 启动前端
cd ruoyi-vue3-rust-master && npm install && npm run dev
```

登录后即可在左侧看到「设备管理 → 设备台账 / 设备数据」。
