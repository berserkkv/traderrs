<script lang="ts">
  import System from "$lib/component/System.svelte";
  import Orders from '$lib/component/Orders.svelte'
  import BotStatistics from '$lib/component/BotStatistics.svelte'

  export let data;

</script>

<System/>
<BotStatistics statistic={data.statistic} />
<Orders orders={data.orders} />
