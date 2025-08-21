<script lang="ts">
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

  let width = 300;
  const height = 100;
  const padding = 10;

  let capital = 0;
  const pnls = orders.map(o => {
    capital += o.pnl;
    return capital;
  });

  $: minPnl = Math.min(...pnls);
  $: maxPnl = Math.max(...pnls);
  $: range = maxPnl - minPnl || 1;

  // 👇 map 0 into chart coordinates
  $: zeroLine = height - padding - ((0 - minPnl) / range) * (height - 2 * padding);

  $: points = pnls.map((value, i) => {
    const x = (i / (pnls.length - 1)) * width;
    const y = height - padding - ((value - minPnl) / range) * (height - 2 * padding);
    return `${x},${y}`;
  }).join(" ");

  const tickCount = 3;

  $: ticks = Array.from({ length: tickCount + 1 }, (_, i) => {
    const value = minPnl + (i / tickCount) * (maxPnl - minPnl);
    const y = height - padding - ((value - minPnl) / range) * (height - 2 * padding);
    return { value, y };
  });
</script>

<div class="px-2 pt-2 bg-gray-900">
  <svg height={height} preserveAspectRatio="none" viewBox={`0 0 ${width} ${height}`} width="100%">
    <!-- grid lines + labels -->
    {#each ticks as t}
      <line
        x1="0"
        x2={width}
        y1={t.y}
        y2={t.y}
        stroke="gray"
        stroke-dasharray="2 2"
        stroke-width="0.5"
        opacity="0.3"
      />
      <text
        x="2"
        y={t.y - 2}
        fill="gray"
        opacity="0.9"
        font-size="10"
        alignment-baseline="middle"
      >
        {t.value.toFixed(2)}
      </text>
    {/each}

    <polyline
      fill="none"
      points={points}
      stroke="rgba(200, 200, 200, 1.0)"
      stroke-width="1"
    />

    <line
      opacity="0.5"
      stroke="lightcoral"
      stroke-dasharray="3 4"
      stroke-linecap="round"
      stroke-width="1"
      vector-effect="non-scaling-stroke"
      x1="0"
      x2={width}
      y1={zeroLine}
      y2={zeroLine}
    />
    <rect  fill="lightcoral"
           height='{height - zeroLine - padding}'
           opacity="0.1"
           width='{width}'
           x="0"
           y={zeroLine}
    />
  </svg>
</div>
