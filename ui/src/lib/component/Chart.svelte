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

    // Calculate chart dimensions and scaling
    const width = 300;
    const height = 100;

    // Get min and max PnL to normalize values
    const pnls = orders.map(o => o.pnl);
    const minPnl = Math.min(...pnls);
    const maxPnl = Math.max(...pnls);
    const range = maxPnl - minPnl || 1;

    // Scale function to map PnL to chart Y coordinates
    const scaleY = (pnl: number) => height - ((pnl - minPnl) / range) * height;

    // Generate the points for the polyline
    const points = orders.map((order, i) => {
        const x = (i / (orders.length - 1)) * width;
        const y = scaleY(order.pnl);
        return `${x},${y}`;
    }).join(' ');
</script>


<div>
    <svg viewBox={`0 0 ${width} ${height}`} width={width} height={height}>
        <polyline
                fill="none"
                stroke="rgba(150, 100, 100, 1.0)"
                stroke-width="1"
                points={points}
        />
    </svg>
</div>
