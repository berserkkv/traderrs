<script lang="ts">
    import {onMount} from "svelte";

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

    // let container: HTMLDivElement;
    //
    // onMount(() => {
    //     width = container.clientWidth;
    // });

    let capital = 0;
    // Get min and max PnL to normalize values
    const pnls = orders.map(o => {
        capital += o.pnl;
        return capital;
    });

    $: maxPnl = Math.max(...pnls);

    $: points = pnls.map((value, i) => {
        const x = (i / (pnls.length - 1) * width);
        const y = height - (value / maxPnl) * height;
        return `${x}, ${y}`
    }).join(' ');


</script>

<div class="px-2 pt-2 bg-gray-900">
    <svg viewBox={`0 0 ${width} ${height}`} width="100%" height={height} preserveAspectRatio="none">
        <polyline
                fill="none"
                stroke="rgba(200, 200, 200, 1.0)"
                stroke-width="1"
                points={points}
        />
    </svg>
</div>
