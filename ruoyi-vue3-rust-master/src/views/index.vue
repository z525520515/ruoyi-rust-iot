<template>
  <div class="app-container dashboard" v-loading="loading">
    <!-- 顶部：工作台欢迎区 -->
    <section class="hero">
      <div class="hero-main">
        <div class="hero-kicker">物联网平台 · 设备运营工作台</div>
        <h1 class="hero-title">{{ greeting }}，{{ userName }}</h1>
        <p class="hero-sub">
          {{ today }} · 已接入 {{ kpi.total }} 台设备，其中在线 {{ kpi.online }} 台、离线 {{ kpi.offline }} 台
        </p>
        <div class="hero-actions">
          <el-button type="primary" @click="goPage('/device/installation')">设备台账</el-button>
          <el-button @click="goPage('/device/deviceData')">设备数据</el-button>
          <el-button text class="hero-refresh" @click="loadData">
            <el-icon><Refresh /></el-icon>
            刷新数据
          </el-button>
        </div>
      </div>
      <div class="hero-rate">
        <el-progress
          type="circle"
          :percentage="onlineRate"
          :width="112"
          :stroke-width="8"
          :color="rateColor"
        />
        <div class="hero-rate-label">设备在线率</div>
      </div>
    </section>

    <!-- 核心指标 -->
    <el-row :gutter="16" class="kpi-row">
      <el-col v-for="item in kpis" :key="item.key" :xs="12" :sm="12" :md="6">
        <div class="kpi-card">
          <div class="kpi-icon" :style="{ color: item.color, background: item.bg }">
            <el-icon :size="22"><component :is="item.icon" /></el-icon>
          </div>
          <div class="kpi-body">
            <div class="kpi-value">
              {{ item.value }}<span class="kpi-unit">{{ item.unit }}</span>
            </div>
            <div class="kpi-label">{{ item.label }}</div>
          </div>
        </div>
      </el-col>
    </el-row>

    <!-- 在线状态 + 设备类型分布 -->
    <el-row :gutter="16">
      <el-col :xs="24" :md="10">
        <div class="panel">
          <div class="panel-head">
            <span class="panel-title">设备在线状态</span>
            <span class="panel-hint">实时统计</span>
          </div>
          <div ref="statusRef" class="chart chart-sm"></div>
        </div>
      </el-col>
      <el-col :xs="24" :md="14">
        <div class="panel">
          <div class="panel-head">
            <span class="panel-title">设备类型分布</span>
            <span class="panel-hint">共 {{ kpi.total }} 台</span>
          </div>
          <div ref="typeRef" class="chart chart-sm"></div>
        </div>
      </el-col>
    </el-row>

    <!-- 部门分布 + 最近告警 -->
    <el-row :gutter="16">
      <el-col :xs="24" :md="12">
        <div class="panel">
          <div class="panel-head">
            <span class="panel-title">部门设备分布</span>
            <span class="panel-hint">Top 8</span>
          </div>
          <div ref="deptRef" class="chart chart-md"></div>
        </div>
      </el-col>
      <el-col :xs="24" :md="12">
        <div class="panel">
          <div class="panel-head">
            <span class="panel-title">最近告警</span>
            <el-link type="primary" :underline="false" @click="goPage('/device/deviceData')">
              查看全部
            </el-link>
          </div>
          <el-table :data="recentAlarms" size="small" class="mini-table" empty-text="暂无告警数据">
            <el-table-column label="设备" prop="equipmentName" min-width="120" show-overflow-tooltip />
            <el-table-column label="分类" width="96" align="center">
              <template #default="scope">
                <dict-tag :options="device_category" :value="scope.row.deviceCategory" />
              </template>
            </el-table-column>
            <el-table-column label="上报时间" width="150" align="center">
              <template #default="scope">
                <span>{{ parseTime(scope.row.uploadTime) }}</span>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-col>
    </el-row>

    <!-- 最近上报 -->
    <el-row :gutter="16">
      <el-col :span="24">
        <div class="panel">
          <div class="panel-head">
            <span class="panel-title">最近上报数据</span>
            <span class="panel-hint">更新于 {{ updatedAt }}</span>
          </div>
          <el-table :data="recentReports" size="small" class="mini-table" empty-text="暂无上报数据">
            <el-table-column label="设备编号" prop="equipmentNumber" min-width="140" show-overflow-tooltip />
            <el-table-column label="设备名称" prop="equipmentName" min-width="160" show-overflow-tooltip />
            <el-table-column label="归属部门" prop="deptName" min-width="120" show-overflow-tooltip />
            <el-table-column label="设备分类" width="110" align="center">
              <template #default="scope">
                <dict-tag :options="device_category" :value="scope.row.deviceCategory" />
              </template>
            </el-table-column>
            <el-table-column label="电量" width="90" align="center">
              <template #default="scope">
                <span>{{ scope.row.batteryLevel ?? "--" }}%</span>
              </template>
            </el-table-column>
            <el-table-column label="信号强度" width="100" align="center">
              <template #default="scope">
                <span>{{ scope.row.signalStrength ?? "--" }}</span>
              </template>
            </el-table-column>
            <el-table-column label="上报时间" width="160" align="center">
              <template #default="scope">
                <span>{{ parseTime(scope.row.uploadTime) }}</span>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup name="Index">
