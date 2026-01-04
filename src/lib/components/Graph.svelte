<script lang="ts">
    import { onMount } from "svelte";
    import uPlot from "uplot";
    import "uplot/dist/uPlot.min.css";

    interface Props {
        title?: string;
        data: [number[], ...number[][]]; // First array is x-axis (timestamps), others are y-series
        series: {
            label: string;
            stroke: string;
            fill?: string;
            width?: number;
        }[];
        height?: number;
        xLabel?: string;
        yLabel?: string;
        yMin?: number;
        yMax?: number;
    }

    let {
        title = "",
        data,
        series,
        height = 200,
        xLabel = "Time",
        yLabel = "Value",
        yMin = 0,
        yMax,
    }: Props = $props();

    let chartContainer: HTMLDivElement | undefined;
    let uplotInst: uPlot | undefined;
    let resizeObserver: ResizeObserver | undefined;

    function initChart() {
        if (!chartContainer) return;

        // Default Series (X-axis)
        const seriesOpts: uPlot.Series[] = [
            {
                label: xLabel,
                value: (_: uPlot, v: number) => {
                    if (!v) return "--:--:--";
                    const d = new Date(v * 1000);
                    return d.toLocaleTimeString([], { hour12: false });
                },
            },
            ...series.map((s) => ({
                label: s.label,
                stroke: s.stroke,
                fill: s.fill,
                width: s.width ?? 2,
                points: { show: false }, // Hide points for performance
            })),
        ];

        const opts: uPlot.Options = {
            title: title,
            width: chartContainer.clientWidth,
            height: height,
            series: seriesOpts,
            axes: [
                {
                    label: xLabel,
                    stroke: "#a1a1aa", // axis color
                    grid: { stroke: "#3f3f4655" },
                    ticks: { stroke: "#3f3f4655" },
                    space: 50,
                },
                {
                    label: yLabel,
                    stroke: "#a1a1aa", // axis color
                    grid: { stroke: "#3f3f4655" },
                    ticks: { stroke: "#3f3f4655" },
                    size: 60,
                    values: (_: uPlot, vals: number[]) =>
                        vals.map((v) => v.toFixed(1)),
                },
            ],
            scales: {
                x: {
                    time: true,
                },
                y: {
                    auto: yMax === undefined,
                    range: yMax !== undefined ? [yMin, yMax] : undefined,
                },
            },
            cursor: {
                drag: { x: false, y: false },
                points: {
                    size: 6,
                    fill: (self: uPlot, i: number) =>
                        (self.series[i]?.stroke as string) || "transparent",
                },
            },
            legend: {
                show: true,
                // Custom legend behavior or styling if needed
            },
        };

        uplotInst = new uPlot(opts, data, chartContainer);

        // Handle Resize
        resizeObserver = new ResizeObserver((entries) => {
            if (!uplotInst) return;
            for (const entry of entries) {
                uplotInst.setSize({
                    width: entry.contentRect.width,
                    height: height,
                });
            }
        });
        resizeObserver.observe(chartContainer);
    }

    // Effect to update data when it changes
    $effect(() => {
        if (uplotInst && data) {
            uplotInst.setData(data);
        }
    });

    // Re-init if series config changes drastically (optional, simpler to just re-mount if needed)

    onMount(() => {
        initChart();
        return () => {
            if (resizeObserver) resizeObserver.disconnect();
            if (uplotInst) uplotInst.destroy();
        };
    });
</script>

<div class="graph-wrapper" bind:this={chartContainer}></div>

<style>
    .graph-wrapper {
        width: 100%;
        position: relative;
        background: var(--md-sys-color-surface);
        border-radius: 12px;
        padding: 1rem;
        color: var(--md-sys-color-on-surface);
    }

    /* uPlot Dark Mode Overrides for better integration */
    :global(.uplot) {
        font-family: "Inter", sans-serif !important;
    }

    :global(.uplot .legend) {
        color: var(--md-sys-color-on-surface) !important;
    }

    :global(.uplot .title) {
        color: var(--md-sys-color-on-surface) !important;
        font-weight: 600;
    }
</style>
