<script lang="ts">
  import * as tools from '$lib/tools'
  import type {Statistic} from '$lib/types'

  export let statistic: Statistic;

</script>
<div class="m-auto text-sm flex justify-center">

{#if statistic}
  <div class="overflow-x-auto">
    <table class="min-w-max text-sm text-neutral-400">
      <tbody>
      {#each statistic.bot_statistics as s}
        <tr class="odd:bg-gray-900">
          <td class="table-cell">
            <a href="/bots/{s.bot_name}/">{s.bot_name}</a>
          </td>
          <td class='table-cell {tools.textUpOrDown(s.capital)}'>
            {s.capital.toFixed(2)}
          </td>
          <td class="table-cell">
            <span class='{tools.textUpOrDown(1)}'>{s.win_days}</span>/<span
            class='{tools.textUpOrDown(-1)}'>{s.lose_days}</span>
          </td>
          {#each s.results as res}
            <td class="table-cell">
              <a href="/bots/{s.bot_name}/statistic?start={res.start_time}&end={res.end_time}" class="{tools.textUpOrDown(res.capital - 100)}">
                {res.capital.toFixed(1)}
              </a>
            </td>
          {/each}
        </tr>
      {/each}
      </tbody>
    </table>
  </div>

{/if}
</div>