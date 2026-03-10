

#[allow(dead_code)]
pub fn bollinger_bands_b(prices: &[f64], period: usize) -> f64 {
    let n = prices.len();
    if n < period || period == 0 { return f64::NAN; }

    let slice = &prices[n - period..];
    let sma = slice.iter().sum::<f64>() / period as f64;
    let variance = slice.iter().map(|p| (p - sma).powi(2)).sum::<f64>() / period as f64;
    let stddev = variance.sqrt();
    let upper = sma + 2.0 * stddev;
    let lower = sma - 2.0 * stddev;
    (slice.last().unwrap() - lower) / (upper - lower)
}

#[allow(dead_code)]
pub fn ema(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period || period == 0 {
        return f64::NAN;
    }
    let k = 2.0 / (period + 1) as f64;
    let mut ema = prices.iter().take(period).sum::<f64>() / period as f64;
    for i in period..prices.len() {
        ema = (prices[i] - ema) * k + ema;
    }
    ema
}

#[allow(dead_code)]
pub fn slma(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period {
        return 0.0;
    }
    let weight_sum = period * (period + 1) / 2;
    prices[prices.len() - period..]
        .iter()
        .enumerate()
        .map(|(i, &v)| v * (i + 1) as f64)
        .sum::<f64>()
        / weight_sum as f64
}

#[allow(dead_code)]
pub fn macd(prices: &[f64]) -> (f64, f64, f64) {
    let (macd, signal, hist) = macd_slice(prices);
    (
        *macd.last().unwrap_or(&0.0),
        *signal.last().unwrap_or(&0.0),
        *hist.last().unwrap_or(&0.0),
    )
}

pub fn macd_slice(prices: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let fast = ema_slice(prices, 12);
    let slow = ema_slice(prices, 25);
    let macd: Vec<f64> = fast.iter().zip(slow.iter()).map(|(f, s)| f - s).collect();
    let signal = ema_slice(&macd, 9);
    let histogram: Vec<f64> = macd.iter().zip(signal.iter()).map(|(m, s)| m - s).collect();
    (macd, signal, histogram)
}

#[allow(dead_code)]
pub fn slma_slice(prices: &[f64], period: usize) -> Vec<f64> {
    if prices.len() < period {
        return vec![];
    }
    let weight_sum = period * (period + 1) / 2;
    (0..=prices.len() - period)
        .map(|i| {
            (0..period)
                .map(|j| prices[i + j] * (j + 1) as f64)
                .sum::<f64>()
                / weight_sum as f64
        })
        .collect()
}

pub fn ema_slice(prices: &[f64], period: usize) -> Vec<f64> {
    let mut result = vec![0.0; prices.len()];
    if prices.is_empty() || period == 0 {
        return result;
    }
    let k = 2.0 / (period + 1) as f64;
    let initial_ema = prices.iter().take(period).sum::<f64>() / period as f64;
    result[period - 1] = initial_ema;
    for i in period..prices.len() {
        result[i] = (prices[i] - result[i - 1]) * k + result[i - 1];
    }
    result
}

#[allow(dead_code)]
pub fn bollinger_percent_b_slice(prices: &[f64], period: usize) -> Vec<f64> {
    let n = prices.len();
    let mut result = vec![0.0; n];

    for i in period - 1..n {
        let sma: f64 = prices[i + 1 - period..=i].iter().sum::<f64>() / period as f64;
        let variance: f64 = prices[i + 1 - period..=i]
            .iter()
            .map(|p| (p - sma).powi(2))
            .sum::<f64>()
            / period as f64;
        let stddev = variance.sqrt();
        let upper = sma + 2.0 * stddev;
        let lower = sma - 2.0 * stddev;

        result[i] = (prices[i] - lower) / (upper - lower);
    }
    result
}


#[allow(dead_code)]
pub fn stochastic_slice(
    close: &[f64],
    high: &[f64],
    low: &[f64],
    period_k: usize,
    smooth_k: usize,
    period_d: usize,
) -> (Vec<f64>, Vec<f64>) {

    let n = close.len();
    let mut raw_k = vec![f64::NAN; n];

    if n == 0 || period_k == 0 {
        return (raw_k.clone(), raw_k);
    }

    // --- raw stochastic ---
    for i in period_k - 1..n {
        let low_slice = &low[i + 1 - period_k..=i];
        let high_slice = &high[i + 1 - period_k..=i];

        let lowest = low_slice.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let highest = high_slice.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let denom = highest - lowest;

        raw_k[i] = if denom == 0.0 {
            0.0
        } else {
            100.0 * (close[i] - lowest) / denom
        };
    }

    // --- smooth %K ---
    let k = sma_slice(&raw_k, smooth_k);

    // --- %D ---
    let d = sma_slice(&k, period_d);

    (k, d)
}

pub fn sma_slice(values: &[f64], period: usize) -> Vec<f64> {
    let mut result = vec![f64::NAN; values.len()];

    if period == 0 || values.len() < period {
        return result;
    }

    for i in period - 1..values.len() {
        let mut sum = 0.0;
        let mut count = 0;

        for j in i + 1 - period..=i {
            if values[j].is_finite() {
                sum += values[j];
                count += 1;
            }
        }

        if count > 0 {
            result[i] = sum / count as f64;
        }
    }

    result
}