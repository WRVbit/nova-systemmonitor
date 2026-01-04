<script lang="ts">
    import {
        X,
        Cpu,
        HardDrive,
        Clock,
        Terminal,
        User,
        Activity,
    } from "lucide-svelte";
    import { fade, scale } from "svelte/transition";

    // Define interface locally or import if shared (for now locally to match main page)
    interface ProcessNode {
        pid: number;
        parent_pid: number | null;
        name: string;
        exe_path: string;
        command: string[];
        status: { type: string } | string;
        normalized_status: string;
        cpu_usage: number;
        memory_bytes: number;
        memory_percent: number;
        start_time: number;
        run_time: number;
        user_id: string | null;
        normalized_user: string;
        nice: number;
    }

    let { process, onClose } = $props<{
        process: ProcessNode;
        onClose: () => void;
    }>();

    function formatBytes(bytes: number): string {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
    }

    function formatTime(timestamp: number): string {
        return new Date(timestamp * 1000).toLocaleString();
    }

    function formatDuration(seconds: number): string {
        const h = Math.floor(seconds / 3600);
        const m = Math.floor((seconds % 3600) / 60);
        const s = seconds % 60;
        return `${h}h ${m}m ${s}s`;
    }

    // Close on escape
    function onKeyDown(e: KeyboardEvent) {
        if (e.key === "Escape") onClose();
    }
</script>

<svelte:window onkeydown={onKeyDown} />

<div
    class="modal-backdrop"
    transition:fade={{ duration: 200 }}
    onclick={onClose}
    role="button"
    tabindex="0"
    onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") onClose();
    }}
    aria-label="Close modal"
>
    <div
        class="modal-card"
        transition:scale={{ duration: 200, start: 0.95 }}
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
        tabindex="-1"
        onkeydown={(e) => e.stopPropagation()}
    >
        <header class="modal-header">
            <div class="header-info">
                <div class="icon-placeholder">
                    <Activity size={24} />
                </div>
                <div>
                    <h2 id="modal-title">{process.name}</h2>
                    <span class="pid">PID: {process.pid}</span>
                </div>
            </div>
            <button class="close-btn" onclick={onClose}>
                <X size={20} />
            </button>
        </header>

        <div class="modal-content">
            <div class="grid-section">
                <div class="info-item">
                    <div class="label"><Cpu size={14} /> CPU Usage</div>
                    <div class="value">{process.cpu_usage.toFixed(1)}%</div>
                </div>
                <div class="info-item">
                    <div class="label"><HardDrive size={14} /> Memory</div>
                    <div class="value">
                        {formatBytes(process.memory_bytes)} ({process.memory_percent.toFixed(
                            1,
                        )}%)
                    </div>
                </div>
                <div class="info-item">
                    <div class="label"><User size={14} /> User</div>
                    <div class="value">{process.normalized_user}</div>
                </div>
                <div class="info-item">
                    <div class="label">
                        <Activity size={14} /> Priority (Nice)
                    </div>
                    <div class="value">{process.nice ?? "N/A"}</div>
                </div>
            </div>

            <div class="divider"></div>

            <div class="detail-group">
                <h3>Execution Details</h3>
                <div class="detail-row">
                    <span class="detail-label"><Clock size={14} /> Started</span
                    >
                    <span class="detail-value"
                        >{formatTime(process.start_time)} (Running for {formatDuration(
                            process.run_time,
                        )})</span
                    >
                </div>
                <div class="detail-row">
                    <span class="detail-label"><Terminal size={14} /> Path</span
                    >
                    <code class="path-code">{process.exe_path || "N/A"}</code>
                </div>
            </div>

            <div class="detail-group">
                <h3>Command Line</h3>
                <div class="command-box">
                    {#if process.command && process.command.length > 0}
                        <code class="cmd-code">
                            {#each process.command as arg}
                                <span class="arg">{arg}</span>{" "}
                            {/each}
                        </code>
                    {:else}
                        <span class="text-muted">No arguments available</span>
                    {/if}
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        backdrop-filter: blur(4px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 100;
    }

    .modal-card {
        background: var(--md-sys-color-surface);
        width: 100%;
        max-width: 600px;
        max-height: 90vh;
        border-radius: 24px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        border: 1px solid var(--md-sys-color-outline-variant);
    }

    .modal-header {
        padding: 1.5rem;
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        background: var(--md-sys-color-surface-variant);
    }

    .header-info {
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    .icon-placeholder {
        width: 48px;
        height: 48px;
        border-radius: 12px;
        background: var(--md-sys-color-primary);
        color: var(--md-sys-color-on-primary);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .header-info h2 {
        margin: 0;
        font-size: 1.5rem;
        color: var(--md-sys-color-on-surface);
    }

    .pid {
        font-family: "JetBrains Mono", monospace;
        color: var(--md-sys-color-on-surface-variant);
        font-size: 0.875rem;
    }

    .close-btn {
        background: transparent;
        border: none;
        color: var(--md-sys-color-on-surface-variant);
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: background 0.2s;
    }

    .close-btn:hover {
        background: rgba(255, 255, 255, 0.1);
        color: var(--md-sys-color-on-surface);
    }

    .modal-content {
        padding: 1.5rem;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .grid-section {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 1rem;
    }

    .info-item {
        background: var(--md-sys-color-surface-variant);
        padding: 1rem;
        border-radius: 12px;
    }

    .label {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.75rem;
        color: var(--md-sys-color-on-surface-variant);
        margin-bottom: 0.25rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .value {
        font-size: 1.125rem;
        font-weight: 500;
        color: var(--md-sys-color-on-surface);
    }

    .divider {
        height: 1px;
        background: var(--md-sys-color-outline-variant);
    }

    .detail-group h3 {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        color: var(--md-sys-color-primary);
    }

    .detail-row {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        margin-bottom: 1rem;
    }

    .detail-label {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .detail-value {
        padding-left: 1.4rem;
        color: var(--md-sys-color-on-surface);
        font-family: "JetBrains Mono", monospace;
        font-size: 0.875rem;
    }

    .path-code {
        padding: 0.5rem;
        background: var(--md-sys-color-surface-variant);
        border-radius: 8px;
        font-family: "JetBrains Mono", monospace;
        font-size: 0.875rem;
        word-break: break-all;
        color: var(--md-sys-color-on-surface);
    }

    .command-box {
        background: #1e1e2e;
        border-radius: 12px;
        padding: 1rem;
        border: 1px solid var(--md-sys-color-outline-variant);
    }

    .cmd-code {
        font-family: "JetBrains Mono", monospace;
        font-size: 0.875rem;
        color: #cdd6f4;
        line-height: 1.5;
        word-break: break-all;
    }

    .arg {
        display: inline-block;
    }

    .text-muted {
        color: var(--md-sys-color-on-surface-variant);
        font-style: italic;
    }
</style>
