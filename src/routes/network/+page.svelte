<!-- Nova System Monitor - Network Detail View -->
<script lang="ts">
    import { onMount } from "svelte";
    import { Network, ArrowDown, ArrowUp } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";
    import Graph from "$lib/components/Graph.svelte";

    type RateUnit = "bytes" | "bits" | "mb";
    let rateUnit = $state<RateUnit>("bytes");
    let intervalId: number | undefined;

    onMount(() => {
        monitor.fetchNetworkInfo();
        intervalId = setInterval(() => {
            monitor.fetchNetworkInfo();
        }, 3000) as unknown as number; // 3 seconds for smoother UI

        // Cleanup on unmount
        return () => {
            if (intervalId !== undefined) {
                clearInterval(intervalId);
                intervalId = undefined;
            }
        };
    });

    function formatRate(bytesPerSec: number, unit: RateUnit): string {
        if (unit === "bits") {
            const bits = bytesPerSec * 8;
            if (bits >= 1_000_000_000)
                return `${(bits / 1_000_000_000).toFixed(2)} Gbps`;
            if (bits >= 1_000_000)
                return `${(bits / 1_000_000).toFixed(2)} Mbps`;
            if (bits >= 1_000) return `${(bits / 1_000).toFixed(2)} Kbps`;
            return `${bits.toFixed(0)} bps`;
        } else if (unit === "mb") {
            const mb = bytesPerSec / (1024 * 1024);
            return `${mb.toFixed(2)} MB/s`;
        } else {
            // bytes
            if (bytesPerSec >= 1_073_741_824)
                return `${(bytesPerSec / 1_073_741_824).toFixed(2)} GB/s`;
            if (bytesPerSec >= 1_048_576)
                return `${(bytesPerSec / 1_048_576).toFixed(2)} MB/s`;
            if (bytesPerSec >= 1024)
                return `${(bytesPerSec / 1024).toFixed(2)} KB/s`;
            return `${bytesPerSec.toFixed(0)} B/s`;
        }
    }

    function formatBytes(bytes: number): string {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
    }
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <Network size={32} />
            <div>
                <h1>Network</h1>
                <p>Interface Statistics</p>
            </div>
        </div>
        <div class="header-controls">
            <div class="rate-toggle">
                <button
                    class="toggle-btn"
                    class:active={rateUnit === "bytes"}
                    onclick={() => (rateUnit = "bytes")}
                >
                    B/s
                </button>
                <button
                    class="toggle-btn"
                    class:active={rateUnit === "bits"}
                    onclick={() => (rateUnit = "bits")}
                >
                    bps
                </button>
                <button
                    class="toggle-btn"
                    class:active={rateUnit === "mb"}
                    onclick={() => (rateUnit = "mb")}
                >
                    MB/s
                </button>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.networkInfo.loading && !monitor.networkInfo.data}
            <div class="loading">Loading network information...</div>
        {:else if monitor.networkInfo.error}
            <div class="error">Error: {monitor.networkInfo.error}</div>
        {:else if monitor.networkInfo.data}
            {@const net = monitor.networkInfo.data}

            <!-- Network History -->
            {#if monitor.networkHistory.timestamps.length > 1}
                <div class="card history-card">
                    <h2>Network History</h2>
                    <div class="graphs-grid">
                        <div class="graph-item">
                            <h3>Total Download Rate</h3>
                            <Graph
                                data={[
                                    monitor.networkHistory.timestamps,
                                    monitor.networkHistory.download,
                                ]}
                                series={[
                                    {
                                        label: "Download",
                                        stroke: "#94e2d5",
                                        fill: "#94e2d522",
                                    },
                                ]}
                                height={150}
                                yMin={0}
                                yLabel="B/s"
                            />
                        </div>
                        <div class="graph-item">
                            <h3>Total Upload Rate</h3>
                            <Graph
                                data={[
                                    monitor.networkHistory.timestamps,
                                    monitor.networkHistory.upload,
                                ]}
                                series={[
                                    {
                                        label: "Upload",
                                        stroke: "#cba6f7",
                                        fill: "#cba6f722",
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

            <!-- Summary -->
            <div class="card summary-card">
                <h2>Total Network Activity</h2>
                <div class="total-stats">
                    <div class="total-stat download">
                        <ArrowDown size={24} />
                        <div>
                            <span class="total-label">Download</span>
                            <span class="total-value"
                                >{formatRate(
                                    net.total_download_rate || 0,
                                    rateUnit,
                                )}</span
                            >
                        </div>
                    </div>
                    <div class="total-stat upload">
                        <ArrowUp size={24} />
                        <div>
                            <span class="total-label">Upload</span>
                            <span class="total-value"
                                >{formatRate(
                                    net.total_upload_rate || 0,
                                    rateUnit,
                                )}</span
                            >
                        </div>
                    </div>
                </div>
            </div>

            <!-- Per-Interface -->
            <div class="interfaces-grid">
                {#each net.interfaces as iface}
                    <div class="card interface-card">
                        <div class="interface-header">
                            <h3>{iface.name}</h3>
                            <span class="status" class:active={iface.download_rate_bps > 0 || iface.upload_rate_bps > 0}>
                                {(iface.download_rate_bps > 0 || iface.upload_rate_bps > 0) ? "Active" : "Idle"}
                            </span>
                        </div>

                        <!-- Rates -->
                        <div class="rates">
                            <div class="rate-item download">
                                <ArrowDown size={20} />
                                <div>
                                    <span class="rate-label">Download</span>
                                    <span class="rate-value"
                                        >{formatRate(
                                            iface.download_rate_bps || 0,
                                            rateUnit,
                                        )}</span
                                    >
                                </div>
                            </div>
                            <div class="rate-item upload">
                                <ArrowUp size={20} />
                                <div>
                                    <span class="rate-label">Upload</span>
                                    <span class="rate-value"
                                        >{formatRate(
                                            iface.upload_rate_bps || 0,
                                            rateUnit,
                                        )}</span
                                    >
                                </div>
                            </div>
                        </div>

                        <!-- Total Transferred -->
                        <div class="totals">
                            <div class="total-item">
                                <span class="total-label"
                                    >Total Downloaded:</span
                                >
                                <span class="total-value"
                                    >{formatBytes(iface.received_bytes || 0)}</span
                                >
                            </div>
                            <div class="total-item">
                                <span class="total-label">Total Uploaded:</span>
                                <span class="total-value"
                                    >{formatBytes(
                                        iface.transmitted_bytes || 0,
                                    )}</span
                                >
                            </div>
                            <div class="total-item">
                                <span class="total-label">Packets RX:</span>
                                <span class="total-value"
                                    >{(iface.received_packets || 0).toLocaleString()}</span
                                >
                            </div>
                            <div class="total-item">
                                <span class="total-label">Packets TX:</span>
                                <span class="total-value"
                                    >{(iface.transmitted_packets || 0).toLocaleString()}</span
                                >
                            </div>
                            <div class="total-item">
                                <span class="total-label">Errors RX:</span>
                                <span class="total-value error-count"
                                    >{(iface.errors_in || 0).toLocaleString()}</span
                                >
                            </div>
                            <div class="total-item">
                                <span class="total-label">Errors TX:</span>
                                <span class="total-value error-count"
                                    >{(iface.errors_out || 0).toLocaleString()}</span
                                >
                            </div>
                        </div>
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
    }

    .page-header {
        padding: 2rem;
        background: var(--md-sys-color-surface);
        border-bottom: 1px solid var(--md-sys-color-outline);
        display: flex;
        justify-content: space-between;
        align-items: center;
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

    .rate-toggle {
        display: flex;
        gap: 0.5rem;
    }

    .toggle-btn {
        padding: 0.5rem 1rem;
        background: var(--md-sys-color-surface-variant);
        border: 1px solid var(--md-sys-color-outline);
        border-radius: 8px;
        color: var(--md-sys-color-on-surface);
        cursor: pointer;
        transition: background 80ms ease-out, transform 80ms ease-out;
        font-size: 0.875rem;
        font-weight: 500;
    }

    .toggle-btn:hover {
        background: var(--md-sys-color-outline);
    }

    .toggle-btn.active {
        background: var(--md-sys-color-primary);
        color: var(--md-sys-color-on-primary);
        border-color: var(--md-sys-color-primary);
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
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        margin-bottom: 1.5rem;
    }

    .card h2,
    .card h3 {
        margin: 0 0 1rem 0;
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .total-stats {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 2rem;
    }

    .total-stat {
        display: flex;
        align-items: center;
        gap: 1.5rem;
        padding: 1rem;
        background: var(--md-sys-color-surface-variant);
        border-radius: 12px;
    }

    .total-stat.download {
        color: #94e2d5;
    }

    .total-stat.upload {
        color: #cba6f7;
    }

    .total-label {
        display: block;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .total-value {
        display: block;
        font-size: 1.75rem;
        font-weight: 700;
        color: var(--md-sys-color-on-surface);
    }

    .interfaces-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
        gap: 1.5rem;
    }

    .interface-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
    }

    .status {
        padding: 0.25rem 0.75rem;
        border-radius: 12px;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        background: var(--md-sys-color-surface-variant);
        color: var(--md-sys-color-on-surface-variant);
    }

    .status.active {
        background: rgba(166, 227, 161, 0.2);
        color: #a6e3a1;
    }

    .rates {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .rate-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 1rem;
        background: var(--md-sys-color-surface-variant);
        border-radius: 8px;
    }

    .rate-item.download {
        color: #94e2d5;
    }

    .rate-item.upload {
        color: #cba6f7;
    }

    .rate-label {
        display: block;
        font-size: 0.75rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .rate-value {
        display: block;
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--md-sys-color-on-surface);
    }

    .totals {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 0.75rem;
    }

    .total-item {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .total-item .total-label {
        font-size: 0.75rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .total-item .total-value {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
        font-family: "JetBrains Mono", monospace;
    }

    .error-count {
        color: #f38ba8;
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

    /* Removed unused selector .graphs-grid */
    /* Removed unused selector .graph-item h3 */
</style>
