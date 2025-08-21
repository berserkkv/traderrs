export function parseIsoToDate(isoString: string): string {
  const date = new Date(isoString);
  const now = new Date();

  const isToday =
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() == now.getDate();

  const pad = (n: number) => String(n).padStart(2, "0");

  if (isToday) {
    return `${pad(date.getHours())}:${pad(date.getMinutes())}`;
  } else {
    const shortYear = String(date.getFullYear()).slice(-2);
    return `${pad(date.getHours())}:${pad(date.getMinutes())} ${pad(date.getDate())}/${pad(date.getMonth())}`;
  }
}

export function textUpOrDown(num: number): string {
  if (num > 0.0) {
    return "text-blue-700";
  } else if (num < 0.0) {
    return "text-fuchsia-800";
  }
  return "text-neutral-600";
}

export function bgUpOrDown(num: number): string {
  if (num > 0.0) {
    return "bg-blue-950";
  } else if (num < 0.0) {
    return "bg-fuchsia-950";
  }
  return "text-neutral-700";
}

export function calculateWinPercentage(wins: number, total: number): number {
  if (total === 0) {
    return 0.0;
  }
  return wins * 100 / total;
}

export function borderUpOrDown(num: number): string {
  if (num > 0.0) {
    return "border-blue-950";
  } else if (num < 0.0) {
    return "border-fuchsia-950";
  }
  return "text-neutral-700";
}

export function formatToShort(datetimeStr: string | null) {
  if (!datetimeStr) return null;
  // Extract only the first 16 characters: "YYYY-MM-DDTHH:MM"
  // This works if the input is like "2025-08-17T15:27:23.017469061 03:00"
  const shortStr = datetimeStr.slice(0, 16);
  return shortStr.replace(' ', 'T'); // just in case there is a space
}