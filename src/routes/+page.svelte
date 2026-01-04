<!-- Nova System Monitor - Main Dashboard -->
<script lang="ts">
  import { onMount } from "svelte";
  import {
    Cpu,
    MemoryStick,
    HardDrive,
    Network,
    Activity,
  } from "lucide-svelte";
  import * as monitor from "$lib/stores/monitor.svelte";
  import { settings } from "$lib/stores/settings.svelte";

  let intervalId: number | undefined;

  onMount(() => {
    // Stagger initial fetches to prevent UI freeze
    // Critical data first (CPU, Memory, System)
    setTimeout(() => monitor.fetchCpuInfo(), 50);
    setTimeout(() => monitor.fetchMemoryInfo(), 100);
    setTimeout(() => monitor.fetchSystemInfo(), 150);

    // Less critical data later
    setTimeout(() => monitor.fetchDiskInfo(), 300);
    setTimeout(() => monitor.fetchNetworkInfo(), 400);
    setTimeout(() => monitor.fetchGpuInfo(), 500);

    // Live monitoring interval - refresh data every 2 seconds
    intervalId = setInterval(() => {
      monitor.fetchCpuInfo();
      monitor.fetchMemoryInfo();
      monitor.fetchDiskInfo();
      monitor.fetchNetworkInfo();
      monitor.fetchGpuInfo();
      monitor.fetchSystemInfo();
    }, 2000) as unknown as number;

    // Cleanup on unmount
    return () => {
      if (intervalId !== undefined) {
        clearInterval(intervalId);
        intervalId = undefined;
      }
    };
  });

  // Helper functions
  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }

  function formatPercent(value: number): string {
    return `${value.toFixed(1)}%`;
  }
</script>

