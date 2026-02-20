import { defineConfig } from 'vite'
import { resolve } from 'path'

export default defineConfig({
    // Базовая директория для dev сервера
    root: 'Main1',

    // Публичная директория для статических файлов
    publicDir: '../public',

        // Настройки сборки
        build: {
            // Выходная директория для сборки
            outDir: '../dist',

            // Очищать выходную директорию перед сборкой
            emptyOutDir: true,

            // Генерировать sourcemaps
            sourcemap: true,

            // Настройки минификации
            minify: 'esbuild',

            // Целевая среда
            target: 'es2020',

            // Настройки rollup
            rollupOptions: {
                input: {
                    main: resolve(__dirname, 'src/index.html'),
                },
                output: {
                    // Имена файлов для chunks
                    chunkFileNames: 'assets/js/[name]-[hash].js',
                    entryFileNames: 'assets/js/[name]-[hash].js',
                    assetFileNames: 'assets/[ext]/[name]-[hash].[ext]'
                }
            }
        },

        // Настройки сервера разработки
        server: {
            // Порт для dev сервера
            port: 1420,

            // Автоматически открывать браузер
            open: true,

            // Разрешить все хосты (для Tauri)
            host: '0.0.0.0',

            // Настройки CORS
            cors: true,

            // Настройки прокси (если нужны)
            proxy: {}
        },

        // Настройки предварительного просмотра
        preview: {
            port: 4173,
            host: '0.0.0.0'
        },

        // Плагины Vite
        plugins: [],

        // Настройки разрешения модулей
        resolve: {
            // Алиасы для путей
            alias: {
                '@': resolve(__dirname, 'src'),
                            '@components': resolve(__dirname, 'src/components'),
                            '@styles': resolve(__dirname, 'src/styles'),
                            '@assets': resolve(__dirname, 'src/assets')
            }
        },

        // Настройки CSS
        css: {
            // Модули CSS
            modules: {
                localsConvention: 'camelCaseOnly',
                scopeBehaviour: 'local',
                generateScopedName: '[name]__[local]__[hash:base64:5]'
            },

            // Препроцессоры
            preprocessorOptions: {
                scss: {
                    additionalData: '@import "@styles/variables.scss";'
                }
            }
        },

        // Настройки оптимизации зависимостей
        optimizeDeps: {
            include: ['@tauri-apps/api'],
            exclude: []
        },

        // Настройки для Tauri
        clearScreen: false,

        // Базовая директория для Tauri
        base: './',

        // Настройки env
        envPrefix: ['VITE_', 'TAURI_'],

        // Режим разработки
        mode: process.env.NODE_ENV || 'development'
})
