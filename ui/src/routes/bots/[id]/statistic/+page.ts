import {API_BASE} from '$lib/config'
import {formatToShort} from '$lib/tools'
import type {Statistic} from '$lib/types'

/** @type {import('./$types').PageLoad } */
export  async function load({params, url, fetch}) {
  const bot_name= params.id;
  const start_time_full = url.searchParams.get('start') ?? '';
  const end_time_full = url.searchParams.get('end') ?? '';

  const start_time = formatToShort(start_time_full) ?? '';
  const end_time = formatToShort(end_time_full) ?? '';


  const res = await fetch(API_BASE + `/api/v1/bots/${bot_name}/statistics/range?start_time=${encodeURIComponent(start_time)}&end_time=${encodeURIComponent(end_time)}`);
  if (!res.ok) throw new Error(`failed to load system info: ${res.status}`);

  const arr: Statistic = await res.json();
  return {
    bot_name: bot_name,
    start_time: start_time,
    end_time: end_time,
    arr: arr
  }
}