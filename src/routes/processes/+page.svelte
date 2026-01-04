<!-- Nova System Monitor - Processes View -->
<script lang="ts">
    import { onMount } from "svelte";
    import { List, Search, ListTree, ListFilter } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import * as monitor from "$lib/stores/monitor.svelte";
    import ProcessDetailsModal from "$lib/components/ProcessDetailsModal.svelte";

    // Match backend ProcessInfo structure exactly
    interface Process {
        pid: number;
        parent_pid: number | null;
        name: string;
        exe_path: string;
        command: string[];
        status: { type: string } | string;
        cpu_usage: number;
        memory_bytes: number;
        memory_percent: number;
        start_time: number;
        run_time: number;
        user_id: string | null;
        nice: number;
    }

    // Extended interface for Tree View
    interface ProcessNode extends Process {
        children: ProcessNode[];
        depth: number;
        is_expanded: boolean;
        normalized_status: string;
        normalized_user: string;
    }

    // Helper to normalize process for sorting/display
    function normalizeProcess(p: Process): ProcessNode {
        return {
            ...p,
            children: [],
            depth: 0,
            is_expanded: true,
            normalized_status:
                typeof p.status === "string"
                    ? p.status
                    : p.status?.type || "Unknown",
            normalized_user: p.user_id || "Unknown",
        };
    }

    let searchQuery = $state("");
    let viewMode = $state<"flat" | "tree">("flat");
    let sortBy = $state<
        "pid" | "name" | "cpu_usage" | "memory_bytes" | "status" | "user"
    >("cpu_usage");
    let sortOrder = $state<"asc" | "desc">("desc");
    let intervalId: number;

    // Modal state
    let selectedProcess = $state<ProcessNode | null>(null);

    // Cache expanded state for tree view so it persists across refreshes
    let expandedState = new Map<number, boolean>();

    onMount(() => {
        monitor.fetchProcessInfo();
        intervalId = setInterval(() => {
            monitor.fetchProcessInfo();
        }, 5000) as unknown as number; // Reduced to 5s for performance

        return () => clearInterval(intervalId);
    });

    function formatBytes(bytes: number): string {
        if (bytes === 0) return "0 B";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
    }

    function sortColumn(
        column:
            | "pid"
            | "name"
            | "cpu_usage"
            | "memory_bytes"
            | "status"
            | "user",
    ) {
        if (sortBy === column) {
            sortOrder = sortOrder === "asc" ? "desc" : "asc";
        } else {
            sortBy = column;
            sortOrder = "desc";
        }
    }

    function compareProcesses(a: ProcessNode, b: ProcessNode): number {
        let aVal: any =
            a[
                sortBy === "status"
                    ? "normalized_status"
                    : sortBy === "user"
                      ? "normalized_user"
                      : sortBy
            ];
        let bVal: any =
            b[
                sortBy === "status"
                    ? "normalized_status"
                    : sortBy === "user"
                      ? "normalized_user"
                      : sortBy
            ];

        if (sortBy === "name") {
            aVal = aVal.toLowerCase();
            bVal = bVal.toLowerCase();
        }

        const compare = aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
        return sortOrder === "asc" ? compare : -compare;
    }

    // Recursive function to flatten the tree for rendering
    function flattenTree(nodes: ProcessNode[], result: ProcessNode[]) {
        for (const node of nodes) {
            result.push(node);
            if (node.children.length > 0 && node.is_expanded) {
                // Add is_expanded logic later if we add collapse
                flattenTree(node.children, result);
            }
        }
    }

    function buildProcessTree(flatProcesses: ProcessNode[]): ProcessNode[] {
        const processMap = new Map<number, ProcessNode>();
        const roots: ProcessNode[] = [];

        // 1. Initialize all nodes
        flatProcesses.forEach((p) => {
            p.children = [];
            p.depth = 0;
            // For now, always expanded. Could check expandedState map here.
            processMap.set(p.pid, p);
        });

        // 2. Build Hierarchy
        flatProcesses.forEach((p) => {
            if (p.parent_pid !== null && processMap.has(p.parent_pid)) {
                const parent = processMap.get(p.parent_pid)!;
                parent.children.push(p);
            } else {
                roots.push(p);
            }
        });

        // 3. Sort children at each level
        const sortRecursive = (nodes: ProcessNode[], depth: number) => {
            nodes.sort(compareProcesses);
            nodes.forEach((node) => {
                node.depth = depth;
                if (node.children.length > 0) {
                    sortRecursive(node.children, depth + 1);
                }
            });
        };

        sortRecursive(roots, 0);
        return roots;
    }

    function getProcesses() {
        if (!monitor.processInfo.data) return [];

        let rawProcesses: ProcessNode[] =
            monitor.processInfo.data.processes.map(normalizeProcess);

        // Filter first
        if (searchQuery) {
            const query = searchQuery.toLowerCase();
            rawProcesses = rawProcesses.filter(
                (p: ProcessNode) =>
                    p.name.toLowerCase().includes(query) ||
                    p.pid.toString().includes(query) ||
                    (typeof p.user_id === "string" &&
                        p.user_id.toLowerCase().includes(query)),
            );
        }

        if (viewMode === "tree") {
            const treeRoots = buildProcessTree(rawProcesses);
            const flattenedTree: ProcessNode[] = [];
            flattenTree(treeRoots, flattenedTree);
            return flattenedTree;
        } else {
            // Flat mode
            return rawProcesses.sort(compareProcesses);
        }
    }

    async function killProcess(pid: number, force: boolean) {
        try {
            await invoke("kill_process", { pid, force });
            // Trigger immediate refresh logic is handled by interval,
            // but we could optimistically remove it from UI if needed.
            // For now, just let the next interval catch it or manually trigger:
            monitor.fetchProcessInfo();
        } catch (e: any) {
            alert(`Failed to kill process: ${e}`);
        }
    }

    function openDetails(process: ProcessNode) {
        selectedProcess = process;
    }
