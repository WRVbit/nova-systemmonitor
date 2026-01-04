<!-- Nova System Monitor - GPU Detail View -->
<script lang="ts">
    import { onMount } from "svelte";
    import { Activity } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";

    let selectedGpuIndex = $state(0);
    let intervalId: number | undefined;

    onMount(() => {
        monitor.fetchGpuInfo();
        intervalId = setInterval(() => {
            monitor.fetchGpuInfo();
        }, 2000) as unknown as number;

        return () => {
            if (intervalId !== undefined) {
                clearInterval(intervalId);
                intervalId = undefined;
            }
        };
    });

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

    function getVendorColor(vendor: string): string {
        if (vendor.toLowerCase() === "nvidia") return "#76b900";
        if (vendor.toLowerCase() === "amd") return "#ed1c24";
        if (vendor.toLowerCase() === "intel") return "#0071c5";
        return "#89b4fa";
    }
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <Activity size={32} />
            <div>
                <h1>GPU</h1>
                <p>Graphics Card Performance</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.gpuInfo.loading && !monitor.gpuInfo.data}
            <div class="loading">Loading GPU information...</div>
        {:else if monitor.gpuInfo.error}
            <div class="error">Error: {monitor.gpuInfo.error}</div>
        {:else if monitor.gpuInfo.data}
            {@const gpuData = monitor.gpuInfo.data}

            {#if gpuData.gpus.length === 0}
                <div class="card no-gpu">
                    <h2>No GPU Detected</h2>
                    <p>No compatible GPUs found on this system.</p>
                    {#if gpuData.errors.length > 0}
                        <div class="error-list">
                            <h3>Detection Errors:</h3>
                            {#each gpuData.errors as error}
                                <div class="error-item">{error}</div>
                            {/each}
                        </div>
                    {/if}
                </div>
            {:else}
                <!-- Multi-GPU Tabs -->
                {#if gpuData.gpus.length > 1}
                    <div class="gpu-tabs">
                        {#each gpuData.gpus as gpu, i}
                            <button
                                class="tab"
                                class:active={selectedGpuIndex === i}
                                onclick={() => (selectedGpuIndex = i)}
                            >
                                GPU {i}: {gpu.name.substring(0, 20)}{gpu.name
                                    .length > 20
                                    ? "..."
                                    : ""}
                            </button>
                        {/each}
                    </div>
                {/if}

                {@const gpu = gpuData.gpus[selectedGpuIndex]}
                {@const isIgpu = (gpu.vendor.toLowerCase() === 'amd' && (gpu.name.toLowerCase().includes('radeon graphics') || gpu.name.toLowerCase().includes('ryzen'))) || gpu.vendor.toLowerCase() === 'intel' || gpu.name.toLowerCase().includes('integrated') || gpu.memory_total === 0}

                <!-- GPU Type Badge -->
                <div class="card gpu-type-card">
                    <div class="gpu-type-badge {isIgpu ? 'igpu' : 'dgpu'}">
                        {isIgpu ? '🔷 Integrated GPU (iGPU)' : '🎮 Dedicated GPU (dGPU)'}
                    </div>
                    <h2>{gpu.name}</h2>
                    <p class="gpu-subtitle">{gpu.vendor} • {isIgpu ? 'Shared System Memory' : gpu.uuid}</p>
                </div>

                <!-- GPU Stats Overview -->
                <div class="stats-row">
                    <div class="card stat-card">
                        <h3 style="color: {getVendorColor(gpu.vendor)}">
                            GPU Utilization
                        </h3>
                        <div class="usage-ring">
                            <svg viewBox="0 0 120 120" width="160" height="160">
                                <circle
                                    cx="60"
                                    cy="60"
                                    r="50"
                                    fill="none"
                                    stroke="var(--md-sys-color-surface-variant)"
                                    stroke-width="10"
                                />
                                <circle
                                    cx="60"
                                    cy="60"
                                    r="50"
                                    fill="none"
                                    stroke="#cba6f7"
                                    stroke-width="10"
                                    stroke-dasharray={`${(gpu.utilization_gpu / 100) * 314} 314`}
                                    stroke-linecap="round"
                                    transform="rotate(-90 60 60)"
                                />
                                <text
                                    x="60"
                                    y="60"
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size="18"
                                    font-weight="bold"
                                    fill="var(--md-sys-color-on-surface)"
                                >
                                    {formatPercent(gpu.utilization_gpu)}
                                </text>
                            </svg>
                        </div>
                    </div>

                    <div class="card stat-card">
                        <h3>VRAM Usage</h3>
                        <div class="usage-ring">
                            <svg viewBox="0 0 120 120" width="160" height="160">
                                <circle
                                    cx="60"
                                    cy="60"
                                    r="50"
                                    fill="none"
                                    stroke="var(--md-sys-color-surface-variant)"
                                    stroke-width="10"
                                />
                                <circle
                                    cx="60"
                                    cy="60"
                                    r="50"
                                    fill="none"
                                    stroke="#89dceb"
                                    stroke-width="10"
                                    stroke-dasharray={`${(gpu.utilization_memory / 100) * 314} 314`}
                                    stroke-linecap="round"
                                    transform="rotate(-90 60 60)"
                                />
                                <text
                                    x="60"
                                    y="60"
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size="18"
                                    font-weight="bold"
                                    fill="var(--md-sys-color-on-surface)"
                                >
                                    {formatPercent(gpu.utilization_memory)}
                                </text>
                            </svg>
                        </div>
                        {#if gpu.memory_total > 0}
                            <div class="stat-detail">
                                {formatBytes(gpu.memory_used)} / {formatBytes(
                                    gpu.memory_total,
                                )}
                            </div>
                        {:else}
                            <div class="stat-detail">Shared Memory</div>
                        {/if}
                    </div>

                    <div class="card stat-card">
                        <h3>Temperature</h3>
                        <div
                            class="temp-display"
                            style="color: {gpu.temperature > 80
                                ? '#f38ba8'
                                : gpu.temperature > 60
                                  ? '#f9e2af'
                                  : '#a6e3a1'}"
                        >
                            {gpu.temperature}°C
                        </div>
                        <div class="stat-detail">
                            {gpu.temperature > 80
                                ? "Hot"
                                : gpu.temperature > 60
                                  ? "Warm"
                                  : "Cool"}
                        </div>
                    </div>

                    <div class="card stat-card">
                        <h3>Power Usage</h3>
                        <div class="power-display">
                            {(gpu.power_usage / 1000).toFixed(1)}W
                        </div>
                        {#if gpu.power_limit > 0}
                            <div class="stat-detail">
                                Limit: {(gpu.power_limit / 1000).toFixed(1)}W
                            </div>
                        {/if}
                    </div>
                </div>

                <!-- GPU Info -->
                <div class="card">
                    <h2>GPU Information</h2>
                    <div class="info-grid">
                        <div class="info-item">
                            <span class="label">Name:</span>
                            <span class="value">{gpu.name}</span>
                        </div>
                        <div class="info-item">
                            <span class="label">Vendor:</span>
                            <span
                                class="value"
                                style="color: {getVendorColor(gpu.vendor)}"
                                >{gpu.vendor}</span
                            >
                        </div>
                        <div class="info-item">
                            <span class="label">Graphics Clock:</span>
                            <span class="value">{gpu.clock_graphics} MHz</span>
                        </div>
                        <div class="info-item">
                            <span class="label">Memory Clock:</span>
                            <span class="value">{gpu.clock_memory} MHz</span>
                        </div>
                        {#if gpu.fan_speed !== null}
                            <div class="info-item">
                                <span class="label">Fan Speed:</span>
                                <span class="value">{gpu.fan_speed}%</span>
                            </div>
                        {/if}
                        {#if gpu.encoder_utilization !== null}
                            <div class="info-item">
                                <span class="label">Encoder:</span>
                                <span class="value"
                                    >{gpu.encoder_utilization}%</span
                                >
                            </div>
                        {/if}
                        {#if gpu.decoder_utilization !== null}
                            <div class="info-item">
                                <span class="label">Decoder:</span>
                                <span class="value"
                                    >{gpu.decoder_utilization}%</span
                                >
                            </div>
                        {/if}
                    </div>
                </div>
            {/if}
        {/if}
    </div>
</div>

<style>
    .page {
        min-height: 100vh;
        background: var(--md-sys-color-background);
    }

    .page-header {
        padding: 2rem;
        background: var(--md-sys-color-surface);
        border-bottom: 1px solid var(--md-sys-color-outline);
    }

    .header-title {
        display: flex;
        align-items: center;
        gap: 1rem;
        color: var(--md-sys-color-primary);
    }

    .header-title h1 {
        margin: 0;
        font-size: 2rem;
        font-weight: 700;
        color: var(--md-sys-color-on-surface);
    }

    .header-title p {
        margin: 0.25rem 0 0 0;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .page-content {
        padding: 2rem;
        max-width: 1400px;
        margin: 0 auto;
    }

    .gpu-tabs {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1.5rem;
        flex-wrap: wrap;
    }

    .tab {
        padding: 0.75rem 1.5rem;
        background: var(--md-sys-color-surface);
        border: 1px solid var(--md-sys-color-outline);
        border-radius: 8px;
        color: var(--md-sys-color-on-surface);
        cursor: pointer;
        transition: all 0.2s ease;
        font-size: 0.875rem;
        font-weight: 500;
    }

    .tab:hover {
        background: var(--md-sys-color-surface-variant);
    }

    .tab.active {
        background: var(--md-sys-color-primary);
        color: var(--md-sys-color-on-primary);
        border-color: var(--md-sys-color-primary);
    }

    .stats-row {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1.5rem;
        margin-bottom: 1.5rem;
    }

    .card {
        background: var(--md-sys-color-surface);
        border-radius: 16px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .card h2,
    .card h3 {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .stat-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .usage-ring {
        margin: 0.5rem 0;
    }

    .temp-display,
    .power-display {
        font-size: 3rem;
        font-weight: 700;
        margin: 1rem 0;
    }

    .stat-detail {
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .gpu-type-card {
        background: linear-gradient(135deg, var(--md-sys-color-surface) 0%, var(--md-sys-color-surface-variant) 100%);
        border-left: 4px solid var(--md-sys-color-primary);
    }

    .gpu-type-badge {
        display: inline-block;
        padding: 0.5rem 1rem;
        border-radius: 20px;
        font-size: 0.875rem;
        font-weight: 600;
        margin-bottom: 1rem;
    }

    .gpu-type-badge.igpu {
        background: rgba(137, 180, 250, 0.2);
        color: #89b4fa;
    }

    .gpu-type-badge.dgpu {
        background: rgba(166, 227, 161, 0.2);
        color: #a6e3a1;
    }

    .gpu-subtitle {
        margin: 0.5rem 0 0 0;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
        font-family: "JetBrains Mono", monospace;
    }

    .info-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1rem;
    }

    .info-item {
        display: flex;
        justify-content: space-between;
        padding: 0.75rem;
        background: var(--md-sys-color-surface-variant);
        border-radius: 8px;
    }

    .label {
        font-weight: 500;
        color: var(--md-sys-color-on-surface-variant);
    }

    .value {
        font-family: "JetBrains Mono", monospace;
        color: var(--md-sys-color-on-surface);
    }

    .no-gpu {
        text-align: center;
        padding: 3rem;
    }

    .error-list {
        margin-top: 2rem;
        text-align: left;
    }

    .error-item {
        margin: 0.5rem 0;
        padding: 0.75rem;
        background: rgba(243, 139, 168, 0.1);
        border-left: 3px solid #f38ba8;
        border-radius: 4px;
        font-size: 0.875rem;
        font-family: "JetBrains Mono", monospace;
    }

    .loading,
    .error {
        padding: 2rem;
        text-align: center;
        color: var(--md-sys-color-on-surface-variant);
    }

    .error {
        color: #f38ba8;
    }
</style>
