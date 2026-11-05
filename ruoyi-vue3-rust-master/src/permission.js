import router from './router'
import {ElMessage} from 'element-plus'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'
import {getToken} from '@/utils/auth'
import {isHttp} from '@/utils/validate'
import {isRelogin} from '@/utils/request'
import useUserStore from '@/store/modules/user'
import useSettingsStore from '@/store/modules/settings'
import usePermissionStore from '@/store/modules/permission'

NProgress.configure({showSpinner: false});

const whiteList = ['/login', '/register'];

// 纯前端演示模式（GitHub Pages）：没有后端，访问需要登录的页面时自动使用演示账号登录
const demoMode = import.meta.env.VITE_APP_DEMO === 'true';

router.beforeEach((to, from, next) => {
    NProgress.start()
    if (demoMode && !getToken() && whiteList.indexOf(to.path) === -1) {
        useUserStore().login({ username: 'admin', password: 'admin123', code: '', uuid: '' }).then(() => {
            next(to.fullPath)
        }).catch(() => {
            next(`/login?redirect=${to.fullPath}`)
            NProgress.done()
        })
        return
    }
    if (getToken()) {
        const q = {}
        Object.assign(q, to.query)
        delete q['redirect']
        to.meta.title && useSettingsStore().setTitle(to.meta.title)
        /* has token*/
        if (to.path === '/login') {
            const path = to.query.redirect || '/'
            next({path, query: q})
            NProgress.done()
        } else {
            if (useUserStore().roles.length === 0) {
                isRelogin.show = true
                // 判断当前用户是否已拉取完user_info信息
                useUserStore().getInfo().then(() => {
                    isRelogin.show = false
                    usePermissionStore().generateRoutes().then(accessRoutes => {
                        // 根据roles权限生成可访问的路由表
                        accessRoutes.forEach(route => {
                            if (!isHttp(route.path)) {
                                router.addRoute(route) // 动态添加可访问路由表
                            }
                        })
                        next({...to, replace: true}) // hack方法 确保addRoutes已完成
                    })
                }).catch(err => {
                    useUserStore().logOut().then(() => {
                        err && ElMessage.error(err)
                        if (!to.query.redirect) {
                            to.query.redirect = to.path
                        }
                        next({path: '/login', query: to.query})
                    })
                })
            } else {
                next()
            }
        }
    } else {
        // 没有token
        if (whiteList.indexOf(to.path) !== -1) {
            // 在免登录白名单，直接进入
            next()
        } else {
            next(`/login?redirect=${to.fullPath}`) // 否则全部重定向到登录页
            NProgress.done()
        }
    }
})

router.afterEach(() => {
    NProgress.done()
})
