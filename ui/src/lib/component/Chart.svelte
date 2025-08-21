<script lang="ts">
  import {onMount} from 'svelte'
  import {createChart, type ISeriesApi, LineSeries, type UTCTimestamp} from 'lightweight-charts'

  type Order = {
    bot_id: number;
    symbol: string;
    entry_price: number;
    exit_price: number;
    fee: number;
    quantity: number;
    pnl: number;
    roe: number;
    order_type: string;
    leverage: number;
    created_at: string;
    closed_at: string;
  }

  export let orders: Order[];


  let chartContainer: HTMLDivElement;
  onMount(() => {

    const pnls = orders.map((o) => ({
      value: o.pnl,
      time: Math.floor(new Date(o.created_at).getTime() / 1000) as UTCTimestamp,
    }));

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