import * as echarts from "echarts";
import { listInstallation } from "@/api/device/installation";
import { listData, getDataStats } from "@/api/device/data";
import useUserStore from "@/store/modules/user";
import { parseTime } from "@/utils/ruoyi";

const { proxy } = getCurrentInstance();
const router = useRouter();
const userStore = useUserStore();
const { device_category, equipment_type } = proxy.useDict("device_category", "equipment_type");

const loading = ref(true);
const updatedAt = ref("--");
const kpi = reactive({ total: 0, online: 0, offline: 0, alarm: 0 });
const recentAlarms = ref([]);
const recentReports = ref([]);
const allDevices = ref([]);

const statusRef = ref();
const typeRef = ref();
const deptRef = ref();
let statusChart = null;
let typeChart = null;
let deptChart = null;

const userName = computed(() => userStore.name || "管理员");
const greeting = computed(() => {
  const hour = new Date().getHours();
  if (hour < 6) return "夜深了";
  if (hour < 9) return "早上好";
  if (hour < 12) return "上午好";
  if (hour < 14) return "中午好";
  if (hour < 18) return "下午好";
  return "晚上好";
});
const today = computed(() => {
  const d = new Date();
  const week = ["日", "一", "二", "三", "四", "五", "六"][d.getDay()];
  return `${d.getFullYear()} 年 ${d.getMonth() + 1} 月 ${d.getDate()} 日 星期${week}`;
});
const onlineRate = computed(() =>
  kpi.total > 0 ? Math.round((kpi.online / kpi.total) * 100) : 0
);
const rateColor = computed(() => {
  if (onlineRate.value >= 80) return "#67C23A";
  if (onlineRate.value >= 50) return "#E6A23C";
  return "#F56C6C";
});
const kpis = computed(() => [
  {
    key: "total",
    label: "设备总数",
    value: kpi.total,
    unit: "台",
    icon: "Monitor",
    color: "#409EFF",
    bg: "rgba(64, 158, 255, 0.12)"
  },
  {
    key: "online",
    label: "在线设备",
    value: kpi.online,
    unit: "台",
    icon: "Connection",
    color: "#67C23A",
    bg: "rgba(103, 194, 58, 0.12)"
  },
  {
    key: "offline",
    label: "离线设备",
    value: kpi.offline,
    unit: "台",
    icon: "CircleClose",
    color: "#909399",
    bg: "rgba(144, 147, 153, 0.14)"
  },
  {
    key: "alarm",
    label: "告警数据",
    value: kpi.alarm,
    unit: "条",
    icon: "Warning",
    color: "#F56C6C",
    bg: "rgba(245, 108, 108, 0.12)"
  }
]);

function goPage(path) {
  router.push(path);
}

function cssVar(name, fallback) {
  const value = getComputedStyle(document.documentElement).getPropertyValue(name);
  return (value || "").trim() || fallback;
}

function dictLabel(options, value) {
  const list = options.value || [];
  const item = list.find(d => String(d.value) === String(value));
  return item ? item.label : value || "未分类";
}

function countBy(list, key) {
  const map = new Map();
  list.forEach(item => {
    const value = item[key] === undefined || item[key] === null || item[key] === "" ? "未设置" : item[key];
    map.set(value, (map.get(value) || 0) + 1);
  });
  return map;
}

