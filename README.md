# MoonTV

Rust 实现的简化版视频聚合平台，使用 SQLite 作为数据库。

## 特性

- 🦀 **Rust** - 高性能、内存安全
- 💾 **SQLite** - 单文件数据库，无需额外依赖
- 📦 **单文件** - 静态编译，无运行时依赖
- 🚀 **轻量** - musl 静态链接，Linux AMD64 单文件运行

## 构建

项目使用 GitHub Actions 自动构建 Linux AMD64 二进制文件。

### 手动构建

```bash
# 开发模式
cargo build

# 发布模式（需要 musl 目标）
cargo build --release --target x86_64-unknown-linux-musl
```

### 手动触发构建

在 GitHub 仓库页面：
1. 进入 **Actions** 标签
2. 选择 **Build Linux AMD64 Binary** 工作流
3. 点击 **Run workflow** 按钮

## 运行

```bash
# 使用默认配置
./moontv

# 指定端口
PORT=8080 ./moontv

# 使用自定义配置文件
cp config.json.example config.json
# 编辑 config.json
./moontv
```

## 配置

编辑 `config.json` 文件：

```json
{
  "host": "0.0.0.0",
  "port": 3000,
  "username": "admin",
  "password": "your_password",
  "cache_time": 7200,
  "db_path": "data/moontv.db",
  "api_site": {
    "site_name": {
      "api": "https://api.example.com/provide/vod",
      "name": "视频站名称",
      "detail": "https://example.com",
      "is_adult": false
    }
  }
}
```

## API 端点

| 端点 | 方法 | 说明 |
|------|------|------|
| `/api/health` | GET | 健康检查 |
| `/api/config` | GET | 获取配置 |
| `/api/search` | GET | 搜索视频 |
| `/api/detail` | GET | 获取视频详情 |
| `/api/play` | GET | 获取播放地址 |
| `/api/tvbox` | GET | TVBox 兼容接口 |
| `/api/login` | POST | 用户登录 |
| `/api/favorites` | GET/POST | 收藏管理 |
| `/api/playrecords` | GET/POST | 播放记录 |
| `/api/user/preferences` | GET/POST | 用户偏好 |

## TVBox 兼容

支持 TVBox 标准接口格式：

- JSON: `http://your-server/api/tvbox?format=json`
- TXT: `http://your-server/api/tvbox?format=txt`

## 目录结构

```
moontv/
├── src/
│   ├── main.rs          # 主程序入口
│   ├── config.rs       # 配置处理
│   ├── api/            # API 处理器
│   ├── db/             # 数据库操作
│   ├── models/         # 数据模型
│   └── utils/          # 工具函数
├── static/             # 静态文件
│   ├── index.html
│   ├── css/
│   └── js/
├── .github/
│   └── workflows/
│       └── build.yml   # CI 工作流
├── Cargo.toml
└── config.json
```

## 许可证

MIT