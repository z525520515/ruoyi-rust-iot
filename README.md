# ruoyi-rust-iot

**Rust + Actix-Web + Vue3 全栈后台管理框架**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> 仓库地址：https://github.com/z525520515/ruoyi-rust-iot

一个基于 Rust 后端（Actix-Web / Rbatis / PostgreSQL / Redis / MQTT）与 Vue3 前端（RuoYi-Vue3）
二次开发的全栈后台管理框架，包含用户与权限体系、系统监控、代码生成，以及设备接入、MQTT
数据解析、PLC 网关等物联网方向扩展示例。

> 本项目是在 RuoYi-Vue3（前端）与 rust-ruoyi / ruoyi-rust（后端）两个开源项目基础上的工程实践，
> 已做敏感信息清理。适合作为学习、二次开发的工程起点；生产使用前请自行完成安全加固与合规评估。

## 特性

- **认证与权限**：JWT 认证、菜单/按钮权限、角色、部门数据权限
- **系统管理**：用户、角色、菜单、部门、岗位、字典、参数、通知公告
- **系统监控**：在线用户、定时任务、服务与缓存监控、登录日志
- **代码生成**：根据数据库表生成后端与前端代码（`codegen/`）
- **文件上传、WebSocket** 通讯示例
- **设备管理**：设备台账（含归属部门与部门数据权限）、设备数据（MQTT 上报数据、在线统计）
- **物联网扩展示例**（可选 feature `plc_gateway`）：设备管理、MQTT 数据解析、TCP/PLC 网关、
  远程运维（OTA、日志、重启、网络配置）
- **前端**：RuoYi-Vue3 后台界面（Vue3 + Vite + Element Plus）

## 技术栈

| 层 | 技术 |
| --- | --- |
| 后端语言 | Rust (stable) |
| Web 框架 | Actix-Web 4 + Axum（网关模块） |
| ORM / 数据库 | Rbatis、SeaORM/SQLx、PostgreSQL 13+ |
| 缓存 | Redis / 内存缓存 |
| 消息 | MQTT（rumqttc）、WebSocket |
| 前端 | Vue 3、Vite 5、Element Plus、Pinia、Vue Router、ECharts |

## 目录结构

```
backend/                    Rust 后端（Actix-Web）
  application.yml           基础配置（敏感项使用环境变量覆盖）
  application-{dev,prod}.yml
  postgresql.sql            PostgreSQL 初始化脚本
  sql/                      设备模块表结构与菜单/字典脚本
  core/dev_protocols/       设备协议解析（可选）
  src/modules/plc_gateway/  PLC / MQTT 网关与远程运维（feature: plc_gateway）
codegen/                    代码生成器
docs/                       设计与接口文档
ruoyi-vue3-rust-master/     前端（RuoYi-Vue3 v3.8.6 二次开发副本）
```

## 快速开始

### 1. 依赖

- Rust stable（建议最新稳定版）
- Node.js 18+
- PostgreSQL 13+
- Redis（`cache_type: "redis"` 时需要，也可改为 `"mem"`）

### 2. 初始化数据库

```bash
createdb iot_core   # 或使用现有库名，并同步修改 APP_DATABASE_URL
psql -U postgres -d iot_core -f backend/postgresql.sql
# 设备台账 / 设备数据相关表、菜单、字典（可重复执行）
psql -U postgres -d iot_core -f backend/sql/device_schema.sql
psql -U postgres -d iot_core -f backend/sql/device_menu.sql
```

设备模块的接口与页面设计见 [docs/device-frontend-api.md](docs/device-frontend-api.md)。

### 3. 启动后端

敏感配置均通过环境变量注入，仓库内不保存真实凭据：

```bash
export APP_DATABASE_URL="postgresql://postgres:YOUR_PASSWORD@127.0.0.1:5432/iot_core?schema=public"
export APP_REDIS_URL="redis://127.0.0.1:6379"
export APP_JWT_SECRET="请替换为一段足够长的随机字符串"
# 如需 MQTT：
export APP_MQTT_HOST="127.0.0.1" APP_MQTT_PORT="1883"
export APP_MQTT_CLIENT_ID="iot_core" APP_MQTT_USERNAME="mqtt_user" APP_MQTT_PASSWORD="YOUR_PASSWORD"

cd backend
cargo run --bin ruoyi_rust_dev      # 或 cargo run --bin ruoyi_rust_prod
```

默认监听 `0.0.0.0:60800`。

### 4. 启动前端

```bash
cd ruoyi-vue3-rust-master
npm install
npm run dev
```

