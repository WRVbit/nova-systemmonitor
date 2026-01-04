<!-- Nova System Monitor - System Info View -->
<script lang="ts">
    import { onMount } from "svelte";
    import { Info } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";

    onMount(() => {
        monitor.fetchSystemInfo();
    });

    function formatUptime(seconds: number): string {
        const days = Math.floor(seconds / 86400);
        const hours = Math.floor((seconds % 86400) / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);

        const parts = [];
        if (days > 0) parts.push(`${days}d`);
        if (hours > 0) parts.push(`${hours}h`);
        if (minutes > 0) parts.push(`${minutes}m`);

        return parts.join(" ") || "0m";
    }

    function formatDate(timestamp: number): string {
        return new Date(timestamp * 1000).toLocaleString();
    }
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <Info size={32} />
            <div>
                <h1>System</h1>
                <p>System Information</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.systemInfo.loading}
            <div class="loading">Loading system information...</div>
        {:else if monitor.systemInfo.error}
            <div class="error">Error: {monitor.systemInfo.error}</div>
        {:else if monitor.systemInfo.data}
            {@const sys = monitor.systemInfo.data}

            <!-- Operating System -->
            <div class="card">
                <h2>Operating System</h2>
                <div class="info-grid">
                    <div class="info-item">
                        <span class="label">OS Name:</span>
                        <span class="value">{sys.os_name}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">OS Version:</span>
                        <span class="value">{sys.os_version}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">Kernel Version:</span>
                        <span class="value">{sys.kernel_version}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">Architecture:</span>
                        <span class="value">{sys.architecture}</span>
                    </div>
                </div>
            </div>

            <!-- Host Information -->
            <div class="card">
                <h2>Host Information</h2>
                <div class="info-grid">
                    <div class="info-item">
                        <span class="label">Hostname:</span>
                        <span class="value">{sys.hostname}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">Uptime:</span>
                        <span class="value">{formatUptime(sys.uptime)}</span>
                    </div>
                    <div class="info-item">
                        <span class="label">Boot Time:</span>
                        <span class="value">{formatDate(sys.boot_time)}</span>
                    </div>
                </div>
            </div>

            <!-- Nova System Monitor-->
            <div class="card about-card">
                <h2>About Nova System Monitor</h2>
                <div class="about-content">
                    <div class="logo">
                        <h1>Nova</h1>
                        <p>System Monitor</p>
                    </div>
                    <div class="about-info">
                        <p><strong>Version:</strong> 1.0.0</p>
                        <p><strong>Backend:</strong> Rust + Tauri v2</p>
                        <p><strong>Frontend:</strong> Svelte 5</p>
                        <p><strong>License:</strong> MIT</p>
                    </div>
                    <div class="features">
                        <h3>Features</h3>
                        <ul>
                            <li>✅ Real-time CPU, Memory, GPU monitoring</li>
                            <li>✅ SMART disk health monitoring</li>
                            <li>✅ Multi-GPU support (NVIDIA, AMD, Intel)</li>
                            <li>✅ Network rate visualization</li>
                            <li>✅ Process management</li>
                            <li>✅ Temperature sensors</li>
                        </ul>
                    </div>
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
        max-width: 1000px;
        margin: 0 auto;
    }

    .card {
        background: var(--md-sys-color-surface);
        border-radius: 16px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        margin-bottom: 1.5rem;
    }

    .card h2 {
        margin: 0 0 1.5rem 0;
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .info-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 1rem;
    }

    .info-item {
        display: flex;
        justify-content: space-between;
        padding: 1rem;
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
        font-weight: 600;
    }

    .about-content {
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .logo {
        text-align: center;
        padding: 2rem;
        background: linear-gradient(135deg, #89b4fa, #cba6f7);
        border-radius: 12px;
        color: white;
    }

    .logo h1 {
        margin: 0;
        font-size: 3rem;
        font-weight: 800;
    }

    .logo p {
        margin: 0.5rem 0 0 0;
        font-size: 1.25rem;
        opacity: 0.9;
    }

    .about-info p {
        margin: 0.5rem 0;
        color: var(--md-sys-color-on-surface-variant);
    }

    .about-info strong {
        color: var(--md-sys-color-on-surface);
    }

    .features h3 {
        margin: 0 0 1rem 0;
        font-size: 1.125rem;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .features ul {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .features li {
        padding: 0.5rem 0;
        color: var(--md-sys-color-on-surface);
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
