const dev = import.meta.env.DEV;
export const API_BASE = dev ? 'http://localhost:3030' : 'http://77.37.96.91:3030'