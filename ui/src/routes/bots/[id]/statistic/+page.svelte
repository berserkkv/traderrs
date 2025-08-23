<script lang="ts">

  import Orders from '$lib/component/Orders.svelte'
  import type {Order} from '$lib/types'

  export let data;

  let orders: Order[] | undefined = data.arr;
</script>
<div>
  {data.bot_name}
  <p>start: {data.start_time}</p>
  <p>end :  {data.end_time}</p>
  {#if orders}
  <Orders orders={orders}/>
    {/if}
</div>