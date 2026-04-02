# MoonTV 部署指南

## GitHub Actions 自动编译

本项目已配置 GitHub Actions 工作流，会在以下情况触发编译：

1. **推送版本标签** - 推送 `v*` 格式的标签（如 `v1.0.0`）
2. **手动触发** - 在 GitHub Actions 页面手动运行工作流

### 编译产物

工作流会生成一个包含以下内容的部署包：

```
deploy/
├── moontv          # 编译后的二进制文件
└── static/         # 静态文件目录（HTML/CSS/JS）
```

## 部署步骤

### 1. 下载编译产物

在 GitHub 仓库的 **Actions** 页面找到对应的构建记录，下载 `moontv-linux-amd64`  artifact。

### 2. 上传到服务器

将下载的 `deploy` 目录整体上传到 Linux 服务器的任意位置，例如：

```bash
scp -r deploy/* user@your-server:/opt/moontv/
```

### 3. 运行服务

```bash
cd /opt/moontv
./moontv
```

服务启动后会监听 `http://0.0.0.0:8080`（端口可在 `config.json` 中配置）。

### 4. 验证部署

访问 `http://your-server-ip:8080` 应该能看到网页正常显示。

## 发布新版本

```bash
# 本地打标签
git tag v1.0.0
git push origin v1.0.0

# 或者使用语义化版本
git tag v1.2.3
git push origin v1.2.3
```

推送标签后会自动触发 GitHub Actions 编译流程。

## 故障排查

### 网页无法显示

1. **检查日志输出** - 启动时会输出静态文件目录的绝对路径，确认路径正确
2. **检查文件权限** - 确保 `static` 目录及其子目录有读取权限
3. **检查路由** - 确保通过 `/static/css/style.css` 等路径访问静态资源

### 查看运行日志

```bash
# 设置日志级别
RUST_LOG=debug ./moontv

# 只查看错误
RUST_LOG=error ./moontv
```

### 配置文件

编辑 `config.json` 可以修改：

- `host`: 监听地址（默认 `0.0.0.0`）
- `port`: 监听端口（默认 `8080`）
- `db_path`: 数据库文件路径

## Docker 部署（可选）

创建 `Dockerfile`：

```dockerfile
FROM alpine:latest
RUN apk add --no-cache libgcc
WORKDIR /app
COPY deploy/* /app/
EXPOSE 8080
CMD ["./moontv"]
```

构建和运行：

```bash
docker build -t moontv .
docker run -p 8080:8080 moontv
```

## 技术说明

### 静态文件服务修复

本次修复包含以下关键点：

1. **绝对路径支持** - 程序启动时基于可执行文件位置计算 `static` 目录的绝对路径
2. **子目录支持** - 使用 `/static/*path` 通配符路由，支持 CSS/JS 等嵌套目录
3. **安全检查** - 添加路径遍历防护，防止访问静态目录外的文件
4. **State 传递** - 所有静态文件 handler 都接收 `AppState`，包含 `static_dir` 字段

### 代码变更摘要

- `AppState` 结构体新增 `static_dir: String` 字段
- 主函数中计算可执行文件父目录，拼接 `static` 得到绝对路径
- 路由 `/static/*path` 使用 `serve_static_file` handler
- `serve_file` 函数接收 `static_dir` 参数，使用 `std::path::Path` 安全拼接路径
- 添加路径规范化检查，确保请求文件在静态目录内

## 支持

如有问题请提交 Issue 或联系开发团队。
