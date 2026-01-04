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
        if (typeof document === "undefined") return;

        const root = document.documentElement;
        const themes = {
            light: {
                "--md-sys-color-primary": "#8D6E63",
                "--md-sys-color-on-primary": "#ffffff",
                "--md-sys-color-surface": "#FFFFFF",
                "--md-sys-color-on-surface": "#4E342E",
                "--md-sys-color-background": "#FAF7F5",
                "--md-sys-color-on-background": "#4E342E",
                "--md-sys-color-surface-variant": "#EBE6E3",
                "--md-sys-color-on-surface-variant": "#6D5D54",
                "--md-sys-color-outline": "#D7CCC8",
            },
            dark: {
                "--md-sys-color-primary": "#D0BCFF",
                "--md-sys-color-on-primary": "#381E72",
                "--md-sys-color-surface": "#201F23",
                "--md-sys-color-on-surface": "#E6E1E5",
                "--md-sys-color-background": "#0E0E0E",
                "--md-sys-color-on-background": "#E6E1E5",
                "--md-sys-color-surface-variant": "#2B292F",
                "--md-sys-color-on-surface-variant": "#CAC4D0",
                "--md-sys-color-outline": "#49454F",
            },
            mocha: {
                "--md-sys-color-primary": "#D7C0A0",
                "--md-sys-color-on-primary": "#3E2723",
                "--md-sys-color-surface": "#221F1D",
                "--md-sys-color-on-surface": "#EDE0D4",
                "--md-sys-color-background": "#1A1816",
                "--md-sys-color-on-background": "#EDE0D4",
                "--md-sys-color-surface-variant": "#2D2A27",
                "--md-sys-color-on-surface-variant": "#D7CACC",
                "--md-sys-color-outline": "#4E4540",
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
