<script lang="ts">
  import type {BotResult, BotResultMap} from "$lib/types";
  import * as tools from "$lib/tools";

  export let data: { botResults: BotResultMap };

  const botResults = data.botResults;
  
  const entries: [string, BotResult[]][] = Object.entries(botResults ?? {});
</script>

<table>
  <tbody>
  {#each entries as [name, results]}
    <tr class="odd:bg-gray-900">
      <td class="table-cell text-neutral-300">
        <p>{name}</p>
      </td>
      {#each results as res}
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
