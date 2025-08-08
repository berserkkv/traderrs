import {API_BASE} from "$lib/config";

export async function load({fetch}) {
  const res = await fetch(`${API_BASE}/api/v1/bots/statistics`);
  const data = await res.json();
  return {data};
}
