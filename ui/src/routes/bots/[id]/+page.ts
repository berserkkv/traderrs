import {API_BASE} from "$lib/config";
import type {Order, Statistic} from '$lib/types';
import {error} from '@sveltejs/kit';

export async function load({ params, fetch }) {
  const id = params.id;

  try {
    const [ordersRes, statsRes] = await Promise.all([
      fetch(`${API_BASE}/api/v1/bots/${id}/orders`),
      fetch(`${API_BASE}/api/v1/bots/${id}/statistics`)
    ]);

    if (!ordersRes.ok) {
      throw error(ordersRes.status, `Failed to load orders: ${ordersRes.statusText}`);
    }
    if (!statsRes.ok) {
      throw error(statsRes.status, `Failed to load statistics: ${statsRes.statusText}`);
    }

    const orders = await ordersRes.json() as Order[];
    const statistic = await statsRes.json() as Statistic;

    return { id, orders, statistic };

  } catch (err) {
    console.error(err);
    throw error(500, "Could not load bot data");
  }
}
