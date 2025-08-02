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

    // const max = Math.max(...orders[0].pnl);

</script>

<div>
    <svg viewBox="0 0 {orders.length} {2}" width="100" height="30">
        <polyline
                fill="none"
                stroke="rgba(150, 100, 100, 1.0)"
                stroke-width="0.5"
                points="{orders.map((d, i) => `${i},${2 - d.pnl}`).join(' ')}"
        />
    </svg>
</div>