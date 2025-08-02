<script lang="ts">
    import {onMount} from "svelte";
    import {writable} from "svelte/store";
    import {API_BASE} from "$lib/config";
    import {bgUpOrDown, parseIsoToDate, textUpOrDown} from "$lib/tools.js";
    import Chart from "$lib/component/Chart.svelte";

    export let id;

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

    const data = writable<Order[] | null>(null);
    const loading = writable(true);
    const error = writable<string | null>(null);

    onMount(async () => {
        try {
            const res = await fetch(API_BASE + "/api/v1/bots/" + id + "/orders");
            if (!res.ok) throw new Error(`failed to load system info: ${res.status}`);

            const json = await res.json() as unknown;

            if (typeof json === 'object' &&
                json != null) {
                data.set(json as Order[])
            } else {
                throw new Error("invalid api response format")
            }
        } catch (e) {
            error.set((e as Error).message);
        } finally {
            loading.set(false);
        }
    });


</script>
<div class="text-sm m-auto flex justify-center text-neutral-300">
    {#if $loading}
        <p>Loading...</p>
    {:else if $error}
        <p>Error: {$error}</p>
    {:else if $data && $data.length !== 0}
        <div>
            <div class="my-2">
                <Chart orders={$data}/>
            </div>
            <div>
                <table class="my-table">
                    <thead>
                    <tr class="text-neutral-500  text-xs bg-neutral-900">
                        <th class=""></th>

                        <th class="my-cell">Entry Price</th>
                        <th class="my-cell">Exit Price</th>
                        <th class="my-cell">Pnl (Roe)</th>
                        <th class="my-cell">Created time</th>
                        <th class="my-cell">Closed time</th>


                    </tr>
                    </thead>
                    <tbody>
                    {#each $data.slice().reverse() as o}
                        <tr>
                            <td class="border-none p-1 {bgUpOrDown(o.order_type === 'Long' ? 1 : -1)}"></td>
                            <td class="my-cell">{o.entry_price.toFixed(2)}</td>
                            <td class="my-cell">{o.exit_price.toFixed(2)}</td>
                            <td class="my-cell {textUpOrDown(o.pnl)}">{o.pnl.toFixed(2)} ({o.roe.toFixed(2)}%)</td>
                            <td class="my-cell">{parseIsoToDate(o.created_at)}</td>
                            <td class="my-cell">{parseIsoToDate(o.closed_at)}</td>
                        </tr>
                    {/each}
                    </tbody>
                </table>
            </div>
        </div>
    {:else}
        <p>No orders yet.</p>

    {/if}
</div>

<style>

</style>

