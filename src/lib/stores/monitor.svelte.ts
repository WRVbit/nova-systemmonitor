// Nova System Monitor - Monitor Data Store (Svelte 5 Runes)
import { invoke } from '@tauri-apps/api/core';

const MAX_HISTORY = 60;

// Track if a fetch is in progress to prevent concurrent calls
let fetchLocks = new Map<string, boolean>();

// Helper to update history buffers with bounds checking
function pushHistory(buffer: number[], value: number) {
    // Ensure value is a valid number
    if (!Number.isFinite(value)) value = 0;
    buffer.push(value);
    if (buffer.length > MAX_HISTORY) buffer.shift();
}

function pushTimestamp(buffer: number[]) {
    buffer.push(Date.now() / 1000);
    if (buffer.length > MAX_HISTORY) buffer.shift();
}

// CPU State
export const cpuInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

export const cpuHistory = $state({
    timestamps: [] as number[],
    global: [] as number[],
    cores: new Map<number, number[]>(), // Map of core index -> history array
});

// Memory State  
export const memoryInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

export const memoryHistory = $state({
    timestamps: [] as number[],
    ram_used: [] as number[],
    swap_used: [] as number[],
});

// GPU State
export const gpuInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

// Disk State
export const diskInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

export const diskHistory = $state({
    timestamps: [] as number[],
    read: [] as number[],
    write: [] as number[],
});

// Network State
export const networkInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

export const networkHistory = $state({
    timestamps: [] as number[],
    download: [] as number[],
    upload: [] as number[],
});

// Process State
export const processInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

// Sensors State
export const sensorsInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

// System Info State
export const systemInfo = $state({
    data: null as any,
    loading: false,
    error: null as string | null,
});

// Fetcher functions with concurrent call protection
export async function fetchCpuInfo() {
    if (fetchLocks.get('cpu')) return;
    fetchLocks.set('cpu', true);

    cpuInfo.loading = true;
    cpuInfo.error = null;
    try {
        const data = await invoke('get_cpu_info') as any;
        cpuInfo.data = data;

        // Update History
        pushTimestamp(cpuHistory.timestamps);
        pushHistory(cpuHistory.global, data.global_usage ?? 0);

        // Initialize core history arrays if needed
        if (Array.isArray(data.cores)) {
            data.cores.forEach((core: any, index: number) => {
                if (!cpuHistory.cores.has(index)) {
                    cpuHistory.cores.set(index, []);
                }
                const coreHist = cpuHistory.cores.get(index)!;
                pushHistory(coreHist, core.usage ?? 0);
            });
        }

    } catch (e: any) {
        cpuInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        cpuInfo.loading = false;
        fetchLocks.set('cpu', false);
    }
}

export async function fetchMemoryInfo() {
    if (fetchLocks.get('memory')) return;
    fetchLocks.set('memory', true);

    memoryInfo.loading = true;
    memoryInfo.error = null;
    try {
        const data = await invoke('get_memory_info') as any;
        memoryInfo.data = data;

        // Update History with safe division
        pushTimestamp(memoryHistory.timestamps);
        const ramPercent = data.total_memory > 0 ? (data.used_memory / data.total_memory) * 100 : 0;
        const swapPercent = data.total_swap > 0 ? (data.used_swap / data.total_swap) * 100 : 0;
        pushHistory(memoryHistory.ram_used, ramPercent);
        pushHistory(memoryHistory.swap_used, swapPercent);

    } catch (e: any) {
        memoryInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        memoryInfo.loading = false;
        fetchLocks.set('memory', false);
    }
}

export async function fetchGpuInfo() {
    if (fetchLocks.get('gpu')) return;
    fetchLocks.set('gpu', true);

    gpuInfo.loading = true;
    gpuInfo.error = null;
    try {
        gpuInfo.data = await invoke('get_gpu_info');
    } catch (e: any) {
        gpuInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        gpuInfo.loading = false;
        fetchLocks.set('gpu', false);
    }
}

export async function fetchDiskInfo() {
    if (fetchLocks.get('disk')) return;
    fetchLocks.set('disk', true);

    diskInfo.loading = true;
    diskInfo.error = null;
    try {
        const data = await invoke('get_disk_info') as any;
        diskInfo.data = data;

        // Calculate total I/O
        let totalRead = 0;
        let totalWrite = 0;
        if (data.disks && Array.isArray(data.disks)) {
            data.disks.forEach((disk: any) => {
                totalRead += disk.read_bytes || 0;
                totalWrite += disk.written_bytes || 0;
            });
        }

        // Update History
        pushTimestamp(diskHistory.timestamps);
        pushHistory(diskHistory.read, totalRead);
        pushHistory(diskHistory.write, totalWrite);

    } catch (e: any) {
        diskInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        diskInfo.loading = false;
        fetchLocks.set('disk', false);
    }
}

export async function fetchNetworkInfo() {
    if (fetchLocks.get('network')) return;
    fetchLocks.set('network', true);

    networkInfo.loading = true;
    networkInfo.error = null;
    try {
        const data = await invoke('get_network_info') as any;
        networkInfo.data = data;

        // Update History
        pushTimestamp(networkHistory.timestamps);
        pushHistory(networkHistory.download, data.total_download_rate ?? 0);
        pushHistory(networkHistory.upload, data.total_upload_rate ?? 0);

    } catch (e: any) {
        networkInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        networkInfo.loading = false;
        fetchLocks.set('network', false);
    }
}

export async function fetchProcessInfo() {
    if (fetchLocks.get('process')) return;
    fetchLocks.set('process', true);

    processInfo.loading = true;
    processInfo.error = null;
    try {
        processInfo.data = await invoke('get_processes');
    } catch (e: any) {
        processInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        processInfo.loading = false;
        fetchLocks.set('process', false);
    }
}

export async function fetchSensorsInfo() {
    if (fetchLocks.get('sensors')) return;
    fetchLocks.set('sensors', true);

    sensorsInfo.loading = true;
    sensorsInfo.error = null;
    try {
        sensorsInfo.data = await invoke('get_sensors_info');
    } catch (e: any) {
        sensorsInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        sensorsInfo.loading = false;
        fetchLocks.set('sensors', false);
    }
}

export async function fetchSystemInfo() {
    if (fetchLocks.get('system')) return;
    fetchLocks.set('system', true);

    systemInfo.loading = true;
    systemInfo.error = null;
    try {
        systemInfo.data = await invoke('get_system_info');
    } catch (e: any) {
        systemInfo.error = e?.message || e?.toString() || 'Unknown error';
    } finally {
        systemInfo.loading = false;
        fetchLocks.set('system', false);
    }
}

// Fetch all data with Promise.allSettled for graceful error handling
export async function fetchAllData() {
    await Promise.allSettled([
        fetchCpuInfo(),
        fetchMemoryInfo(),
        fetchGpuInfo(),
        fetchDiskInfo(),
        fetchNetworkInfo(),
        fetchProcessInfo(),
        fetchSensorsInfo(),
        fetchSystemInfo(),
    ]);
}
