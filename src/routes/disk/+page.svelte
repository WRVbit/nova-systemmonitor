<!-- Nova System Monitor - Disk Detail View -->
<script lang="ts">
    import { onMount } from "svelte";
    import {
        HardDrive,
        AlertCircle,
        CheckCircle,
        XCircle,
        Database,
        Server,
        ArrowRight,
    } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";

    import Graph from "$lib/components/Graph.svelte";

    let intervalId: number | undefined;

    onMount(() => {
        // Initial fetch
        monitor.fetchDiskInfo();

        // Refresh frequently for I/O graphs (SMART is cached backend-side)
        intervalId = setInterval(() => {
            monitor.fetchDiskInfo();
        }, 2000) as unknown as number;

        // Cleanup on unmount
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

    function formatRate(bytes: number): string {
        return `${formatBytes(bytes)}/s`;
    }

    function getUsageColor(percentage: number): string {
        if (percentage > 90) return "#ef4444"; // Red
        if (percentage > 75) return "#f59e0b"; // Amber
        return "#3b82f6"; // Blue
    }
</script>

<div class="page">
    <header class="page-header">
        <div class="header-content">
            <div class="header-icon">
                <HardDrive size={32} />
            </div>
            <div>
                <h1>Disk Storage</h1>
                <p>Manage and monitor your partitions</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.diskInfo.loading && !monitor.diskInfo.data}
            <div class="loading-state">
                <div class="spinner"></div>
                <p>Scanning storage devices...</p>
            </div>
        {:else if monitor.diskInfo.error}
            <div class="error-state">
                <AlertCircle size={48} />
                <p>Error: {monitor.diskInfo.error}</p>
            </div>
        {:else if monitor.diskInfo.data}
            {@const disk = monitor.diskInfo.data}

            <!-- Summary Section -->
            <section class="summary-section">
                <div class="summary-card total">
                    <div class="summary-icon">
                        <Database size={24} />
                    </div>
                    <div class="summary-info">
                        <span class="label">Total Capacity</span>
                        <span class="value"
                            >{formatBytes(disk.total_space)}</span
                        >
                    </div>
                </div>

                <div class="summary-card used">
                    <div class="summary-icon">
                        <Server size={24} />
                    </div>
                    <div class="summary-info">
                        <span class="label">Used / Available</span>
                        <div class="value-group">
                            <span class="value"
                                >{formatBytes(disk.total_used)}</span
                            >
                            <span class="sub-value"
                                >/ {formatBytes(disk.total_available)}</span
                            >
                        </div>
                    </div>
                </div>

                <div class="summary-card count">
                    <div class="summary-icon">
                        <HardDrive size={24} />
                    </div>
                    <div class="summary-info">
                        <span class="label">Partitions</span>
                        <span class="value">{disk.disks.length}</span>
                    </div>
                </div>
            </section>

            <!-- Disk I/O Graphs -->
            {#if monitor.diskHistory.timestamps.length > 1}
                <div class="card graph-card">
                    <h2>Disk Activity</h2>
                    <div class="graphs-grid">
                        <div class="graph-item">
                            <div class="graph-header">
                                <h3>Read Rate</h3>
                                <span class="rate-value"
                                    >{formatRate(
                                        monitor.diskHistory.read[
                                            monitor.diskHistory.read.length - 1
                                        ] || 0,
                                    )}</span
                                >
                            </div>
                            <Graph
                                data={[
                                    monitor.diskHistory.timestamps,
                                    monitor.diskHistory.read,
                                ]}
                                series={[
                                    {
                                        label: "Read",
                                        stroke: "#89b4fa",
                                        fill: "#89b4fa22",
                                    },
                                ]}
                                height={150}
                                yMin={0}
                                yLabel="B/s"
                            />
                        </div>
                        <div class="graph-item">
                            <div class="graph-header">
                                <h3>Write Rate</h3>
                                <span class="rate-value"
                                    >{formatRate(
                                        monitor.diskHistory.write[
                                            monitor.diskHistory.write.length - 1
                                        ] || 0,
                                    )}</span
                                >
                            </div>
                            <Graph
                                data={[
                                    monitor.diskHistory.timestamps,
                                    monitor.diskHistory.write,
                                ]}
                                series={[
                                    {
                                        label: "Write",
                                        stroke: "#f38ba8",
                                        fill: "#f38ba822",
                                    },
                                ]}
                                height={150}
                                yMin={0}
                                yLabel="B/s"
                            />
                        </div>
                    </div>
                </div>
            {/if}

            <!-- Partitions Grid -->
            <h2 class="section-title">Partitions</h2>
            <div class="partitions-grid">
                {#each disk.disks as diskItem}
                    {@const usagePercent =
                        (diskItem.used_space / diskItem.total_space) * 100}
                    {@const usageColor = getUsageColor(usagePercent)}

                    <div class="partition-card">
                        <div class="card-header">
                            <div class="disk-icon-wrapper">
                                <HardDrive size={20} />
                            </div>
                            <div class="disk-title">
                                <h3>{diskItem.name}</h3>
                                <div class="disk-meta">
                                    <span class="mount-badge"
                                        >{diskItem.mount_point}</span
                                    >
                                    <span class="fs-badge"
                                        >{diskItem.file_system}</span
                                    >
                                </div>
                            </div>
                        </div>

                        <div class="card-body">
                            <div class="usage-stats">
                                <div class="stat-row">
                                    <span>Used</span>
                                    <span class="font-mono"
                                        >{formatBytes(
                                            diskItem.used_space,
                                        )}</span
                                    >
                                </div>
                                <div class="stat-row">
                                    <span>Free</span>
                                    <span class="font-mono"
                                        >{formatBytes(
                                            diskItem.available_space,
                                        )}</span
                                    >
                                </div>
                                <div class="stat-row total-row">
                                    <span>Total</span>
                                    <span class="font-mono"
                                        >{formatBytes(
                                            diskItem.total_space,
                                        )}</span
                                    >
                                </div>
                            </div>

                            <div class="progress-section">
                                <div class="progress-labels">
                                    <span>Usage</span>
                                    <span style="color: {usageColor}"
                                        >{usagePercent.toFixed(1)}%</span
                                    >
                                </div>
                                <div class="progress-track">
                                    <div
                                        class="progress-fill"
                                        style="width: {usagePercent}%; background: {usageColor}; box-shadow: 0 0 10px {usageColor}40;"
                                    ></div>
                                </div>
                            </div>
                        </div>

                        <!-- SMART Info Footer -->
                        {#if diskItem.smart}
                            <div class="card-footer">
                                <div
                                    class="smart-status {diskItem.smart.health?.toLowerCase() ||
                                        'unknown'}"
                                >
                                    {#if diskItem.smart.health === "Passed"}
                                        <CheckCircle size={14} />
                                        <span>Good</span>
                                    {:else if diskItem.smart.health === "Failed"}
                                        <XCircle size={14} />
                                        <span>Bad</span>
                                    {:else}
                                        <AlertCircle size={14} />
                                        <span>Unknown</span>
                                    {/if}
                                </div>
                                {#if diskItem.smart.temperature}
                                    <div class="smart-temp">
                                        {diskItem.smart.temperature}°C
                                    </div>
                                {/if}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .page {
        min-height: 100vh;
        background: var(--md-sys-color-background);
        color: var(--md-sys-color-on-background);
        font-family: "Inter", system-ui, sans-serif;
    }

    .page-header {
        padding: 2.5rem 3rem 1.5rem;
        background: linear-gradient(
            to bottom,
            var(--md-sys-color-surface),
            transparent
        );
    }

    .header-content {
        display: flex;
        align-items: center;
        gap: 1.5rem;
        max-width: 1400px;
        margin: 0 auto;
    }

    .header-icon {
        padding: 1rem;
        background: var(--md-sys-color-primary-container);
        color: var(--md-sys-color-on-primary-container);
        border-radius: 16px;
    }

    .header-content h1 {
        margin: 0;
        font-size: 2rem;
        font-weight: 700;
        letter-spacing: -0.5px;
    }

    .header-content p {
        margin: 0.25rem 0 0;
        opacity: 0.7;
    }

    .page-content {
        padding: 0 3rem 3rem;
        max-width: 1400px;
        margin: 0 auto;
    }

    /* Summary Section */
    .summary-section {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 1.5rem;
        margin-bottom: 3rem;
    }

    .summary-card {
        background: var(--md-sys-color-surface-container);
        padding: 1.5rem;
        border-radius: 20px;
        display: flex;
        align-items: center;
        gap: 1.25rem;
        border: 1px solid var(--md-sys-color-outline-variant);
        transition:
            transform 0.2s,
            box-shadow 0.2s;
    }

    .summary-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }

    .summary-icon {
        width: 48px;
        height: 48px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 12px;
        background: var(--md-sys-color-surface);
        color: var(--md-sys-color-primary);
    }

    .summary-info {
        display: flex;
        flex-direction: column;
    }

    .summary-info .label {
        font-size: 0.875rem;
        opacity: 0.7;
        font-weight: 500;
    }

    .summary-info .value {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--md-sys-color-on-surface);
    }

    .value-group {
        display: flex;
        align-items: baseline;
        gap: 0.5rem;
    }

    .sub-value {
        font-size: 0.875rem;
        opacity: 0.5;
    }

    /* Grid Section */
    .section-title {
        font-size: 1.25rem;
        font-weight: 600;
        margin-bottom: 1.5rem;
        opacity: 0.9;
    }

    .partitions-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
        gap: 1.5rem;
    }

    .partition-card {
        background: var(--md-sys-color-surface-container-low);
        border-radius: 24px;
        padding: 1.5rem;
        border: 1px solid var(--md-sys-color-outline-variant);
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        position: relative;
        overflow: hidden;
        transition: all 0.2s ease;
    }

    .partition-card:hover {
        background: var(--md-sys-color-surface-container);
        border-color: var(--md-sys-color-outline);
        transform: translateY(-4px);
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
    }

    .card-header {
        display: flex;
        gap: 1rem;
        align-items: flex-start;
    }

    .disk-icon-wrapper {
        padding: 0.75rem;
        background: var(--md-sys-color-surface);
        border-radius: 12px;
        color: var(--md-sys-color-secondary);
    }

    .disk-title h3 {
        margin: 0 0 0.5rem;
        font-size: 1.1rem;
        font-weight: 600;
    }

    .disk-meta {
        display: flex;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .mount-badge,
    .fs-badge {
        font-size: 0.75rem;
        padding: 0.25rem 0.75rem;
        border-radius: 99px;
        background: var(--md-sys-color-surface-variant);
        color: var(--md-sys-color-on-surface-variant);
        font-family: "JetBrains Mono", monospace;
    }

    .card-body {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .usage-stats {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .stat-row {
        display: flex;
        justify-content: space-between;
        font-size: 0.875rem;
        opacity: 0.7;
    }

    .total-row {
        margin-top: 0.5rem;
        padding-top: 0.5rem;
        border-top: 1px solid var(--md-sys-color-outline-variant);
        font-weight: 600;
        opacity: 1;
    }

    .font-mono {
        font-family: "JetBrains Mono", monospace;
    }

    .progress-section {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .progress-labels {
        display: flex;
        justify-content: space-between;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        opacity: 0.8;
    }

    .progress-track {
        height: 8px;
        background: var(--md-sys-color-surface-variant);
        border-radius: 99px;
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        border-radius: 99px;
        transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .card-footer {
        margin-top: auto;
        padding-top: 1rem;
        border-top: 1px solid var(--md-sys-color-outline-variant);
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 0.875rem;
    }

    .smart-status {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 500;
    }

    .smart-status.passed {
        color: #22c55e;
    }
    .smart-status.failed {
        color: #ef4444;
    }
    .smart-status.unknown {
        color: #f59e0b;
    }

    .smart-temp {
        font-family: "JetBrains Mono", monospace;
        opacity: 0.8;
    }

    .loading-state,
    .error-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 400px;
        gap: 1rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .spinner {
        width: 40px;
        height: 40px;
        border: 3px solid var(--md-sys-color-surface-variant);
        border-top-color: var(--md-sys-color-primary);
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    /* Responsive adjustments */
    @media (max-width: 768px) {
        .page-content {
            padding: 1rem;
        }
        .page-header {
            padding: 1.5rem 1rem;
        }
    }

    /* Graph Styles */
    .graph-card {
        background: var(--md-sys-color-surface-container-low);
        border-radius: 24px;
        padding: 1.5rem;
        border: 1px solid var(--md-sys-color-outline-variant);
        margin-bottom: 3rem;
    }

    .graph-card h2 {
        font-size: 1.25rem;
        font-weight: 600;
        margin-bottom: 1.5rem;
        opacity: 0.9;
    }

    .graphs-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
        gap: 2rem;
    }

    .graph-item {
        background: var(--md-sys-color-surface);
        padding: 1rem;
        border-radius: 16px;
        border: 1px solid var(--md-sys-color-outline-variant);
    }

    .graph-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .graph-header h3 {
        font-size: 1rem;
        font-weight: 500;
        margin: 0;
        opacity: 0.8;
    }

    .rate-value {
        font-family: "JetBrains Mono", monospace;
        font-weight: 700;
        color: var(--md-sys-color-on-surface);
    }
</style>
