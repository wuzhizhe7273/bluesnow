# BlueSnow:一个简单的博客系统

## 配置优先级
1. 命令行参数
2. 环境变量
3. 配置文件
## 迁移数据库
~~~bash
cargo run  --manifest-path ./crates/migration/Cargo.toml -- refresh -u postgres://bluesnow:123456@localhost:5432/bluesnow 
~~~

## 生成entity
~~~bash
sea generate entity -l --with-serde both --expanded-format -u postgres://bluesnow:123456@localhost:5432/bluesnow -o ./crates/entity/src 
~~~