<script lang="ts">
  import System from "$lib/component/System.svelte";
  import Orders from "$lib/component/Orders.svelte";
  import BotStatistics from '$lib/component/BotStatistics.svelte'

  export let data2 = [12, 1, 3, 10];
    const max = Math.max(...data2);
    export let data;

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
<BotStatistics bot_name="EmaBounce_1m_Sol" />
<Orders id={data.id}/>
