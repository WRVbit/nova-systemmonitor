// Nova System Monitor - Optimized Settings Store (Svelte 5 Runes)

export type Theme = 'light' | 'dark' | 'mocha';

// Browser environment check
const isBrowser = typeof window !== 'undefined';

export const settings = $state({
    theme: 'light' as Theme,
    refreshInterval: 3000, // Increased to 3s for better performance
    autoRefresh: false, // Disabled by default - manual refresh or per-view refresh
});

export function setTheme(theme: Theme) {
    settings.theme = theme;
    if (isBrowser) {
        localStorage.setItem('nova-theme', theme);
    }
}

export function setRefreshInterval(interval: number) {
    settings.refreshInterval = interval;
    if (isBrowser) {
        localStorage.setItem('nova-refresh-interval', interval.toString());
    }
}

export function toggleAutoRefresh() {
    settings.autoRefresh = !settings.autoRefresh;
    if (isBrowser) {
        localStorage.setItem('nova-auto-refresh', settings.autoRefresh.toString());
    }
}

// Load settings from localStorage on init
export function loadSettings() {
    if (!isBrowser) return;

    const savedTheme = localStorage.getItem('nova-theme') as Theme | null;
    if (savedTheme) settings.theme = savedTheme;

    const savedInterval = localStorage.getItem('nova-refresh-interval');
    if (savedInterval) settings.refreshInterval = parseInt(savedInterval);

    const savedAutoRefresh = localStorage.getItem('nova-auto-refresh');
    if (savedAutoRefresh) settings.autoRefresh = savedAutoRefresh === 'true';
}
