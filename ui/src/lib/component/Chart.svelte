<script lang="ts">
  import {onMount} from 'svelte'
  import {createChart, type ISeriesApi, LineSeries, type UTCTimestamp} from 'lightweight-charts'
  import type {Order} from '$lib/types'

  export let orders: Order[];

  let chartContainer: HTMLDivElement;
  onMount(() => {
    let total = 0;

    const pnls = orders.map((o) => ({

      value: total += o.pnl,
      time: Math.floor(new Date(o.created_at).getTime() / 1000) as UTCTimestamp,
    }));

    pnls.sort((a, b) => new Date(a.time).getTime() - new Date(b.time).getTime());

    const chart = createChart(chartContainer, {
      width: chartContainer.clientWidth,
      height: 200,
      layout: {
        // background: {color: '#111827'},
        background: {color: '#101010'},
        textColor: '#9ca3af',
        attributionLogo: false,
      },
      grid: {
        vertLines: {color: '#1f2937'}, // Tailwind gray-800
        horzLines: {color: '#1f2937'}, // Tailwind gray-800
      },
      timeScale: {
        timeVisible: true,
        secondsVisible: false,
        borderColor: '#374151', // Tailwind gray-700
      },
      crosshair: {
        vertLine: {
          color: '#4b5563', // Tailwind gray-600
          width: 1,
          style: 0,
        },
        horzLine: {
          color: '#4b5563',
          width: 1,
          style: 0,
        },
      },
    });

    const lineSeries: ISeriesApi<"Line">= chart.addSeries(LineSeries, {
      color: '#2196F3',
      lineWidth: 1,
      crosshairMarkerVisible: true,
      crosshairMarkerRadius: 2,
    });


    lineSeries.setData(pnls);

    chart.timeScale().fitContent();
    lineSeries.createPriceLine({
      price: 0,
      color: 'rgba(89,89,89,0.4)',
      lineWidth: 1,
      lineStyle: 0,
      axisLabelVisible: false,
    });

    const resizeObserver = new ResizeObserver(() => {
      chart.applyOptions({
        width: chartContainer.clientWidth,
        timeScale: {
          rightOffset: 0,
          fixLeftEdge: true,
          fixRightEdge: true,
          barSpacing:1,
        },
        rightPriceScale: {
          scaleMargins: {
            top: 0.05,
            bottom: 0.05,
          }
        }
      });
    });
    resizeObserver.observe(chartContainer);

    return () => {
      resizeObserver.disconnect();
      chart.remove();
    };
  });
</script>


<div bind:this={chartContainer} class="w-full h-[200px]">

</div>
