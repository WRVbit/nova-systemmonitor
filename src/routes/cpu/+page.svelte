<!-- Nova System Monitor - CPU Detail View -->
<script lang="ts">
    import { onMount } from "svelte";

    interface CpuCore {
        name: string;
        usage: number;
        frequency: number;
    }
    import { Cpu } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";
    import Graph from "$lib/components/Graph.svelte";

    let intervalId: number | undefined;

    onMount(() => {
        monitor.fetchCpuInfo();
        intervalId = setInterval(() => {
            monitor.fetchCpuInfo();
        }, 2000) as unknown as number;

        return () => {
            if (intervalId !== undefined) {
                clearInterval(intervalId);
                intervalId = undefined;
            }
        };
    });

    function formatPercent(value: number): string {
        return `${value.toFixed(1)}%`;
    }

    function formatFreq(mhz: number): string {
        if (mhz >= 1000) {
            return `${(mhz / 1000).toFixed(2)} GHz`;
        }
        return `${mhz} MHz`;
    }
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <Cpu size={32} />
            <div>
                <h1>CPU</h1>
                <p>Processor Usage & Performance</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.cpuInfo.loading && !monitor.cpuInfo.data}
            <div class="loading">Loading CPU information...</div>
        {:else if monitor.cpuInfo.error}
            <div class="error">Error: {monitor.cpuInfo.error}</div>
        {:else if monitor.cpuInfo.data}
            {@const cpu = monitor.cpuInfo.data}

            <!-- CPU Info Card -->
            <div class="card info-card">
                <h2>Processor Information</h2>
                <div class="info-grid">
                    <div class="info-item">
                        <span class="label">Model:</span>
                        <span class="value">{cpu.brand}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">Vendor:</span>
                        <span class="value">{cpu.vendor}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">Cores:</span>
                        <span class="value">{cpu.physical_cores} cores / {cpu.logical_cores} threads</span>
                    </div>
                </div>
            </div>

            <!-- Global Usage -->
            <div class="card usage-card">
                <h2>Overall CPU Usage</h2>
                <div class="usage-display">
                    <div class="usage-ring">
                        <svg viewBox="0 0 120 120" width="200" height="200">
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
                                stroke="var(--md-sys-color-primary)"
                                stroke-width="10"
                                stroke-dasharray={`${(cpu.global_usage / 100) * 314} 314`}
                                stroke-linecap="round"
                                transform="rotate(-90 60 60)"
                            />
                            <text
                                x="60"
                                y="60"
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="24"
                                font-weight="bold"
                                fill="var(--md-sys-color-on-surface)"
                            >
                                {formatPercent(cpu.global_usage)}
                            </text>
                        </svg>
                    </div>
                    <div class="usage-stats">
                        <div class="stat">
                            <span class="stat-label">Average Usage</span>
                            <span class="stat-value"
                                >{formatPercent(cpu.global_usage)}</span
                            >
                        </div>
                        <div class="stat">
                            <span class="stat-label">Active Cores</span>
                            <span class="stat-value"
                                >{cpu.cores.filter((c: CpuCore) => c.usage > 5)
                                    .length} /
                                {cpu.cores.length}</span
                            >
                        </div>
                    </div>
                </div>
                <!-- History Graph -->
                {#if monitor.cpuHistory.timestamps.length > 1}
                    <div class="history-graph">
                        <h3>Usage History</h3>
                        <Graph
                            data={[
                                monitor.cpuHistory.timestamps,
                                monitor.cpuHistory.global,
                            ]}
                            series={[
                                {
                                    label: "Total CPU",
                                    stroke: "#cba6f7",
                                    fill: "#cba6f722",
                                },
                            ]}
                            height={200}
                            yMin={0}
                            yMax={100}
                            yLabel="%"
                        />
                    </div>
                {/if}
            </div>

            <!-- Per-Core Usage -->
            <div class="card cores-card">
                <h2>Per-Core Usage</h2>
                <div class="cores-grid">
                    {#each cpu.cores as core, i}
                        <div class="core-item">
                            <div class="core-header">
                                <span class="core-name">Core {i}</span>
                                <span class="core-freq"
                                    >{formatFreq(core.frequency)}</span
                                >
                            </div>
                            <div class="core-bar-container">
                                <div
                                    class="core-bar"
                                    style="width: {core.usage}%"
                                >
                                    <span class="core-usage"
                                        >{formatPercent(core.usage)}</span
                                    >
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
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

    .card {
        background: var(--md-sys-color-surface);
        border-radius: 16px;
        padding: 1.5rem;
        margin-bottom: 1.5rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .card h2 {
        margin: 0 0 1.5rem 0;
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

    .usage-display {
        display: flex;
        gap: 3rem;
        align-items: center;
        margin-bottom: 2rem;
    }

    .history-graph {
        margin-top: 1rem;
    }

    .usage-ring {
        flex-shrink: 0;
    }

    .usage-stats {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .stat {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .stat-label {
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .stat-value {
        font-size: 2rem;
        font-weight: 700;
        color: var(--md-sys-color-on-surface);
    }

    .cores-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1rem;
    }

    .core-item {
        background: var(--md-sys-color-surface-variant);
        padding: 1rem;
        border-radius: 8px;
    }

    .core-header {
        display: flex;
        justify-content: space-between;
        margin-bottom: 0.75rem;
    }

    .core-name {
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .core-freq {
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
        font-family: "JetBrains Mono", monospace;
    }

    .core-bar-container {
        background: var(--md-sys-color-background);
        height: 32px;
        border-radius: 16px;
        overflow: hidden;
        position: relative;
    }

    .core-bar {
        height: 100%;
        background: linear-gradient(90deg, #89b4fa, #74c7ec);
        display: flex;
        align-items: center;
        justify-content: flex-end;
        padding-right: 0.75rem;
        transition: width 0.3s ease;
        min-width: 60px;
    }

    .core-usage {
        font-size: 0.875rem;
        font-weight: 600;
        color: white;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
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
