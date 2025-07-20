<script lang="ts">
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

<div class="pt-1 max-w-3xl flex-col  justify-between m-auto">
    {#each data.bots as bot}
        <div class="{bot.is_not_active ? 'text-neutral-600' : ''} border rounded-xl px-2 mx-1 py-1 border-neutral-600 mb-1 text-neutral-300">
            <a href={`/bot/${bot.id}`}>
                <div class="flex justify-between">
                  <span class=" text-sm">
                      <span class="">{bot.name}</span>
                      <span class="ml-1 text-xs">
                          <span class="text-indigo-700">{bot.wins}</span>/<span
                              class="text-fuchsia-700">{bot.losses}</span>
                      </span>
                  </span>
                    <span class="text-right text-sm text-gray-300">
                    {(bot.capital + bot.order_capital).toFixed(1)}
                  </span>
                </div>
                <div class="text-xs  text-neutral-400 flex justify-between">
                    <span>{bot.log}</span>
                    <span class="text-right">
            {parseIsoToDate(bot.last_scanned)}
          </span>
                </div>
            </a>

            {#if bot.in_pos}
                <div class="border-t-2  my-1 {bot.order_type === 'Long' ? 'border-t-indigo-950' : 'border-t-fuchsia-950'}
                     "></div>

                <div class="flex justify-between text-sm">
                    <span class="text-xs">{parseIsoToDate(bot.order_created_at)}</span>
                    <span><span
                            class="text-xs {bot.is_trailing_stop_active ? 'text-neutral-300' : 'text-neutral-700'}">TS</span></span>
                    <span class="bg-fuchsia-950 rounded-lg px-1">{bot.order_stop_loss.toFixed(2)}</span>
                    <span class="bg-indigo-950 rounded-lg px-1">{bot.order_take_profit.toFixed(2)}</span>

                    <span class="{bot.pnl > 0 ? 'text-indigo-800' : bot.pnl < 0 ? 'text-fuchsia-800' : 'text-neutral-500'}">{bot.pnl.toFixed(1)}
                        <span class="text-xs">({bot.roe.toFixed(1)}%)</span></span>
                </div>
            {/if}
        </div>
    {/each}
</div>
