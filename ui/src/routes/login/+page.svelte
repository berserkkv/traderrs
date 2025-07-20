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

        }[]
    };

    export function parseIsoToDate(isoString: string): string {
        const date = new Date(isoString);
        const now = new Date();

        const isToday = date.getFullYear() === now.getFullYear()
            && date.getMonth() === now.getMonth()
            && date.getDate() == now.getDate();

        const pad = (n: number) => String(n).padStart(2, "0");

        if (isToday) {
            return `${pad(date.getHours())}:${pad(date.getMinutes())}`
        } else {
            const shortYear = String(date.getFullYear()).slice(-2);
            return `${pad(date.getHours())}:${pad(date.getMinutes())} ${pad(date.getDate())}.${pad(date.getMonth())}.${shortYear}`
        }
    }
</script>

<style>
    .bot-card {
        padding: 0.2rem;
        border: 1px solid #818181;
        margin-bottom: 0.5rem;
        border-radius: 4px;
    }

    .highlight {
        background-color: #3b3b3b;
    }
</style>

<div>
    bots
    {#each data.bots as bot}
        <div class="bot-card {bot.id % 2 === 0 ? 'highlight' : ''}">
            <a href={`/bot/${bot.id}`}>
                <span>
                    {bot.name}
                    <span>
                        {bot.wins}/{bot.losses}
                    </span>
                    <span>
                        {parseIsoToDate(bot.last_scanned)}
                    </span>
                    <span>
                        {bot.capital + bot.order_capital}
                    </span>
                </span>
                <div>
                    <span>log: </span><span>{bot.log}</span>
                </div>
            </a>
            {#if bot.in_pos}
                <div>
                    {parseIsoToDate(bot.order_created_at)}

                    <span>Pnl:</span><span>{bot.pnl}</span>
                    <span>Roe:</span><span>{bot.roe}</span>

                </div>
            {/if}
        </div>
    {/each}
</div>

