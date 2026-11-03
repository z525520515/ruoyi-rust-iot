-- ============================================================
-- 设备管理模块：菜单 / 按钮权限 / 数据字典
-- 执行后需要重启后端（或删除 redis 中的 sys_menu:all 缓存）才能看到新菜单
-- ============================================================

-- ------------------------------------------------------------
-- 1. 菜单（目录 + 页面 + 按钮权限）
--    权限码与后端接口 has_permit 保持一致
-- ------------------------------------------------------------
INSERT INTO "public"."sys_menu"
  ("menu_id", "menu_name", "parent_id", "order_num", "path", "component", "query", "is_frame", "is_cache", "menu_type", "visible", "status", "perms", "icon", "create_by", "create_time", "update_by", "update_time", "remark")
VALUES
  (2100, '设备管理', 0,    2, 'device',       NULL,                          '', '1', '0', 'M', '0', '0', '',                             'server', 'admin', now(), NULL, NULL, '设备管理目录'),
  (2101, '设备台账', 2100, 1, 'installation', 'device/installation/index',   '', '1', '0', 'C', '0', '0', 'equipment:installation:query',  'list',   'admin', now(), NULL, NULL, '设备台账菜单'),
  (2102, '设备数据', 2100, 2, 'deviceData',   'device/data/index',           '', '1', '0', 'C', '0', '0', 'equipment_data:daum:query',     'chart',  'admin', now(), NULL, NULL, '设备数据菜单'),
  -- 设备台账按钮权限
  (2103, '设备台账查询', 2101, 1, '', '', '', '1', '0', 'F', '0', '0', 'equipment:installation:query',  '#', 'admin', now(), NULL, NULL, ''),
  (2104, '设备台账新增', 2101, 2, '', '', '', '1', '0', 'F', '0', '0', 'equipment:installation:add',    '#', 'admin', now(), NULL, NULL, ''),
  (2105, '设备台账修改', 2101, 3, '', '', '', '1', '0', 'F', '0', '0', 'equipment:installation:edit',   '#', 'admin', now(), NULL, NULL, ''),
  (2106, '设备台账删除', 2101, 4, '', '', '', '1', '0', 'F', '0', '0', 'equipment:installation:remove', '#', 'admin', now(), NULL, NULL, ''),
  (2107, '设备台账导入', 2101, 5, '', '', '', '1', '0', 'F', '0', '0', 'equipment:installation:import', '#', 'admin', now(), NULL, NULL, ''),
  (2108, '设备台账导出', 2101, 6, '', '', '', '1', '0', 'F', '0', '0', 'equipment:installation:export', '#', 'admin', now(), NULL, NULL, ''),
  -- 设备数据按钮权限
  (2109, '设备数据查询', 2102, 1, '', '', '', '1', '0', 'F', '0', '0', 'equipment_data:daum:query',  '#', 'admin', now(), NULL, NULL, ''),
  (2110, '设备数据新增', 2102, 2, '', '', '', '1', '0', 'F', '0', '0', 'equipment_data:daum:add',    '#', 'admin', now(), NULL, NULL, ''),
  (2111, '设备数据修改', 2102, 3, '', '', '', '1', '0', 'F', '0', '0', 'equipment_data:daum:edit',   '#', 'admin', now(), NULL, NULL, ''),
  (2112, '设备数据删除', 2102, 4, '', '', '', '1', '0', 'F', '0', '0', 'equipment_data:daum:remove', '#', 'admin', now(), NULL, NULL, '')
ON CONFLICT DO NOTHING;

-- 超级管理员角色默认拥有全部菜单（admin 用户本身在代码中已放行，这里补全角色-菜单关系）
INSERT INTO "public"."sys_role_menu" ("role_id", "menu_id")
VALUES
  ('1', 2100), ('1', 2101), ('1', 2102),
  ('1', 2103), ('1', 2104), ('1', 2105), ('1', 2106), ('1', 2107), ('1', 2108),
  ('1', 2109), ('1', 2110), ('1', 2111), ('1', 2112)
ON CONFLICT DO NOTHING;

