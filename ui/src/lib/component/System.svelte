<script lang="ts">
    import {onMount} from "svelte";
    import {writable} from "svelte/store";
    import {API_BASE} from "$lib/config";

    type System = {
        cpu_usage: number;
        memory_usage: number;
    }

    const data = writable<System | null>(null);
    const loading = writable(true);
    const error = writable<string | null>(null);

    onMount(async () => {
        try {
            const res = await fetch(API_BASE + "/api/v1/system");
            if (!res.ok) throw new Error(`failed to load system info: ${res.status}`);

            const json = await res.json() as unknown;

            if (typeof json === 'object' &&
                json != null &&
                'cpu_usage' in json &&
                'memory_usage' in json) {
                data.set(json as System)
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
<div class="bg-gray-800 text-xs m-auto flex justify-center text-neutral-300">
    {#if $loading}
        <p>Loading...</p>
    {:else if $error}
        <p>Error: {$error}</p>
    {:else if $data}
        <p>cpu: {$data.cpu_usage}%, mem: {$data.memory_usage}%</p>
    {/if}
</div>


