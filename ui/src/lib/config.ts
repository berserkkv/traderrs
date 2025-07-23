const dev = import.meta.env.DEV;
export const API_BASE = dev ? 'http://localhost:3030' : 'http://193.180.208.245:3030'