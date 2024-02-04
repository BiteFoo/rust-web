#Rust-Web-Server
基于`rust+axum`开发的一个 web服务端，当然会将`react`打包的 html 等文件一同部署。

## 开发命令
```shell

# 启动main  Terminal1
cargo watch -q -c -w src/  -w .cargo/ -x "run"
# 启动example Terminal2
cargo watch -q -c -w exmaples/ -x "run --example dev"
```


## 参考资料
- axum的基本使用方法
```shell
https://mp.weixin.qq.com/s?__biz=Mzg3NjU2OTE1Mw==&mid=2247491296&idx=1&sn=52e4d75535dc0b4ee1c4e2535a232bf9&chksm=cf3169cef846e0d8dd06bb0196e9dbc29847a034ece26053ee053a881d0efb0d8a6da8adcff8&scene=178&cur_album_id=2653052954631061505#rd
```
- youtube上的博主提供的程序
```shell
https://github.com/rust10x/rust-web-app/blob/main/crates/services/web-server/src/main.rs
```