import { defineConfig, loadEnv } from 'vite'
import path from 'path'
import createVitePlugins from './vite/plugins'

// https://vitejs.dev/config/
export default defineConfig(({ mode, command }) => {
  const env = loadEnv(mode, process.cwd())
  const { VITE_APP_ENV } = env
  // GitHub Pages 部署在子路径时通过环境变量注入，例如 /ruoyi-rust-iot/
  const appBase = process.env.VITE_APP_BASE || env.VITE_APP_BASE || '/'
  return {
    // 部署生产环境和开发环境下的URL。
    // 默认情况下，vite 会假设你的应用是被部署在一个域名的根路径上
    // 例如 https://www.ruoyi.vip/。如果应用被部署在一个子路径上，你就需要用这个选项指定这个子路径。例如，如果你的应用被部署在 https://www.ruoyi.vip/admin/，则设置 baseUrl 为 /admin/。
    base: appBase,
    plugins: createVitePlugins(env, command === 'build'),
    resolve: {
      // https://cn.vitejs.dev/config/#resolve-alias
      alias: {
        // 设置路径
        '~': path.resolve(__dirname, './'),
        // 设置别名
        '@': path.resolve(__dirname, './src')
      },
      // https://cn.vitejs.dev/config/#resolve-extensions
      extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue']
    },
    build: {
      minify: 'terser', // 必须开启：使用 terserOptions 才有效果
      terserOptions: {
        compress: {
          drop_console: VITE_APP_ENV === 'production' ? true : false,
          drop_debugger: VITE_APP_ENV === 'production' ? true : false,
        },
      },
      rollupOptions: {
        output: {
          chunkFileNames: 'js/[name]-[hash].js',
          entryFileNames: 'js/[name]-[hash].js',
          assetFileNames: (assetInfo) => {
            if (/\.(png|svg|jpg|jpeg|gif|ttf)$/.test(assetInfo.name)) { // 匹配资源文件后缀
              return `img/[name]-[hash].[ext]`;  // 创建media文件夹存放匹配的资源文件,name为该文件的原名，hash为哈希值，ext为文件后缀名，以[name].[hash][ext]命名规则
            }
            if (/\.(mp4|webm|ogg|mp3|wav|flac|aac)$/.test(assetInfo.name)) { // 匹配资源文件后缀
              return `media/[name].[hash][ext]`;  // 创建media文件夹存放匹配的资源文件,name为该文件的原名，hash为哈希值，ext为文件后缀名，以[name].[hash][ext]命名规则
            }
            return `css/[name]-[hash].[ext]`; // 不匹配的资源文件存放至assets，以[name]-[hash].[ext]命名规则，注意两处的命名规则不同
          },
        },
      }
    },

    // vite 相关配置
    server: {
      port: 80,
      host: true,
      open: true,
      proxy: {
        // https://cn.vitejs.dev/config/#server-proxy
        '/dev-api': {
          target: 'http://127.0.0.1:60800',
          changeOrigin: true,
          rewrite: (p) =>  p.replace(/^\/dev-api/, '/dev-api')
        }
      }
    },
    //fix:error:stdin>:7356:1: warning: "@charset" must be the first rule in the file
    css: {
      postcss: {
        plugins: [
          {
            postcssPlugin: 'internal:charset-removal',
            AtRule: {
              charset: (atRule) => {
                if (atRule.name === 'charset') {
                  atRule.remove();
                }
              }
            }
          }
        ]
      }
    }
  }
})
