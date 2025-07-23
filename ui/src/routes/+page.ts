import {API_BASE} from "$lib/config";

export async function load({fetch}) {
    const res = await fetch(`${API_BASE}/api/v1/bots`);
    const bots = await res.json();
    return {bots};
}

