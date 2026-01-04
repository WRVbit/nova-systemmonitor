<!-- Nova System Monitor - Memory Detail View -->
<script lang="ts">
    import { onMount } from "svelte";
    import { MemoryStick } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";
    import Graph from "$lib/components/Graph.svelte";

    let intervalId: number | undefined;

    onMount(() => {
        monitor.fetchMemoryInfo();
        intervalId = setInterval(() => {
            monitor.fetchMemoryInfo();
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
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <MemoryStick size={32} />
            <div>
                <h1>Memory</h1>
                <p>RAM & SWAP Usage</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.memoryInfo.loading && !monitor.memoryInfo.data}
            <div class="loading">Loading memory information...</div>
        {:else if monitor.memoryInfo.error}
            <div class="error">Error: {monitor.memoryInfo.error}</div>
        {:else if monitor.memoryInfo.data}
            {@const mem = monitor.memoryInfo.data}

            <!-- Memory Overview -->
            <div class="stats-row">
                <div class="card stat-card">
                    <h3>RAM Usage</h3>
                    <div class="usage-ring">
                        <svg viewBox="0 0 120 120" width="180" height="180">
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
                                stroke="#a6e3a1"
                                stroke-width="10"
                                stroke-dasharray={`${(mem.memory_usage_percent / 100) * 314} 314`}
                                stroke-linecap="round"
                                transform="rotate(-90 60 60)"
                            />
                            <text
                                x="60"
                                y="60"
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="20"
                                font-weight="bold"
                                fill="var(--md-sys-color-on-surface)"
                            >
                                {formatPercent(mem.memory_usage_percent)}
                            </text>
                        </svg>
                    </div>
                    <div class="stat-details">
                        <div class="detail-item">
                            <span>Used:</span>
                            <span class="value"
                                >{formatBytes(mem.used_memory)}</span
                            >
                        </div>
                        <div class="detail-item">
                            <span>Available:</span>
                            <span class="value"
                                >{formatBytes(mem.available_memory)}</span
                            >
                        </div>
                        <div class="detail-item">
                            <span>Total:</span>
                            <span class="value"
                                >{formatBytes(mem.total_memory)}</span
                            >
                        </div>
                    </div>
                </div>

                <div class="card stat-card">
                    <h3>SWAP Usage</h3>
                    <div class="usage-ring">
                        <svg viewBox="0 0 120 120" width="180" height="180">
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
                                stroke="#f9e2af"
                                stroke-width="10"
                                stroke-dasharray={`${(mem.swap_usage_percent / 100) * 314} 314`}
                                stroke-linecap="round"
                                transform="rotate(-90 60 60)"
                            />
                            <text
                                x="60"
                                y="60"
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="20"
                                font-weight="bold"
                                fill="var(--md-sys-color-on-surface)"
                            >
                                {formatPercent(mem.swap_usage_percent)}
                            </text>
                        </svg>
                    </div>
                    <div class="stat-details">
                        <div class="detail-item">
                            <span>Used:</span>
                            <span class="value"
                                >{formatBytes(mem.used_swap)}</span
                            >
                        </div>
                        <div class="detail-item">
                            <span>Total:</span>
                            <span class="value"
                                >{formatBytes(mem.total_swap)}</span
                            >
                        </div>
                    </div>
                </div>
            </div>

            <!-- Memory Bars -->
            <div class="card">
                <h2>Memory Breakdown</h2>
                <div class="memory-bars">
                    <div class="bar-item">
                        <div class="bar-header">
                            <span>RAM</span>
                            <span class="bar-value"
                                >{formatBytes(mem.used_memory)} / {formatBytes(
                                    mem.total_memory,
                                )}</span
                            >
                        </div>
                        <div class="bar-container">
                            <div
                                class="bar ram"
                                style="width: {mem.memory_usage_percent}%"
                            ></div>
                        </div>
                    </div>

                    <div class="bar-item">
                        <div class="bar-header">
                            <span>Available</span>
                            <span class="bar-value"
                                >{formatBytes(mem.available_memory)}</span
                            >
                        </div>
                        <div class="bar-container">
                            <div
                                class="bar available"
                                style="width: {(mem.available_memory /
                                    mem.total_memory) *
                                    100}%"
                            ></div>
                        </div>
                    </div>

                    {#if mem.total_swap > 0}
                        <div class="bar-item">
                            <div class="bar-header">
                                <span>SWAP</span>
                                <span class="bar-value"
                                    >{formatBytes(mem.used_swap)} / {formatBytes(
                                        mem.total_swap,
                                    )}</span
                                >
                            </div>
                            <div class="bar-container">
                                <div
                                    class="bar swap"
                                    style="width: {mem.swap_usage_percent}%"
                                ></div>
                            </div>
                        </div>
                    {/if}
                </div>
            </div>

            <!-- History Graphs -->
            {#if monitor.memoryHistory.timestamps.length > 1}
                <div class="card history-card">
                    <h2>History</h2>
                    <div class="graphs-grid">
                        <div class="graph-item">
                            <h3>RAM Usage History</h3>
                            <Graph
                                data={[
                                    monitor.memoryHistory.timestamps,
                                    monitor.memoryHistory.ram_used,
                                ]}
                                series={[
                                    {
                                        label: "RAM Usage",
                                        stroke: "#a6e3a1",
                                        fill: "#a6e3a122",
                                    },
                                ]}
                                height={150}
                                yMin={0}
                                yMax={100}
                                yLabel="%"
                            />
                        </div>
                        {#if mem.total_swap > 0}
                            <div class="graph-item">
                                <h3>SWAP Usage History</h3>
                                <Graph
                                    data={[
                                        monitor.memoryHistory.timestamps,
                                        monitor.memoryHistory.swap_used,
                                    ]}
                                    series={[
                                        {
                                            label: "SWAP Usage",
                                            stroke: "#f9e2af",
                                            fill: "#f9e2af22",
                                        },
                                    ]}
                                    height={150}
                                    yMin={0}
                                    yMax={100}
                                    yLabel="%"
                                />
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

    .stats-row {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
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
        margin: 0 0 1.5rem 0;
        font-size: 1.25rem;
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
        margin: 1rem 0;
    }

    .stat-details {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .detail-item {
        display: flex;
        justify-content: space-between;
        padding: 0.5rem;
        background: var(--md-sys-color-surface-variant);
        border-radius: 6px;
        font-size: 0.875rem;
    }

    .detail-item .value {
        font-family: "JetBrains Mono", monospace;
        font-weight: 600;
    }

    .memory-bars {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .bar-item {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .bar-header {
        display: flex;
        justify-content: space-between;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .bar-value {
        font-family: "JetBrains Mono", monospace;
        font-weight: 600;
    }

    .bar-container {
        height: 32px;
        background: var(--md-sys-color-surface-variant);
        border-radius: 16px;
        overflow: hidden;
    }

    .bar {
        height: 100%;
        transition: width 0.3s ease;
        border-radius: 16px;
    }

    .bar.ram {
        background: linear-gradient(90deg, #a6e3a1, #94e2d5);
    }

    .bar.available {
        background: linear-gradient(90deg, #89b4fa, #74c7ec);
    }

    .bar.swap {
        background: linear-gradient(90deg, #f9e2af, #fab387);
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

    .graphs-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
        gap: 2rem;
    }

    .graph-item h3 {
        font-size: 1rem;
        margin-bottom: 0.5rem;
        color: var(--md-sys-color-on-surface-variant);
    }
</style>
