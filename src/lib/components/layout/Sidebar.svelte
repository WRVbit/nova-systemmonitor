<!-- Nova System Monitor - Sidebar Navigation -->
<script lang="ts">
    import { page } from "$app/stores";
    import {
        Cpu,
        MemoryStick,
        HardDrive,
        Network,
        Activity,
        Thermometer,
        Info,
        List,
        LayoutDashboard,
        Settings,
    } from "lucide-svelte";

    const navItems = [
        { href: "/", icon: LayoutDashboard, label: "Overview" },
        { href: "/cpu", icon: Cpu, label: "CPU" },
        { href: "/memory", icon: MemoryStick, label: "Memory" },
        { href: "/disk", icon: HardDrive, label: "Disk" },
        { href: "/network", icon: Network, label: "Network" },
        { href: "/gpu", icon: Activity, label: "GPU" },
        { href: "/processes", icon: List, label: "Processes" },
        { href: "/system", icon: Info, label: "System" },
        { href: "/settings", icon: Settings, label: "Settings" },
    ];

    // Check if current path matches the href (with proper root path handling)
    function isActivePath(href: string, pathname: string): boolean {
        if (href === "/") {
            return pathname === "/";
        }
        return pathname.startsWith(href);
    }
</script>

<aside class="sidebar">
    <div class="sidebar-header">
        <h2>Nova</h2>
        <p>System Monitor</p>
    </div>

    <nav class="sidebar-nav">
        {#each navItems as item}
            {@const isActive = isActivePath(item.href, $page.url.pathname)}
            <a href={item.href} class="nav-item" class:active={isActive}>
                <svelte:component this={item.icon} size={20} />
                <span>{item.label}</span>
            </a>
        {/each}
    </nav>
</aside>

<style>
    .sidebar {
        width: 240px;
        height: 100vh;
        background: var(--md-sys-color-surface);
        border-right: 1px solid var(--md-sys-color-outline);
        display: flex;
        flex-direction: column;
        position: fixed;
        left: 0;
        top: 0;
    }

    .sidebar-header {
        padding: 2rem 1.5rem;
        border-bottom: 1px solid var(--md-sys-color-outline);
    }

    .sidebar-header h2 {
        margin: 0;
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--md-sys-color-primary);
    }

    .sidebar-header p {
        margin: 0.25rem 0 0 0;
        font-size: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
    }

    .sidebar-nav {
        flex: 1;
        padding: 1rem 0;
        overflow-y: auto;
    }

    .nav-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1.5rem;
        color: var(--md-sys-color-on-surface);
        text-decoration: none;
        transition: all 0.2s ease;
        border-left: 3px solid transparent;
    }

    .nav-item:hover {
        background: var(--md-sys-color-surface-variant);
    }

    .nav-item.active {
        background: var(--md-sys-color-surface-variant);
        border-left-color: var(--md-sys-color-primary);
        color: var(--md-sys-color-primary);
        font-weight: 600;
    }

    .nav-item span {
        font-size: 0.875rem;
    }
</style>
