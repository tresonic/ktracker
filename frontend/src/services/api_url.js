const isDev = () => !process.env.NODE_ENV || process.env.NODE_ENV === 'development';
export const apiUrl = () => isDev() ? "http://" + window.location.hostname + ":3000/api/" : "/api/";