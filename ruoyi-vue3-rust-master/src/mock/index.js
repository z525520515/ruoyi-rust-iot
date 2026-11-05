/**
 * 纯前端 Mock 适配器（GitHub Pages 演示用）
 *
 * 启用条件：构建/运行时 VITE_APP_DEMO=true
 * 作用：拦截 axios 请求，用浏览器内存数据模拟后端接口，不需要部署 Rust 后端。
 */
import {
  DEMO_ACCOUNT,
  demoUser,
  demoRoles,
  demoPermissions,
  demoRouters,
  deptTree,
  dicts,
  queryDevices,
  getDevice,
  saveDevice,
  removeDevice,
  queryDeviceData,
  getDeviceData,
  saveDeviceData,
  removeDeviceData,
  deviceStats
} from "./data";

export const isDemoMode = import.meta.env.VITE_APP_DEMO === "true";

function makeResponse(data, config) {
  return Promise.resolve({
    data,
    status: 200,
    statusText: "OK",
    headers: { "content-type": "application/json; charset=utf-8" },
    config,
    request: {}
  });
}

function parseBody(config) {
  const data = config.data;
  if (data === undefined || data === null || data === "") return {};
  if (typeof data === "string") {
    try {
      return JSON.parse(data);
    } catch (e) {
      return {};
    }
  }
  return data;
}

function parseQuery(url) {
  const search = url.split("?")[1];
  if (!search) return {};
  const query = {};
  new URLSearchParams(search).forEach((value, key) => {
    query[key] = value;
  });
  return query;
}

/**
 * 返回一个 axios adapter：命中 Mock 路由返回内存数据，未命中返回 501 提示。
 */
export function setupMockAdapter() {
  return function mockAdapter(config) {
    const rawUrl = config.url || "";
    const path = rawUrl.split("?")[0];
    const query = parseQuery(rawUrl);
    const method = (config.method || "get").toLowerCase();
    const body = parseBody(config);

    // ---------- 登录 / 用户 / 菜单 ----------
    if (path === "/login" && method === "post") {
      const ok = body.username === DEMO_ACCOUNT.username && body.password === DEMO_ACCOUNT.password;
      if (ok) {
        return makeResponse(
          { code: 200, msg: "登录成功", token: "demo-token-" + Date.now() },
          config
        );
      }
      return makeResponse(
        { code: 500, msg: "演示环境请使用 admin / admin123 登录" },
        config
      );
    }
    if (path === "/captchaImage") {
      return makeResponse({ code: 200, captchaEnabled: false, img: "", uuid: "" }, config);
    }
    if (path === "/logout") {
      return makeResponse({ code: 200, msg: "退出成功" }, config);
    }
    if (path === "/getInfo") {
      return makeResponse(
        { code: 200, user: demoUser, roles: demoRoles, permissions: demoPermissions },
        config
      );
    }
    if (path === "/getRouters") {
      return makeResponse({ code: 200, data: demoRouters }, config);
    }

    // ---------- 字典 / 部门树 ----------
    if (path.startsWith("/system/dict/data/type/")) {
      const dictType = path.split("/").pop();
      return makeResponse({ code: 200, data: dicts[dictType] || [] }, config);
    }
    if (path === "/system/dept/treeselect") {
      return makeResponse({ code: 200, data: deptTree }, config);
    }

    // ---------- 设备台账 ----------
    if (path === "/equipment/installation/list" && method === "get") {
      const { rows, total } = queryDevices(query);
      return makeResponse({ code: 200, rows, total }, config);
    }
    if (path === "/equipment/status/list" && method === "get") {
      const { rows, total } = queryDeviceData(query);
      return makeResponse({ code: 200, rows, total }, config);
    }
    if (path === "/equipment/installation" && (method === "post" || method === "put")) {
      const ok = saveDevice(body);
      return makeResponse({ code: ok ? 200 : 500, msg: ok ? "保存成功" : "保存失败" }, config);
    }
    const installationMatch = path.match(/^\/equipment\/installation\/([^/]+)$/);
    if (installationMatch) {
      const id = decodeURIComponent(installationMatch[1]);
      if (method === "get") {
        const row = getDevice(id);
        return makeResponse(
          row ? { code: 200, data: row } : { code: 500, msg: "设备不存在" },
          config
        );
      }
      if (method === "delete") {
        const ok = removeDevice(id);
        return makeResponse({ code: ok ? 200 : 500, msg: ok ? "删除成功" : "删除失败" }, config);
      }
    }
    if (path === "/equipment/export") {
      return makeResponse({ code: 500, msg: "演示模式不支持下载导入模板" }, config);
    }
    if (path === "/equipment/import") {
      return makeResponse({ code: 500, msg: "演示模式不支持导入" }, config);
    }

    // ---------- 设备数据 ----------
    if (path === "/equipment_data/daum/list" && method === "get") {
      const { rows, total } = queryDeviceData(query);
      return makeResponse({ code: 200, rows, total }, config);
    }
    if (path === "/equipment_data/daum/stats" && method === "get") {
      return makeResponse({ code: 200, data: deviceStats() }, config);
    }
    if (path === "/equipment_data/daum" && (method === "post" || method === "put")) {
      const ok = saveDeviceData(body);
      return makeResponse({ code: ok ? 200 : 500, msg: ok ? "保存成功" : "保存失败" }, config);
    }
    const deviceDataMatch = path.match(/^\/equipment_data\/daum\/([^/]+)$/);
    if (deviceDataMatch) {
      const id = decodeURIComponent(deviceDataMatch[1]);
      if (method === "get") {
        const row = getDeviceData(id);
        return makeResponse(
          row ? { code: 200, data: row } : { code: 500, msg: "设备数据不存在" },
          config
        );
      }
      if (method === "delete") {
        const ok = removeDeviceData(id);
        return makeResponse({ code: ok ? 200 : 500, msg: ok ? "删除成功" : "删除失败" }, config);
      }
    }

    // ---------- 未模拟接口 ----------
    return makeResponse(
      { code: 501, msg: "演示模式：接口未模拟 " + method.toUpperCase() + " " + path },
      config
    );
  };
}
