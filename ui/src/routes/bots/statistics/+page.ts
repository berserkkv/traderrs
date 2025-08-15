import {API_BASE} from "$lib/config";
import type {Statistic} from "$lib/types";

export async function load({fetch}) {
  const res = await fetch(`${API_BASE}/api/v1/bots/statistics`);
  if (!res.ok) throw  new Error('Failed to load bot results');

  const statistic: Statistic = await res.json()

  return {statistic};
}