开发服务器会把 `/dev-api` 代理到 `http://127.0.0.1:60800`（见 `vite.config.js`）。

### 5. 默认账号

```
用户名：admin
密码：  admin123
```

> 首次部署后请立即修改默认密码，并替换 `APP_JWT_SECRET`。

## 前端 Mock 演示站（物联网平台 · GitHub Pages）

仓库内置了**纯前端 Mock 演示模式**，不需要部署 Rust 后端和数据库，可直接通过 GitHub Pages 访问：

> 演示地址：`https://<你的GitHub用户名>.github.io/ruoyi-rust-iot/`
>
> 演示账号：`admin / admin123`（前端 Mock 校验，不请求后端）

说明：

- 直接访问首页会自动以演示账号登录，无需手动登录；
- 设备台账、设备数据使用浏览器内存中的 Mock 数据，新增/修改/删除仅在当前页面内存生效，刷新后重置；
- 演示模式会隐藏 Excel 导入/下载模板按钮；
- 演示模式下点击“退出登录”会自动重新以演示账号登录（演示站保持可访问的预期行为）；
- 本地体验：`cd ruoyi-vue3-rust-master && npm install && npm run dev:demo`
- 本地构建：`npm run build:demo`（产物在 `ruoyi-vue3-rust-master/dist/`）
- 自动化部署：`.github/workflows/deploy-pages.yml`，推送到 `main` 后自动构建并发布到 GitHub Pages

前置条件：

1. GitHub Pages 对私有仓库需要 GitHub Pro/Team，免费账号请先将仓库设为 **public**；
2. 仓库 **Settings → Pages → Build and deployment → Source** 选择 **GitHub Actions**；
3. 如果修改了仓库名，无需改代码，工作流会自动使用 `/<仓库名>/` 作为前端 base 路径。

Mock 数据与接口模拟代码位于 `ruoyi-vue3-rust-master/src/mock/`，通过 `VITE_APP_DEMO=true` 启用，不影响真实后端联调。

## 配置说明

`backend/application.yml` 中仅保留占位符，运行时可用环境变量覆盖：

| 环境变量 | 说明 |
| --- | --- |
| `APP_DATABASE_URL` | PostgreSQL 连接串 |
| `APP_REDIS_URL` | Redis 连接串 |
| `APP_JWT_SECRET` | JWT 签名密钥（务必替换） |
| `APP_MQTT_HOST` / `APP_MQTT_PORT` | MQTT Broker 地址与端口 |
| `APP_MQTT_CLIENT_ID` / `APP_MQTT_USERNAME` / `APP_MQTT_PASSWORD` | MQTT 客户端配置 |
| `DATABASE_URL` | `codegen` 使用的数据库连接串 |

## 构建

```bash
# 后端
cd backend
cargo build --release

# 网关功能（可选）
cargo build --release --features plc_gateway

# 前端
cd ruoyi-vue3-rust-master
npm run build:prod
```

## 代码生成

```bash
cd codegen
DATABASE_URL="postgres://postgres:YOUR_PASSWORD@127.0.0.1:5432/iot_core" cargo run
```

按 `codegen/src/main.rs` 中的模块名、表名配置生成后端与前端代码。

## 设备接入示例（可选）

MQTT 订阅与设备协议解析、PLC 配置 API、远程运维接口等示例位于
`backend/src/modules/plc_gateway/` 与 `backend/core/dev_protocols/`，
设备协议说明见 `backend/docs/linker-protocol.md`。

> 注意：设备/网关协议相关资料可能涉及第三方厂商权利，公开使用或再分发前请自行确认授权范围。

## 许可证

本项目采用 [MIT License](LICENSE)。

前端、后端上游项目及第三方依赖的版权归属与许可证见 [NOTICE](NOTICE)。

## 致谢

- [RuoYi / RuoYi-Vue3](https://github.com/yangzongzhuan/RuoYi-Vue3) — 前端与管理后台基础
- [rust-ruoyi / ruoyi-rust](https://gitee.com/wizount/ruoyi-rust) — Rust 后端框架基础
- Rust 生态中的 Actix-Web、Rbatis、Tokio、rumqttc 等优秀开源项目

## 贡献

欢迎提交 Issue 与 Pull Request。提交前请确认：

1. 不包含任何真实凭据、内网地址、客户数据或第三方受限资料；
2. 新增代码可编译、通过现有检查；
3. 遵循现有代码风格与目录结构。

## 免责声明

本项目为学习与工程实践用途的示例工程，未经过完整的安全审计与生产验证。
使用者需自行评估并承担部署、合规与安全责任。
