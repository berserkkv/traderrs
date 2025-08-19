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
  const padding = 10; // 👈 space at top/bottom

  let capital = 0;
  const pnls = orders.map(o => {
    capital += o.pnl;
    return capital;
  });

  $: minPnl = Math.min(...pnls);
  $: maxPnl = Math.max(...pnls);

  $: points = pnls.map((value, i) => {
    const x = (i / (pnls.length - 1)) * width;

    const range = maxPnl - minPnl || 1;

    // scale to [padding, height - padding]
    const y = height - padding - ((value - minPnl) / range) * (height - 2 * padding);

    return `${x},${y}`;
  }).join(" ");
</script>

<div class="px-2 pt-2 bg-gray-900">
  <svg height={height} preserveAspectRatio="none" viewBox={`0 0 ${width} ${height}`} width="100%">
    <polyline
      fill="none"
      points={points}
      stroke="rgba(200, 200, 200, 1.0)"
      stroke-width="1"
    />
  </svg>
</div>
