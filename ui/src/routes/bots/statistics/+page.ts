import {API_BASE} from "$lib/config";
import type {BotResultMap} from "$lib/types";

export async function load({fetch}) {
  const res = await fetch(`${API_BASE}/api/v1/bots/statistics`);
  if (!res.ok) throw  new Error('Failed to load bot results');

  const botResults: BotResultMap = await res.json()
  return {botResults};
}
