<script lang="ts">
  import {onMount} from "svelte";
  import {writable} from "svelte/store";
  import {API_BASE} from "$lib/config";
  import type {Statistic} from '$lib/types'
  import * as tools from '$lib/tools'

  export let bot_name;


  const data = writable<Statistic | null>(null);
  const loading = writable(true);
  const error = writable<string | null>(null);
  let statistic: Statistic;

  onMount(async () => {
    try {
      const res = await fetch(API_BASE + "/api/v1/bots/" + bot_name + "/statistics");
      if (!res.ok) throw new Error(`failed to load system info: ${res.status}`);

      const json = await res.json() as unknown;

      if (typeof json === 'object' &&
        json != null) {
        data.set(json as Statistic);
      } else {
        throw new Error("invalid api response format")
      }
    } catch (e) {
      error.set((e as Error).message);
    } finally {
      loading.set(false);
    }
  });


</script>
<div class="m-auto text-sm flex justify-center">

{#if $loading}
  <p>Loading...</p>
{:else if $error}
  <p>Error: {$error}</p>
{:else if $data && $data.bot_statistics.length === 0}
  <p>No statistics yet.</p>
{:else if $data}
  <div class="overflow-x-auto">
    <table class="min-w-max text-sm text-neutral-400">
      <tbody>
      {#each $data.bot_statistics as s}
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

{/if}
</div>