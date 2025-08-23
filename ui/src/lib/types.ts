

export type StatisticResult = {
  name: string,
  capital: number,
  wins: number,
  losses: number,
  start_time: string,
  end_time: string,
};

export type BotStatistic = {
  bot_name: string,
  win_days: number,
  lose_days: number,
  capital: number,
  results: StatisticResult[],
}

export type Statistic = {
  bot_statistics: BotStatistic[],
}

export type Order = {
  bot_id: number;
  symbol: string;
  entry_price: number;
  exit_price: number;
  fee: number;
  quantity: number;
  pnl: number;
  roe: number;
  order_type: string;
  leverage: number;
  created_at: string;
  closed_at: string;
}

export type ChartData = {
  value: number;
  time: string;
}