<div class="dashboard">
  <header class="dashboard-header">
    <h1>Nova System Monitor</h1>
    <div class="header-actions">
      <span class="status">
        {#if monitor.cpuInfo.loading}
          Loading...
        {:else}
          {monitor.systemInfo.data?.hostname || "Unknown Host"}
        {/if}
      </span>
    </div>
  </header>

  <main class="dashboard-content">
    <!-- Quick Stats Grid -->
    <div class="stats-grid">
      <!-- CPU Card -->
      <div class="card">
        <div class="stat-card">
          <div class="stat-icon cpu">
            <Cpu size={32} />
          </div>
          <div class="stat-content">
            <h3>CPU</h3>
            {#if monitor.cpuInfo.data}
              <div class="stat-value">
                {formatPercent(monitor.cpuInfo.data.global_usage)}
              </div>
              <div class="stat-detail">{monitor.cpuInfo.data.brand}</div>
              <div class="stat-detail">
                {monitor.cpuInfo.data.physical_cores} cores / {monitor.cpuInfo
                  .data.logical_cores} threads
              </div>
            {:else if monitor.cpuInfo.loading}
              <div class="stat-value">Loading...</div>
            {:else}
              <div class="stat-value">--</div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Memory Card -->
      <div class="card">
        <div class="stat-card">
          <div class="stat-icon memory">
            <MemoryStick size={32} />
          </div>
          <div class="stat-content">
            <h3>Memory</h3>
            {#if monitor.memoryInfo.data}
              <div class="stat-value">
                {formatPercent(monitor.memoryInfo.data.memory_usage_percent)}
              </div>
              <div class="stat-detail">
                {formatBytes(monitor.memoryInfo.data.used_memory)} / {formatBytes(
                  monitor.memoryInfo.data.total_memory,
                )}
              </div>
            {:else if monitor.memoryInfo.loading}
              <div class="stat-value">Loading...</div>
            {:else}
              <div class="stat-value">--</div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Disk Card -->
      <div class="card">
        <div class="stat-card">
          <div class="stat-icon disk">
            <HardDrive size={32} />
          </div>
          <div class="stat-content">
            <h3>Disk</h3>
            {#if monitor.diskInfo.data}
              {@const usage =
                (monitor.diskInfo.data.total_used /
                  monitor.diskInfo.data.total_space) *
                100}
              <div class="stat-value">{formatPercent(usage)}</div>
              <div class="stat-detail">
                {formatBytes(monitor.diskInfo.data.total_used)} / {formatBytes(
                  monitor.diskInfo.data.total_space,
                )}
              </div>
            {:else if monitor.diskInfo.loading}
              <div class="stat-value">Loading...</div>
            {:else}
              <div class="stat-value">--</div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Network Card -->
      <div class="card">
        <div class="stat-card">
          <div class="stat-icon network">
            <Network size={32} />
          </div>
          <div class="stat-content">
            <h3>Network</h3>
            {#if monitor.networkInfo.data}
              <div class="stat-value">
                {monitor.networkInfo.data.interfaces.length} interfaces
              </div>
              <div class="stat-detail">
                ↓ {formatBytes(
                  monitor.networkInfo.data.total_download_rate || 0,
                )}/s
              </div>
              <div class="stat-detail">
                ↑ {formatBytes(
                  monitor.networkInfo.data.total_upload_rate || 0,
                )}/s
              </div>
            {:else if monitor.networkInfo.loading}
              <div class="stat-value">Loading...</div>
            {:else}
              <div class="stat-value">--</div>
            {/if}
          </div>
        </div>
      </div>

      <!-- GPU Card (if available) -->
      {#if monitor.gpuInfo.data && monitor.gpuInfo.data.gpus.length > 0}
        {@const gpu = monitor.gpuInfo.data.gpus[0]}
        {@const isIgpu =
          (gpu.vendor.toLowerCase() === "amd" &&
            (gpu.name.toLowerCase().includes("radeon graphics") ||
              gpu.name.toLowerCase().includes("ryzen"))) ||
          gpu.vendor.toLowerCase() === "intel" ||
          gpu.name.toLowerCase().includes("integrated") ||
          gpu.memory_total === 0}
        <div class="card">
          <div class="stat-card">
            <div class="stat-icon gpu">
              <Activity size={32} />
            </div>
            <div class="stat-content">
              <h3>{isIgpu ? "iGPU" : "dGPU"}</h3>
              <div class="stat-value">{formatPercent(gpu.utilization_gpu)}</div>
              <div class="stat-detail">{gpu.name}</div>
              <div class="stat-detail">{gpu.temperature}°C</div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- System Info -->
    {#if monitor.systemInfo.data}
      <div class="card">
        <div class="system-info">
          <h2>System Information</h2>
          <div class="info-grid">
            <div class="info-item">
              <span class="info-label">OS:</span>
              <span class="info-value"
                >{monitor.systemInfo.data.os_name}
                {monitor.systemInfo.data.os_version}</span
              >
            </div>
            <div class="info-item">
              <span class="info-label">Kernel:</span>
              <span class="info-value"
                >{monitor.systemInfo.data.kernel_version}</span
              >
            </div>
            <div class="info-item">
              <span class="info-label">Hostname:</span>
              <span class="info-value">{monitor.systemInfo.data.hostname}</span>
            </div>
            <div class="info-item">
              <span class="info-label">Uptime:</span>
              <span class="info-value"
                >{Math.floor(monitor.systemInfo.data.uptime / 3600)}h {Math.floor(
                  (monitor.systemInfo.data.uptime % 3600) / 60,
                )}m</span
              >
            </div>
          </div>
        </div>
      </div>
    {/if}
  </main>
</div>

<style>
  .dashboard {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background: var(--md-sys-color-background);
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    background: var(--md-sys-color-surface);
    border-bottom: 1px solid var(--md-sys-color-outline);
  }

  .dashboard-header h1 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .status {
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface-variant);
  }

  .dashboard-content {
    flex: 1;
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
    width: 100%;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .card {
    background: var(--md-sys-color-surface);
    border-radius: 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    transition: box-shadow 0.2s ease;
  }

  .card:hover {
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  }

  .stat-card {
    display: flex;
    gap: 1.5rem;
    padding: 1.5rem;
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 64px;
    height: 64px;
    border-radius: 16px;
    flex-shrink: 0;
  }

  .stat-icon.cpu {
    background: rgba(137, 180, 250, 0.15);
    color: #89b4fa;
  }

  .stat-icon.memory {
    background: rgba(166, 227, 161, 0.15);
    color: #a6e3a1;
  }

  .stat-icon.disk {
    background: rgba(249, 226, 175, 0.15);
    color: #f9e2af;
  }

  .stat-icon.network {
    background: rgba(148, 226, 213, 0.15);
    color: #94e2d5;
  }

  .stat-icon.gpu {
    background: rgba(203, 166, 247, 0.15);
    color: #cba6f7;
  }

  .stat-content {
    flex: 1;
    min-width: 0;
  }

  .stat-content h3 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .stat-value {
    font-size: 2rem;
    font-weight: 700;
    color: var(--md-sys-color-on-surface);
    margin-bottom: 0.25rem;
  }

  .stat-detail {
    font-size: 0.875rem;
    color: var(--md-sys-color-on-surface-variant);
    margin-top: 0.25rem;
  }

  .system-info {
    padding: 1.5rem;
  }

  .system-info h2 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--md-sys-color-on-surface);
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .info-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: var(--md-sys-color-surface-variant);
    border-radius: 8px;
  }

  .info-label {
    font-weight: 500;
    color: var(--md-sys-color-on-surface-variant);
  }

  .info-value {
    font-family: "JetBrains Mono", monospace;
    color: var(--md-sys-color-on-surface);
  }
</style>