async function loadData() {
  loading.value = true;
  try {
    const [statsRes, installRes, reportRes, alarmRes] = await Promise.all([
      getDataStats().catch(() => ({ data: {} })),
      listInstallation({ pageNum: 1, pageSize: 200 }).catch(() => ({ rows: [], total: 0 })),
      listData({ pageNum: 1, pageSize: 5 }).catch(() => ({ rows: [], total: 0 })),
      listData({ pageNum: 1, pageSize: 5, alarmStatus: 1 }).catch(() => ({ rows: [], total: 0 }))
    ]);

    const stats = statsRes.data || {};
    const devices = installRes.rows || [];
    allDevices.value = devices;
    kpi.total = stats.total ?? installRes.total ?? 0;
    kpi.online = stats.online ?? 0;
    kpi.offline = stats.offline ?? Math.max(0, kpi.total - kpi.online);
    kpi.alarm = alarmRes.total || 0;
    recentAlarms.value = alarmRes.rows || [];
    recentReports.value = reportRes.rows || [];
    updatedAt.value = parseTime(new Date(), "{h}:{i}:{s}") || "--";
    renderCharts(devices);
  } finally {
    loading.value = false;
  }
}

function renderCharts(devices) {
  nextTick(() => {
    const textColor = cssVar("--el-text-color-secondary", "#909399");
    const primaryText = cssVar("--el-text-color-primary", "#303133");
    const splitColor = cssVar("--el-border-color-lighter", "#ebeef5");
    const panelBg = cssVar("--el-bg-color", "#ffffff");

    // 在线状态环形图
    statusChart = statusChart || echarts.init(statusRef.value);
    statusChart.setOption(
      {
        tooltip: { trigger: "item", formatter: "{b}：{c} 台（{d}%）" },
        title: {
          text: onlineRate.value + "%",
          subtext: "在线率",
          left: "center",
          top: "31%",
          textStyle: { color: primaryText, fontSize: 26, fontWeight: 600 },
          subtextStyle: { color: textColor, fontSize: 12 }
        },
        legend: {
          bottom: 4,
          icon: "circle",
          itemWidth: 8,
          itemHeight: 8,
          textStyle: { color: textColor }
        },
        series: [
          {
            type: "pie",
            radius: ["60%", "78%"],
            center: ["50%", "44%"],
            avoidLabelOverlap: true,
            label: { show: false },
            labelLine: { show: false },
            itemStyle: { borderWidth: 2, borderColor: panelBg },
            data: [
              { value: kpi.online, name: "在线", itemStyle: { color: "#67C23A" } },
              { value: kpi.offline, name: "离线", itemStyle: { color: "#C0C4CC" } }
            ]
          }
        ]
      },
      true
    );

    // 设备类型分布柱状图
    const typeCount = countBy(devices, "equipmentType");
    const typeLabels = Array.from(typeCount.keys()).map(v => dictLabel(equipment_type, v));
    const typeValues = Array.from(typeCount.values());
    typeChart = typeChart || echarts.init(typeRef.value);
    typeChart.setOption(
      {
        tooltip: { trigger: "axis", axisPointer: { type: "shadow" } },
        grid: { left: 8, right: 16, top: 24, bottom: 8, containLabel: true },
        xAxis: {
          type: "category",
          data: typeLabels.length ? typeLabels : ["暂无数据"],
          axisLine: { lineStyle: { color: splitColor } },
          axisTick: { show: false },
          axisLabel: { color: textColor, interval: 0, rotate: typeLabels.length > 6 ? 30 : 0 }
        },
        yAxis: {
          type: "value",
          minInterval: 1,
          splitLine: { lineStyle: { color: splitColor, type: "dashed" } },
          axisLabel: { color: textColor }
        },
        series: [
          {
            type: "bar",
            barMaxWidth: 30,
            data: typeValues.length ? typeValues : [0],
            itemStyle: {
              borderRadius: [6, 6, 0, 0],
              color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                { offset: 0, color: "#409EFF" },
                { offset: 1, color: "#79BBFF" }
              ])
            }
          }
        ]
      },
      true
    );

    // 部门设备分布（横向柱状图）
    const deptCount = countBy(devices, "deptName");
    const deptList = Array.from(deptCount.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, 8)
      .reverse();
    const deptLabels = deptList.length ? deptList.map(i => i[0]) : ["暂无数据"];
    const deptValues = deptList.length ? deptList.map(i => i[1]) : [0];
    deptChart = deptChart || echarts.init(deptRef.value);
    deptChart.setOption(
      {
        tooltip: { trigger: "axis", axisPointer: { type: "shadow" } },
        grid: { left: 8, right: 24, top: 12, bottom: 8, containLabel: true },
        xAxis: {
          type: "value",
          minInterval: 1,
          splitLine: { lineStyle: { color: splitColor, type: "dashed" } },
          axisLabel: { color: textColor }
        },
        yAxis: {
          type: "category",
          data: deptLabels,
          axisLine: { show: false },
          axisTick: { show: false },
          axisLabel: { color: textColor }
        },
        series: [
          {
            type: "bar",
            barMaxWidth: 18,
            data: deptValues,
            itemStyle: {
              borderRadius: [0, 6, 6, 0],
              color: new echarts.graphic.LinearGradient(0, 0, 1, 0, [
                { offset: 0, color: "#36CFC9" },
                { offset: 1, color: "#409EFF" }
              ])
            }
          }
        ]
      },
      true
    );
  });
}

