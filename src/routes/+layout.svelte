<!-- Nova System Monitor - Root Layout -->
<script lang="ts">
    import { onMount } from "svelte";
    import type { Snippet } from "svelte";
    import "../app.css";
    import Sidebar from "$lib/components/layout/Sidebar.svelte";
    import { settings, loadSettings } from "$lib/stores/settings.svelte";

    // Accept children prop from SvelteKit
    let { children }: { children: Snippet } = $props();

    // Load saved settings on mount
    onMount(() => {
        loadSettings();
    });

    // Apply theme via CSS variables
    $effect(() => {
        // Ensure we're in browser context
        if (typeof document === 'undefined') return;
        
        const root = document.documentElement;
        const themes = {
            light: {
                "--md-sys-color-primary": "#0066cc",
                "--md-sys-color-on-primary": "#ffffff",
                "--md-sys-color-surface": "#fafafa",
                "--md-sys-color-on-surface": "#1a1c1e",
                "--md-sys-color-background": "#fafafa",
                "--md-sys-color-on-background": "#1a1c1e",
                "--md-sys-color-surface-variant": "#e8eaed",
                "--md-sys-color-on-surface-variant": "#444746",
                "--md-sys-color-outline": "#ccc",
            },
            dark: {
                "--md-sys-color-primary": "#89b4fa",
                "--md-sys-color-on-primary": "#003258",
                "--md-sys-color-surface": "#1e1e2e",
                "--md-sys-color-on-surface": "#cdd6f4",
                "--md-sys-color-background": "#181825",
                "--md-sys-color-on-background": "#cdd6f4",
                "--md-sys-color-surface-variant": "#313244",
                "--md-sys-color-on-surface-variant": "#a6adc8",
                "--md-sys-color-outline": "#45475a",
            },
            mocha: {
                "--md-sys-color-primary": "#89b4fa",
                "--md-sys-color-on-primary": "#1e1e2e",
                "--md-sys-color-surface": "#313244",
                "--md-sys-color-on-surface": "#cdd6f4",
                "--md-sys-color-background": "#1e1e2e",
                "--md-sys-color-on-background": "#cdd6f4",
                "--md-sys-color-surface-variant": "#45475a",
                "--md-sys-color-on-surface-variant": "#a6adc8",
                "--md-sys-color-outline": "#585b70",
            },
            cappuccino: {
                "--md-sys-color-primary": "#1e66f5",
                "--md-sys-color-on-primary": "#ffffff",
                "--md-sys-color-surface": "#e6e9ef",
                "--md-sys-color-on-surface": "#4c4f69",
                "--md-sys-color-background": "#eff1f5",
                "--md-sys-color-on-background": "#4c4f69",
                "--md-sys-color-surface-variant": "#dce0e8",
                "--md-sys-color-on-surface-variant": "#5c5f77",
                "--md-sys-color-outline": "#9ca0b0",
            },
        };

        const theme = themes[settings.theme as keyof typeof themes];
        if (theme) {
            Object.entries(theme).forEach(([key, value]) => {
                root.style.setProperty(key, value);
            });
        }
    });
</script>

<div class="app">
    <Sidebar />
    <main class="main-content">
        {@render children()}
    </main>
</div>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family:
            "Inter",
            -apple-system,
            BlinkMacSystemFont,
            "Segoe UI",
            "Roboto",
            sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    .app {
        display: flex;
        min-height: 100vh;
        background: var(--md-sys-color-background);
        color: var(--md-sys-color-on-background);
    }

    .main-content {
        flex: 1;
        margin-left: 240px;
        min-height: 100vh;
    }
</style>
