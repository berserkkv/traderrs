const dev = import.meta.env.DEV;
export const API_BASE = dev ? 'http://localhost:3030' : 'http://92.113.151.200:3030'
// export const API_BASE = import.meta.env.VITE_API_BASE;