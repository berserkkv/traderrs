import {API_BASE} from '$lib/config'
import type {Order} from '$lib/types'
import {formatToShort} from '$lib/tools'

/** @type {import('./$types').PageLoad } */
export  async function load({params, url, fetch}) {
  const bot_name= params.id;
  const start_time_full = url.searchParams.get('start') ?? '';
  const end_time_full = url.searchParams.get('end') ?? '';

  const start_time = formatToShort(start_time_full) ?? '';
  const end_time = formatToShort(end_time_full) ?? '';

  try {
    const res = await fetch(API_BASE + `/api/v1/bots/${bot_name}/statistics/range?start_time=${encodeURIComponent(start_time)}&end_time=${encodeURIComponent(end_time)}`);
    if (!res.ok) throw new Error(`failed to load system info: ${res.status}`);

    const json = await res.json() as unknown;
    if (typeof json === 'object' &&
      json != null) {

      return {
        bot_name: bot_name,
        start_time: start_time,
        end_time: end_time,
        arr: json as Order[],
      }
    } else {
      throw new Error("invalid api response format")
    }
  } catch (e) {
    console.log((e as Error).message);
  }




}