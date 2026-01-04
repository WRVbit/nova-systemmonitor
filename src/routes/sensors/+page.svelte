<!-- Nova System Monitor - Sensors View -->
<script lang="ts">
    import { onMount } from "svelte";
    import { Thermometer } from "lucide-svelte";
    import * as monitor from "$lib/stores/monitor.svelte";

    let intervalId: number | undefined;
    let mounted = $state(false);

    onMount(() => {
        mounted = true;
        // Delay initial fetch slightly to prevent UI freeze on mount
        const initialTimeout = setTimeout(() => {
            monitor.fetchSensorsInfo();
        }, 100);
        
        // Slower refresh interval (5 seconds) to prevent overloading
        intervalId = setInterval(() => {
            if (mounted) {
                monitor.fetchSensorsInfo();
            }
        }, 5000) as unknown as number;

        return () => {
            mounted = false;
            clearTimeout(initialTimeout);
            if (intervalId !== undefined) {
                clearInterval(intervalId);
                intervalId = undefined;
            }
        };
    });

    function getTempColor(temp: number): string {
        if (temp >= 80) return "#f38ba8"; // Red - Hot
        if (temp >= 60) return "#f9e2af"; // Yellow - Warm
        return "#a6e3a1"; // Green - Cool
    }

    function getTempStatus(temp: number): string {
        if (temp >= 80) return "Hot";
        if (temp >= 60) return "Warm";
        return "Cool";
    }
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <Thermometer size={32} />
            <div>
                <h1>Sensors</h1>
                <p>Temperature Monitoring</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.sensorsInfo.loading && !monitor.sensorsInfo.data}
            <div class="loading">Loading sensors information...</div>
        {:else if monitor.sensorsInfo.error}
            <div class="error">Error: {monitor.sensorsInfo.error}</div>
        {:else if monitor.sensorsInfo.data}
            {@const sensors = monitor.sensorsInfo.data}

            {#if sensors.sensors.length === 0}
                <div class="card no-sensors">
                    <h2>No Sensors Detected</h2>
                    <p>No temperature sensors found on this system.</p>
                </div>
            {:else}
                <div class="sensors-grid">
                    {#each sensors.sensors as sensor}
                        <div
                            class="card sensor-card"
                            style="border-left: 4px solid {getTempColor(
                                sensor.value,
                            )}"
                        >
                            <div class="sensor-header">
                                <h3>{sensor.label}</h3>
                                <span class="sensor-type"
                                    >{sensor.sensor_type}</span
                                >
                            </div>
                            <div
                                class="temp-display"
                                style="color: {getTempColor(sensor.value)}"
                            >
                                {sensor.value.toFixed(1)}°C
                            </div>
                            <div
                                class="temp-status"
                                style="color: {getTempColor(sensor.value)}"
                            >
                                {getTempStatus(sensor.value)}
                            </div>
                            {#if (sensor.critical_value !== null && sensor.critical_value !== undefined) || (sensor.max_value !== null && sensor.max_value !== undefined)}
                                <div class="sensor-limits">
                                    {#if sensor.max_value !== null && sensor.max_value !== undefined}
                                        <div class="limit-item">
                                            <span class="limit-label">Max:</span>
                                            <span class="limit-value">{sensor.max_value}°C</span>
                                        </div>
                                    {/if}
                                    {#if sensor.critical_value !== null && sensor.critical_value !== undefined}
                                        <div class="limit-item">
                                            <span class="limit-label">Critical:</span>
                                            <span class="limit-value" style="color: #f38ba8;">{sensor.critical_value}°C</span>
                                        </div>
                                    {/if}
                                </div>
                            {/if}
                        </div>
                    {/each}
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

    .sensors-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
        gap: 1.5rem;
    }

    .card {
        background: var(--md-sys-color-surface);
        border-radius: 16px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .sensor-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .sensor-header {
        width: 100%;
        margin-bottom: 1rem;
    }

    .sensor-header h3 {
        margin: 0 0 0.5rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .sensor-type {
        font-size: 0.75rem;
        color: var(--md-sys-color-on-surface-variant);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .temp-display {
        font-size: 3.5rem;
        font-weight: 700;
        margin: 1rem 0 0.5rem 0;
    }

    .temp-status {
        font-size: 1.25rem;
        font-weight: 600;
        margin-bottom: 1rem;
    }

    .sensor-limits {
        width: 100%;
        padding-top: 1rem;
        border-top: 1px solid var(--md-sys-color-outline);
    }

    .limit-item {
        display: flex;
        justify-content: space-between;
        font-size: 0.875rem;
    }

    .limit-label {
        color: var(--md-sys-color-on-surface-variant);
    }

    .limit-value {
        font-family: "JetBrains Mono", monospace;
        font-weight: 600;
        color: var(--md-sys-color-on-surface);
    }

    .no-sensors {
        text-align: center;
        padding: 3rem;
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