</script>

<div class="page">
    {#if selectedProcess}
        <ProcessDetailsModal
            process={selectedProcess}
            onClose={() => (selectedProcess = null)}
        />
    {/if}

    <header class="page-header">
        <div class="header-title">
            <List size={32} />
            <div>
                <h1>Processes</h1>
                <p>Running Applications & Services</p>
            </div>
        </div>
        <div class="header-controls">
            <div class="view-toggle">
                <button
                    class="toggle-btn"
                    class:active={viewMode === "flat"}
                    onclick={() => (viewMode = "flat")}
                    title="Flat View"
                >
                    <ListFilter size={18} />
                    <span>Flat</span>
                </button>
                <button
                    class="toggle-btn"
                    class:active={viewMode === "tree"}
                    onclick={() => (viewMode = "tree")}
                    title="Tree View"
                >
                    <ListTree size={18} />
                    <span>Tree</span>
                </button>
            </div>

            <div class="search-box">
                <Search size={18} />
                <input
                    type="text"
                    placeholder="Search processes..."
                    bind:value={searchQuery}
                />
            </div>
        </div>
    </header>

    <div class="page-content">
        {#if monitor.processInfo.loading && !monitor.processInfo.data}
            <div class="loading">Loading process information...</div>
        {:else if monitor.processInfo.error}
            <div class="error">Error: {monitor.processInfo.error}</div>
        {:else if monitor.processInfo.data}
            {@const processes = getProcesses()}

            <div class="card">
                <div class="table-container">
                    <table>
                        <thead>
                            <tr>
                                <th
                                    onclick={() => sortColumn("pid")}
                                    style="width: 80px;"
                                >
                                    PID {sortBy === "pid"
                                        ? sortOrder === "asc"
                                            ? "↑"
                                            : "↓"
                                        : ""}
                                </th>
                                <th onclick={() => sortColumn("name")}>
                                    Name {sortBy === "name"
                                        ? sortOrder === "asc"
                                            ? "↑"
                                            : "↓"
                                        : ""}
                                </th>
                                <th
                                    onclick={() => sortColumn("cpu_usage")}
                                    style="width: 100px;"
                                >
                                    CPU% {sortBy === "cpu_usage"
                                        ? sortOrder === "asc"
                                            ? "↑"
                                            : "↓"
                                        : ""}
                                </th>
                                <th
                                    onclick={() => sortColumn("memory_bytes")}
                                    style="width: 120px;"
                                >
                                    Memory {sortBy === "memory_bytes"
                                        ? sortOrder === "asc"
                                            ? "↑"
                                            : "↓"
                                        : ""}
                                </th>
                                <th
                                    onclick={() => sortColumn("status")}
                                    style="width: 100px;"
                                >
                                    Status {sortBy === "status"
                                        ? sortOrder === "asc"
                                            ? "↑"
                                            : "↓"
                                        : ""}
                                </th>
                                <th
                                    onclick={() => sortColumn("user")}
                                    style="width: 120px;"
                                >
                                    User {sortBy === "user"
                                        ? sortOrder === "asc"
                                            ? "↑"
                                            : "↓"
                                        : ""}
                                </th>
                                <th style="width: 140px;">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each processes.slice(0, viewMode === "tree" ? undefined : 100) as process}
                                <tr
                                    class="process-row clickable"
                                    ondblclick={() => openDetails(process)}
                                >
                                    <td class="pid">{process.pid}</td>
                                    <td class="name">
                                        <div
                                            class="name-cell"
                                            style="padding-left: {viewMode ===
                                            'tree'
                                                ? process.depth * 20
                                                : 0}px"
                                        >
                                            {#if viewMode === "tree" && process.depth > 0}
                                                <span class="tree-line">└─</span
                                                >
                                            {/if}
                                            {process.name}
                                        </div>
                                    </td>
                                    <td class="cpu"
                                        >{process.cpu_usage.toFixed(1)}%</td
                                    >
                                    <td class="memory"
                                        >{formatBytes(process.memory_bytes)}</td
                                    >
                                    <td>
                                        <span
                                            class="status-badge status-{process.normalized_status.toLowerCase()}"
                                        >
                                            {process.normalized_status}
                                        </span>
                                    </td>
                                    <td class="user"
                                        >{process.normalized_user}</td
                                    >
                                    <td class="actions">
                                        <button
                                            class="action-btn terminate"
                                            onclick={() =>
                                                killProcess(process.pid, false)}
                                            title="Terminate (SIGTERM)"
                                        >
                                            Term
                                        </button>
                                        <button
                                            class="action-btn kill"
                                            onclick={() =>
                                                killProcess(process.pid, true)}
                                            title="Kill (SIGKILL)"
                                        >
                                            Kill
                                        </button>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
                <div class="table-footer">
                    {processes.length} processes (Displaying {Math.min(
                        viewMode === "tree" ? processes.length : 100,
                        processes.length,
                    )})
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
        display: flex;
        justify-content: space-between;
        align-items: center;
        position: sticky;
        top: 0;
        z-index: 10;
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

    .header-controls {
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    .view-toggle {
        display: flex;
        background: var(--md-sys-color-surface-variant);
        padding: 0.25rem;
        border-radius: 8px;
        gap: 0.25rem;
    }

    .toggle-btn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 6px;
        background: transparent;
        color: var(--md-sys-color-on-surface-variant);
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .toggle-btn:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--md-sys-color-on-surface);
    }

    .toggle-btn.active {
        background: var(--md-sys-color-surface);
        color: var(--md-sys-color-primary);
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
    }

    .search-box {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        background: var(--md-sys-color-surface-variant);
        border-radius: 8px;
        color: var(--md-sys-color-on-surface-variant);
    }

    .search-box input {
        border: none;
        background: none;
        outline: none;
        color: var(--md-sys-color-on-surface);
        font-size: 0.875rem;
        width: 200px;
    }

    .page-content {
        padding: 2rem;
    }

    .card {
        background: var(--md-sys-color-surface);
        border-radius: 16px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        display: flex;
        flex-direction: column;
        height: calc(
            100vh - 200px
        ); /* Fixed height for virtual scrolling feel */
    }

    .table-container {
        flex: 1;
        overflow: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        min-width: 800px;
    }

    thead {
        background: var(--md-sys-color-surface-variant);
        position: sticky;
        top: 0;
        z-index: 5;
    }

    th {
        padding: 1rem;
        text-align: left;
        font-weight: 600;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface);
        cursor: pointer;
        user-select: none;
        white-space: nowrap;
    }

    th:hover {
        background: var(--md-sys-color-outline);
    }

    td {
        padding: 0.5rem 1rem;
        border-bottom: 1px solid var(--md-sys-color-outline);
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface);
        white-space: nowrap;
    }

    tr:hover td {
        background: var(--md-sys-color-surface-variant);
    }

    .pid {
        font-family: "JetBrains Mono", monospace;
        font-weight: 600;
        color: var(--md-sys-color-secondary);
    }

    .name-cell {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .tree-line {
        color: var(--md-sys-color-outline);
        font-family: monospace;
        opacity: 0.5;
    }

    .name {
        font-weight: 500;
        max-width: 300px;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .cpu,
    .memory {
        font-family: "JetBrains Mono", monospace;
    }

    .user {
        color: var(--md-sys-color-on-surface-variant);
        max-width: 150px;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .status-badge {
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
    }

    .status-running {
        background: rgba(166, 227, 161, 0.2);
        color: #a6e3a1;
    }

    .status-sleeping {
        background: rgba(137, 180, 250, 0.2);
        color: #89b4fa;
    }

    .status-idle,
    .status-zombie {
        background: rgba(249, 226, 175, 0.2);
        color: #f9e2af;
    }

    .actions {
        display: flex;
        gap: 0.5rem;
    }

    .action-btn {
        padding: 0.25rem 0.5rem;
        border: none;
        border-radius: 4px;
        font-size: 0.75rem;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.2s ease;
    }

    .action-btn:hover {
        opacity: 0.8;
    }

    .action-btn.terminate {
        background: #f9e2af;
        color: #1e1e2e;
    }

    .action-btn.kill {
        background: #f38ba8;
        color: white;
    }

    .table-footer {
        padding: 0.75rem;
        text-align: center;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
        background: var(--md-sys-color-surface-variant);
        border-top: 1px solid var(--md-sys-color-outline);
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

    .clickable {
        cursor: pointer;
    }
</style>
