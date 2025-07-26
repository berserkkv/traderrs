<script lang="ts">
    import System from "$lib/component/System.svelte";

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

    export let data2 = [12, 1, 3, 10];
    const max = Math.max(...data2);

    export function parseIsoToDate(isoString: string): string {
        const date = new Date(isoString);
        const now = new Date();

        const isToday =
            date.getFullYear() === now.getFullYear() &&
            date.getMonth() === now.getMonth() &&
            date.getDate() == now.getDate();

        const pad = (n: number) => String(n).padStart(2, "0");

        if (isToday) {
            return `${pad(date.getHours())}:${pad(date.getMinutes())}`;
        } else {
            const shortYear = String(date.getFullYear()).slice(-2);
            return `${pad(date.getHours())}:${pad(date.getMinutes())} ${pad(date.getDate())}.${pad(date.getMonth())}`;
        }
    }
</script>

<System/>
<div class="pt-2 max-w-3xl flex-col justify-between m-auto">
    {#each data.bots as b}
        <div
                class="{b.is_not_active
        ? 'text-neutral-600'
        : 'text-neutral-400'} rounded-xl px-2 mx-1 py-1  mb-2 text-neutral-300 bg-zinc-900  "
        >
            <div>
                <div class="flex justify-between items-start">
                    <div class=" text-sm">
                        <span class="">{b.name}</span>
                        <span class="ml-1 text-xs">
              <span class="text-indigo-700">{b.wins}</span>/<span
                                class="text-fuchsia-700">{b.losses}</span
                        >
            </span>
                        <div class="text-neutral-600 text-tight">{b.log}</div>
                    </div>
                    <!--                    <div>-->
                    <!--                        <svg viewBox="0 0 {data2.length} {max}" width="100" height="30">-->
                    <!--                            <polyline-->
                    <!--                                    fill="none"-->
                    <!--                                    stroke="rgba(150, 100, 100, 1.0)"-->
                    <!--                                    stroke-width="0.5"-->
                    <!--                                    points="{data2.map((d, i) => `${i},${max - d}`).join(' ')}"-->
                    <!--                            />-->
                    <!--                        </svg>-->
                    <!--                    </div>-->
                    <div class="text-right text-sm">
            <span>
              {(b.capital + b.order_capital).toFixed(2)}
            </span>
                        <div class=" text-neutral-600 text-right text-tight">
                            {parseIsoToDate(b.last_scanned)}
                        </div>
                    </div>
                </div>
            </div>

            {#if b.in_pos}
                <div
                        class="border-t-2 my-1 {b.order_type === 'Long'
            ? 'border-t-indigo-950'
            : 'border-t-fuchsia-950'}
                     "
                ></div>

                <div class="flex justify-between text-sm">
          <span>
            <span
                    class=" {b.is_trailing_stop_active
                ? 'text-neutral-400'
                : 'text-neutral-700'}">TS</span
            >
            <span class="">{parseIsoToDate(b.order_created_at)}</span>
          </span>

                    <span class="">{b.order_entry_price.toFixed(2)}</span>
                    <span>
            <span class="bg-fuchsia-950 rounded-lg px-1"
            >{b.order_stop_loss.toFixed(2)}</span
            >
            <span class="bg-indigo-950 rounded-lg px-1"
            >{b.order_take_profit.toFixed(2)}</span
            >
          </span>

                    <span class="text-neutral-500">
            <span
                    class={b.pnl > 0
                ? "text-indigo-800"
                : b.pnl < 0
                  ? "text-fuchsia-800"
                  : "text-neutral-500"}
            >
              {b.pnl.toFixed(1)}
            </span>(
            <span
                    class={b.roe > 0
                ? "text-indigo-800"
                : b.roe < 0
                  ? "text-fuchsia-800"
                  : "text-neutral-500"}
            >
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