-- 如需给普通角色授权，可在「系统管理 -> 角色管理 -> 修改 -> 菜单权限」中勾选，
-- 或参考下面语句（role_id 替换为实际角色ID）：
-- INSERT INTO "public"."sys_role_menu" ("role_id", "menu_id") VALUES
--   ('63d8c82a00e2c21200979b90', 2100), ('63d8c82a00e2c21200979b90', 2101), ('63d8c82a00e2c21200979b90', 2103)
-- ON CONFLICT DO NOTHING;

-- ------------------------------------------------------------
-- 2. 数据字典
-- ------------------------------------------------------------
INSERT INTO "public"."sys_dict_type"
  ("dict_id", "dict_name", "dict_type", "status", "create_by", "create_time", "update_by", "update_time", "remark")
VALUES
  ('9001', '设备分类', 'device_category',     '0', 'admin', now(), NULL, NULL, '设备数据分类'),
  ('9002', '报警状态', 'alarm_status',        '0', 'admin', now(), NULL, NULL, '设备报警状态'),
  ('9003', '设备状态', 'equipment_status',    '0', 'admin', now(), NULL, NULL, '设备台账状态'),
  ('9004', '设备类型', 'equipment_type',      '0', 'admin', now(), NULL, NULL, '设备类型'),
  ('9005', '通信方式', 'communication_means', '0', 'admin', now(), NULL, NULL, '设备通信方式')
ON CONFLICT DO NOTHING;

INSERT INTO "public"."sys_dict_data"
  ("dict_code", "dict_sort", "dict_label", "dict_value", "dict_type", "css_class", "list_class", "is_default", "status", "create_by", "create_time", "update_by", "update_time", "remark")
VALUES
  -- 设备分类 device_category
  ('910001', 1, '电表',     'electric', 'device_category', '', 'default', 'Y', '0', 'admin', now(), NULL, NULL, ''),
  ('910002', 2, '水表',     'water',    'device_category', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910003', 3, '终端',     'terminal', 'device_category', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910004', 4, '报警器',   'alarm',    'device_category', '', 'danger',  'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910005', 5, '无线终端', 'wireless', 'device_category', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  -- 报警状态 alarm_status
  ('910011', 1, '正常', '0', 'alarm_status', '', 'success', 'Y', '0', 'admin', now(), NULL, NULL, ''),
  ('910012', 2, '报警', '1', 'alarm_status', '', 'danger',  'N', '0', 'admin', now(), NULL, NULL, ''),
  -- 设备状态 equipment_status
  ('910021', 1, '在线', '0', 'equipment_status', '', 'success', 'Y', '0', 'admin', now(), NULL, NULL, ''),
  ('910022', 2, '离线', '1', 'equipment_status', '', 'info',    'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910023', 3, '故障', '2', 'equipment_status', '', 'danger',  'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910024', 4, '停用', '3', 'equipment_status', '', 'warning', 'N', '0', 'admin', now(), NULL, NULL, ''),
  -- 设备类型 equipment_type
  ('910031', 1, '压力表', '01', 'equipment_type', '', 'default', 'Y', '0', 'admin', now(), NULL, NULL, ''),
  ('910032', 2, '水表',   '02', 'equipment_type', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910033', 3, '电表',   '03', 'equipment_type', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910034', 4, '温度计', '04', 'equipment_type', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910035', 5, '流量计', '05', 'equipment_type', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910036', 9, '其他',   '99', 'equipment_type', '', 'info',    'N', '0', 'admin', now(), NULL, NULL, ''),
  -- 通信方式 communication_means
  ('910041', 1, 'MQTT',   'MQTT',   'communication_means', '', 'default', 'Y', '0', 'admin', now(), NULL, NULL, ''),
  ('910042', 2, 'TCP',    'TCP',    'communication_means', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910043', 3, 'HTTP',   'HTTP',   'communication_means', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910044', 4, '串口',   'SERIAL', 'communication_means', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910045', 5, 'PLC',    'PLC',    'communication_means', '', 'default', 'N', '0', 'admin', now(), NULL, NULL, ''),
  ('910046', 6, '其他',   'OTHER',  'communication_means', '', 'info',    'N', '0', 'admin', now(), NULL, NULL, '')
ON CONFLICT DO NOTHING;
