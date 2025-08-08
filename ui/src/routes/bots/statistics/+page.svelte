<script lang="ts">

  export let data: Record<string, [number, string][]>;


  console.log(data);
</script>

{#each Object.entries(data.data) as [name, arr]}
  <p>{name}</p>
  {#each arr as [capital, time]}
    <span class="text-xs">{capital} </span>
  {/each}
{/each}