<script lang="ts">
  import type {Statistic} from "$lib/types";
  import System from "$lib/component/System.svelte";
  import BotStatistics from '$lib/component/BotStatistics.svelte'

  export let data: { statistic: Statistic };

  const statistic = data.statistic;

</script>

<div>
  <System/>
  <a class="underline text-blue-600" href="/">Main</a>
  <BotStatistics statistic={statistic}/>
</div>