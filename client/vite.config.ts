import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    server: {
        port: 8080,
        proxy: {
            '/api/': 'http://192.168.1.4:8000/',
        },
    },
});
