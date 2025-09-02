<script lang="ts">
  import System from "$lib/component/System.svelte";
  import Orders from '$lib/component/Orders.svelte'
  import BotStatistics from '$lib/component/BotStatistics.svelte'
  import {bgUpOrDown, calculateWinPercentage, textUpOrDown} from "$lib/tools.js";
  import * as tools from '$lib/tools'

  export let data;
  const b = data.bot;

</script>

<System/>
<div class="pt-2 max-w-3xl flex-col justify-between m-auto">
  <div
    class="{b.is_not_active
        ? 'text-neutral-600'
        : 'text-neutral-400'} rounded-xl border border-neutral-900 px-2 mx-1 py-1 mb-2 text-neutral-300 card-bg"
  >
    <div>
      <div class="flex justify-between items-start">
        <div class=" text-sm">
          <span class=""><a href="/bots/{b.name}">{b.name}</a></span>
          <span class="ml-1 text-xs font-semibold">
              <span class={textUpOrDown(1)}>{b.wins}</span>/<span class={textUpOrDown(-1)}>{b.losses}</span>
              <span>({calculateWinPercentage(b.wins, b.wins + b.losses).toFixed(1)}%)</span>
            </span>
          <div class="text-neutral-600 text-tight">{b.log}</div>
        </div>
        <div class="text-right text-sm">
            <span>
              {(b.capital + b.order_capital).toFixed(2)}
            </span>
          <div class=" text-neutral-600 text-right text-tight">
            {tools.parseIsoToDate(b.last_scanned)}
          </div>
        </div>
      </div>
    </div>

    {#if b.in_pos}
      <div class="border-t-1 my-1 border-neutral-800"></div>

      <div class="flex justify-between text-xs">
          <span>
            <span
              class=" px-1 rounded-md {b.order_type === 'Long'
                ? bgUpOrDown(1)
                : bgUpOrDown(-1)}"
            >
              {b.order_type === "Long" ? "L" : "S"}
            </span>
            <span
              class=" {b.is_trailing_stop_active
                ? 'text-neutral-400'
                : 'text-neutral-700'}">TS</span
            >
            <span class="">{tools.parseIsoToDate(b.order_created_at)}</span>
          </span>

        <span class="">{b.order_entry_price.toFixed(2)}</span>
        <span>
            <span class="{bgUpOrDown(-1)} rounded-lg px-1"
            >{b.order_stop_loss.toFixed(2)}</span
            >
            <span class="{bgUpOrDown(1)} rounded-lg px-1"
            >{b.order_take_profit.toFixed(2)}</span
            >
          </span>
        <span class="text-neutral-500">
            <span class={textUpOrDown(b.pnl)}>
              {b.pnl.toFixed(1)}
            </span>(
            <span class={textUpOrDown(b.roe)}>
              {b.roe.toFixed(1)}%
            </span>)
          </span>
      </div>
    {/if}
  </div>
</div>

<BotStatistics statistic={data.statistic}/>
<Orders orders={data.orders}/>
