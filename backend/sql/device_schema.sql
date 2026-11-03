-- ============================================================
-- 设备台账 / 设备数据 表结构与设备状态视图
-- 适用于 PostgreSQL 13+
-- 已有生产库如果已存在同名表，可跳过本文件（本文件使用 IF NOT EXISTS，不会覆盖数据）
-- ============================================================

-- ------------------------------------------------------------
-- 1. 设备台账表
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS "public"."equipment_installation" (
  "id" int8 NOT NULL,
  "create_by" varchar(64),
  "create_time" timestamp(6),
  "update_by" varchar(64),
  "update_time" timestamp(6),
  "dept_id" int8,
  "remark" varchar(500),
  "expand" text,
  "spare1" varchar(500),
  "equipment_number" varchar(100),
  "equipment_name" varchar(200),
  "installation_drawing" varchar(500),
  "equipment_type" varchar(50),
  "dict_value" varchar(100),
  "equipment_model_label" varchar(100),
  "equipment_status" varchar(10),
  "communication_means" varchar(50),
  "addr" varchar(255),
  "location_id" varchar(100),
  "lower_equipment" varchar(100),
  "project_num" varchar(100),
  PRIMARY KEY ("id")
);
COMMENT ON TABLE "public"."equipment_installation" IS '设备台账';
COMMENT ON COLUMN "public"."equipment_installation"."dept_id" IS '归属部门ID（对应 sys_dept.dept_id 的数字型ID）';
COMMENT ON COLUMN "public"."equipment_installation"."equipment_number" IS '设备编号';
COMMENT ON COLUMN "public"."equipment_installation"."equipment_name" IS '设备名称';
COMMENT ON COLUMN "public"."equipment_installation"."equipment_type" IS '设备类型（字典 equipment_type）';
COMMENT ON COLUMN "public"."equipment_installation"."equipment_status" IS '设备状态（字典 equipment_status）';
COMMENT ON COLUMN "public"."equipment_installation"."communication_means" IS '通信方式（字典 communication_means）';

CREATE INDEX IF NOT EXISTS "idx_equipment_installation_dept" ON "public"."equipment_installation" ("dept_id");
CREATE INDEX IF NOT EXISTS "idx_equipment_installation_number" ON "public"."equipment_installation" ("equipment_number");

-- ------------------------------------------------------------
-- 2. 设备数据表（设备上报的时序数据）
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS "public"."equipment_data" (
  "id" int8 NOT NULL,
  "equipment_number" varchar(100),
  "equipment_name" varchar(200),
  "device_category" varchar(50),
  "upload_time" timestamp(6),
  "battery_level" float8,
  "signal_strength" int4,
  "alarm_status" int4,
  "payload" jsonb,
  "dept_id" int4,
  "remark" varchar(500),
  "create_time" timestamp(6),
  PRIMARY KEY ("id")
);
COMMENT ON TABLE "public"."equipment_data" IS '设备数据（时序数据）';
COMMENT ON COLUMN "public"."equipment_data"."device_category" IS '设备分类（字典 device_category）';
COMMENT ON COLUMN "public"."equipment_data"."alarm_status" IS '报警状态（0正常 1报警，字典 alarm_status）';
COMMENT ON COLUMN "public"."equipment_data"."dept_id" IS '归属部门ID（对应 sys_dept.dept_id 的数字型ID）';
COMMENT ON COLUMN "public"."equipment_data"."payload" IS '设备原始数据（JSON）';

CREATE INDEX IF NOT EXISTS "idx_equipment_data_number" ON "public"."equipment_data" ("equipment_number");
CREATE INDEX IF NOT EXISTS "idx_equipment_data_upload_time" ON "public"."equipment_data" ("upload_time");
CREATE INDEX IF NOT EXISTS "idx_equipment_data_dept" ON "public"."equipment_data" ("dept_id");

-- ------------------------------------------------------------
-- 3. 设备最新状态视图
--    每个设备取最新一条上报数据，并按 10 分钟未上报判定为离线
-- ------------------------------------------------------------
CREATE OR REPLACE VIEW "public"."view_device_status" AS
SELECT DISTINCT ON (d."equipment_number")
  d."id",
  d."equipment_number",
  d."equipment_name",
  d."device_category",
  d."upload_time",
  d."battery_level",
  d."signal_strength",
  d."alarm_status",
  d."payload",
  d."dept_id",
  d."remark",
  d."create_time",
  CASE
    WHEN d."upload_time" IS NOT NULL
         AND d."upload_time" >= (now() - interval '10 minutes') THEN '在线'
    ELSE '离线'
  END AS "online_state_calc",
  d."upload_time" AS "update_time"
FROM "public"."equipment_data" d
ORDER BY d."equipment_number", d."upload_time" DESC NULLS LAST;
COMMENT ON VIEW "public"."view_device_status" IS '设备最新状态视图（10分钟内有上报为在线）';