function handleResize() {
  statusChart && statusChart.resize();
  typeChart && typeChart.resize();
  deptChart && deptChart.resize();
}

// 字典是异步加载的，加载完成后重绘一次类型分布图，避免显示原始字典值
watch(equipment_type, () => {
  if (allDevices.value.length) {
    renderCharts(allDevices.value);
  }
});

onMounted(() => {
  loadData();
  window.addEventListener("resize", handleResize);
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", handleResize);
  statusChart && statusChart.dispose();
  typeChart && typeChart.dispose();
  deptChart && deptChart.dispose();
});
</script>

<style scoped lang="scss">
.dashboard {
  background: var(--el-bg-color-page);
  min-height: calc(100vh - 84px);
}

.hero {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  padding: 26px 32px;
  margin-bottom: 16px;
  border-radius: 12px;
  color: #fff;
  background: linear-gradient(118deg, #1d5fd0 0%, #2f8bfd 52%, #2fd3c4 100%);
  overflow: hidden;
}
.hero::after {
  content: "";
  position: absolute;
  right: -60px;
  top: -80px;
  width: 260px;
  height: 260px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.08);
}
.hero-kicker {
  font-size: 13px;
  letter-spacing: 1px;
  opacity: 0.82;
}
.hero-title {
  margin: 8px 0 6px;
  font-size: 26px;
  font-weight: 600;
}
.hero-sub {
  margin: 0;
  font-size: 13px;
  opacity: 0.9;
}
.hero-actions {
  margin-top: 18px;
  display: flex;
  align-items: center;
  gap: 12px;
}
.hero-actions :deep(.el-button) {
  border: none;
}
.hero-actions :deep(.el-button--primary) {
  background: #fff;
  color: #1d5fd0;
}
.hero-refresh {
  color: rgba(255, 255, 255, 0.9);
  margin-left: 4px;
}
.hero-rate {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  position: relative;
  z-index: 1;
}
.hero-rate :deep(.el-progress__text) {
  color: #fff;
  font-size: 24px !important;
  font-weight: 600;
}
.hero-rate :deep(.el-progress-circle__track) {
  stroke: rgba(255, 255, 255, 0.25);
}
.hero-rate-label {
  font-size: 12px;
  opacity: 0.85;
}

.kpi-row {
  margin-bottom: 16px;
}
.kpi-card {
  display: flex;
  align-items: center;
  gap: 14px;
  height: 100%;
  padding: 18px 20px;
  border-radius: 10px;
  background: var(--el-bg-color);
  border: 1px solid var(--el-border-color-lighter);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.kpi-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(31, 45, 61, 0.08);
}
.kpi-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 46px;
  height: 46px;
  border-radius: 12px;
  flex-shrink: 0;
}
.kpi-value {
  font-size: 26px;
  font-weight: 600;
  line-height: 1.1;
  color: var(--el-text-color-primary);
  font-variant-numeric: tabular-nums;
}
.kpi-unit {
  margin-left: 4px;
  font-size: 13px;
  font-weight: 400;
  color: var(--el-text-color-secondary);
}
.kpi-label {
  margin-top: 4px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.panel {
  height: 100%;
  margin-bottom: 16px;
  padding: 16px 18px 12px;
  border-radius: 10px;
  background: var(--el-bg-color);
  border: 1px solid var(--el-border-color-lighter);
}
.panel-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}
.panel-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}
.panel-hint {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}
.chart {
  width: 100%;
}
.chart-sm {
  height: 260px;
}
.chart-md {
  height: 300px;
}
.mini-table {
  margin-top: 4px;
}

@media (max-width: 768px) {
  .hero {
    flex-direction: column;
    align-items: flex-start;
    padding: 20px;
  }
  .hero-title {
    font-size: 21px;
  }
  .chart-sm {
    height: 220px;
  }
  .chart-md {
    height: 240px;
  }
}
</style>
