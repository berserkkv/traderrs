<script lang="ts">
  import {bgUpOrDown, parseIsoToDate, textUpOrDown} from "$lib/tools.js";
  import Chart from '$lib/component/Chart.svelte'
  import type {Order} from '$lib/types'

  export let orders: Order[];

</script>
<div class="text-sm m-auto flex justify-center text-neutral-300">
    {#if orders && orders.length !== 0}
        <div>
            <div class="my-2">
                <Chart orders={orders}/>
            </div>
            <div class="overflow-x-auto w-full">
                <table class="table">
                    <thead>
                    <tr class="text-neutral-500  text-xs bg-neutral-900">
                        <th class=""></th>

                        <th class="table-cell">Entry Price</th>
                        <th class="table-cell">Exit Price</th>
                        <th class="table-cell">Pnl (Roe)</th>
                        <th class="table-cell">Created time</th>
                        <th class="table-cell">Closed time</th>


                    </tr>
                    </thead>
                    <tbody>
                    {#each orders.slice().reverse() as o}
                        <tr>
                            <td class="border-none p-1 {bgUpOrDown(o.order_type === 'Long' ? 1 : -1)}"></td>
                            <td class="table-cell">{o.entry_price.toFixed(2)}</td>
                            <td class="table-cell">{o.exit_price.toFixed(2)}</td>
                            <td class="table-cell {textUpOrDown(o.pnl)}">{o.pnl.toFixed(2)} ({o.roe.toFixed(2)}%)</td>
                            <td class="table-cell">{parseIsoToDate(o.created_at)}</td>
                            <td class="table-cell">{parseIsoToDate(o.closed_at)}</td>
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

