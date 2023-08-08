import axios from 'axios';
import { headers } from 'next/dist/client/components/headers';

const DOMAIN = process.env.NEXT_PUBLIC_API_HOST;
const HOST_API = `${DOMAIN}`;
const axiosInstance = axios.create({ baseURL: HOST_API, headers: { 'Content-Type': 'application/json' }});

axiosInstance.interceptors.response.use(
  (response) => response,
  (error) => Promise.reject((error.response && error.response.data) || 'Something went wrong')
);

export default axiosInstance;

export const API_ENDPOINTS = {
	file: {
		meta: (hash: string) => (`/meta/${hash}`),
        upload: '/upload',
        download: (hash: string) => (`/download/${hash}`),

	},
    misc: {
        health: '/health',
    }
};

