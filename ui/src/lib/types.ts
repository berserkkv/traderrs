

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

export type Bot = {
    id: number;
    name: string;
    in_pos: boolean;
    is_not_active: boolean;
    is_trailing_stop_active: boolean;
    last_scanned: string;
    leverage: number;
    wins: number;
    losses: number;
    capital: number;
    order_capital: number;
    log: string;
    order_capital_with_leverage: number;
    order_created_at: string;
    order_entry_price: number;
    order_scanned_at: string;
    order_fee: number;
    order_quantity: number;
    order_stop_loss: number;
    order_take_profit: number;
    order_type: string;
    pnl: number;
    roe: number;
    stop_loss_ratio: number;
    take_profit_ratio: number;
    strategy_name: string;
    symbol: string;
    timeframe: string;
    trailing_stop_activation_point: number;
}