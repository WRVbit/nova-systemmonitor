<script lang="ts">
    import { Settings, Moon, Sun, Coffee } from "lucide-svelte";
    import {
        settings,
        setTheme,
        setRefreshInterval,
        toggleAutoRefresh,
    } from "$lib/stores/settings.svelte";
    import type { Theme } from "$lib/stores/settings.svelte";

    const themes: { id: Theme; label: string; icon: any }[] = [
        { id: "light", label: "Light", icon: Sun },
        { id: "dark", label: "Dark", icon: Moon },
        { id: "mocha", label: "Mocha", icon: Coffee },
    ];
</script>

<div class="page">
    <header class="page-header">
        <div class="header-title">
            <Settings size={32} />
            <div>
                <h1>Settings</h1>
                <p>Customize your experience</p>
            </div>
        </div>
    </header>

    <div class="page-content">
        <div class="card">
            <h2>Appearance</h2>
            <div class="setting-group">
                <div class="setting-label">Theme</div>
                <div class="theme-grid">
                    {#each themes as theme}
                        <button
                            class="theme-btn"
                            class:active={settings.theme === theme.id}
                            onclick={() => setTheme(theme.id)}
                        >
                            <svelte:component this={theme.icon} size={20} />
                            <span>{theme.label}</span>
                        </button>
                    {/each}
                </div>
            </div>

            <div class="divider"></div>

            <h2>Monitoring</h2>
            <div class="setting-group">
                <div class="setting-label">Refresh Interval (ms)</div>
                <div class="control-row">
                    <input
                        type="range"
                        min="1000"
                        max="10000"
                        step="500"
                        value={settings.refreshInterval}
                        onchange={(e) =>
                            setRefreshInterval(parseInt(e.currentTarget.value))}
                    />
                    <span class="value-display"
                        >{settings.refreshInterval}ms</span
                    >
                </div>
            </div>

            <div class="setting-group">
                <div class="setting-label">Auto Refresh</div>
                <div class="control-row">
                    <label class="switch">
                        <input
                            type="checkbox"
                            checked={settings.autoRefresh}
                            onchange={toggleAutoRefresh}
                        />
                        <span class="slider round"></span>
                    </label>
                    <span class="value-display"
                        >{settings.autoRefresh ? "Enabled" : "Disabled"}</span
                    >
                </div>
            </div>
        </div>
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

    .page-content {
        padding: 2rem;
        max-width: 800px;
        margin: 0 auto;
    }

    .card {
        background: var(--md-sys-color-surface);
        border-radius: 16px;
        padding: 2rem;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    h2 {
        margin: 0 0 1.5rem 0;
        font-size: 1.25rem;
        color: var(--md-sys-color-primary);
    }

    .setting-group {
        margin-bottom: 2rem;
    }

    .setting-label {
        font-weight: 500;
        margin-bottom: 1rem;
        color: var(--md-sys-color-on-surface);
    }

    .theme-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
        gap: 1rem;
    }

    .theme-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        padding: 1rem;
        background: var(--md-sys-color-surface-variant);
        border: 2px solid transparent;
        border-radius: 12px;
        color: var(--md-sys-color-on-surface-variant);
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .theme-btn:hover {
        background: var(--md-sys-color-outline);
    }

    .theme-btn.active {
        background: var(--md-sys-color-primary);
        color: var(--md-sys-color-on-primary);
        border-color: var(--md-sys-color-primary);
    }

    .divider {
        height: 1px;
        background: var(--md-sys-color-outline);
        margin: 2rem 0;
    }

    .control-row {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .value-display {
        font-family: "JetBrains Mono", monospace;
        color: var(--md-sys-color-on-surface-variant);
    }

    input[type="range"] {
        flex: 1;
        accent-color: var(--md-sys-color-primary);
    }

    /* Toggle Switch */
    .switch {
        position: relative;
        display: inline-block;
        width: 50px;
        height: 24px;
    }

    .switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .slider {
        position: absolute;
        cursor: pointer;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: var(--md-sys-color-outline);
        transition: 0.4s;
        border-radius: 24px;
    }

    .slider:before {
        position: absolute;
        content: "";
        height: 16px;
        width: 16px;
        left: 4px;
        bottom: 4px;
        background-color: white;
        transition: 0.4s;
        border-radius: 50%;
    }

    input:checked + .slider {
        background-color: var(--md-sys-color-primary);
    }

    input:checked + .slider:before {
        transform: translateX(26px);
    }
</style>
