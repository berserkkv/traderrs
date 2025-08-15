

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