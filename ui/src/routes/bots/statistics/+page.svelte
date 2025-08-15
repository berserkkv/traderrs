<script lang="ts">
  import type {Statistic} from "$lib/types";
  import * as tools from "$lib/tools";
  import System from "$lib/component/System.svelte";

  export let data: { statistic: Statistic };

  const statistic = data.statistic;

</script>

<div>
  <System/>
  <a class="underline text-blue-600" href="/">Main</a>
  <div class="overflow-x-auto">
  <table class="min-w-max text-sm text-neutral-400">
    <tbody>
    {#each statistic.bot_statistics as s}
      <tr class="odd:bg-gray-900">
        <td class="table-cell">
          <p>{s.bot_name}</p>
        </td>
        <td class='table-cell {tools.textUpOrDown(s.capital)}'>
          {s.capital}
        </td>
        <td class="table-cell">
          <span class='{tools.textUpOrDown(1)}'>{s.win_days}</span>/<span
          class='{tools.textUpOrDown(-1)}'>{s.lose_days}</span>
        </td>
        {#each s.results as res}
          <td class="table-cell">
            <p class="{tools.textUpOrDown(res.capital - 100)}">
              {res.capital.toFixed(1)}
            </p>
          </td>
        {/each}
      </tr>
    {/each}
    </tbody>
  </table>
  </div>
</div>