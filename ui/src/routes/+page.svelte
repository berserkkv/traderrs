<script lang="ts">
  import System from "$lib/component/System.svelte";
  import {bgUpOrDown, calculateWinPercentage, textUpOrDown} from "$lib/tools.js";
  import * as tools from "$lib/tools";

  export let data: {
    bots: {
      id: number;
      name: string;
      in_pos: boolean;
      is_not_active: boolean;
      is_trailing_stop_active: boolean;
      last_scanned: string;
      leverage: number;
      wins: number;
      losses: number;
      capital: number;
      order_capital: number;
      log: string;
      order_capital_with_leverage: number;
      order_created_at: string;
      order_entry_price: number;
      order_scanned_at: string;
      order_fee: number;
      order_quantity: number;
      order_stop_loss: number;
      order_take_profit: number;
      order_type: string;
      pnl: number;
      roe: number;
      stop_loss_ratio: number;
      take_profit_ratio: number;
      strategy_name: string;
      symbol: string;
      timeframe: string;
      trailing_stop_activation_point: number;
    }[];
  };

  const totalCapital = data.bots.reduce(
    (sum, bot) => sum + bot.capital + bot.order_capital,
    0
  );
  const startCapital = data.bots.length * 100;
  const totalRoe = (totalCapital * 100) / startCapital - 100;

  
</script>

<System />

<div class="pt-2 max-w-3xl flex-col justify-between m-auto">
  <div class="text-xs mx-1 mb-2 px-2 card-bg rounded-lg text-neutral-500 flex">
    <p>
      Total: <span class=" text-sm font-semibold {textUpOrDown(totalRoe)}"
        >{totalCapital.toFixed(2)}
        ({totalRoe.toFixed(2)}%)</span
      >
    </p>
    <a class="ml-2 underline text-blue-600" href="/bots/statistics">Statistics</a>
  </div>
  {#each data.bots as b}
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
  {/each}
</div>

<style>
  .text-tight {
    font-size: 0.77rem; /* or text-xs */
    line-height: 0.6rem;
    margin: 0;
    padding: 0;
  }
</style>
