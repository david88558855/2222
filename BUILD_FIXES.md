# MoonTV 网页显示问题修复说明

## 🔧 修复的问题

### 1. 路由 Handler 不匹配
**问题**: 代码中引用了未定义的 `serve_static_file` 和 `serve_static_file_index` 函数  
**修复**: 统一使用正确的 handler 函数，并添加 `State` 参数以访问应用状态

### 2. 静态文件路径问题
**问题**: 使用相对路径 `static/`，当从不同目录运行二进制文件时无法找到静态文件  
**修复**: 
- 在启动时获取可执行文件的绝对路径
- 基于可执行文件路径计算 `static` 目录的绝对路径
- 将绝对路径存储在 `AppState` 中供所有 handler 使用

### 3. 缺少子目录支持
**问题**: 路由 `/:file` 只能匹配单层路径，无法处理 `/css/style.css` 或 `/js/app.js`  
**修复**: 
- 修改路由为 `/static/*path` 通配符模式
- 支持任意深度的静态文件子目录

### 4. 安全性增强
**新增**: 添加路径遍历安全检查，防止通过 `../` 访问静态目录外的文件

## 📝 代码变更摘要

### `src/main.rs` 主要修改:

```rust
// 1. AppState 增加 static_dir 字段
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub config: AppConfig,
    pub static_dir: String,  // 新增
}

// 2. 启动时计算静态文件目录的绝对路径
let exe_path = std::env::current_exe()
    .expect("Failed to get executable path")
    .parent()
    .expect("Failed to get executable parent")
    .to_path_buf();
let static_dir = exe_path.join("static").to_string_lossy().to_string();

// 3. 更新路由配置
.route("/static/*path", get(serve_static_file))

// 4. 更新 handler 函数签名
async fn serve_index(State(state): State<AppState>) -> impl IntoResponse
async fn serve_static_file(
    State(state): State<AppState>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse

// 5. 使用绝对路径读取文件
let path = std::path::Path::new(static_dir).join(file);
```

## 🚀 编译部署指南

### 在 Linux/macOS 上编译

#### 方法一：使用编译脚本（推荐）

```bash
# 赋予执行权限
chmod +x build.sh

# 编译（Release 模式）
./build.sh

# 或清理后重新编译
./build.sh --clean
```

#### 方法二：手动编译

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 编译 Release 版本
cargo build --release

# 二进制文件位置
./target/release/moontv
```

### 部署步骤

编译脚本会自动创建 `deploy/` 目录，包含：
```
deploy/
├── moontv          # 编译后的二进制文件
├── static/         # 静态文件目录（HTML/CSS/JS）
└── config.json     # 配置文件
```

运行服务：
```bash
cd deploy
./moontv
```

访问：http://localhost:8080

### 跨平台编译（可选）

如需在其他平台编译，可使用交叉编译工具：

```bash
# 安装 cross 工具
cargo install cross

# 为 Linux x86_64 编译
cross build --release --target x86_64-unknown-linux-gnu

# 为 macOS ARM64 编译
cross build --release --target aarch64-apple-darwin
```

## ✅ 验证修复

启动服务后，检查以下端点是否正常返回：

```bash
# 主页 HTML
curl http://localhost:8080/

# CSS 文件
curl http://localhost:8080/static/css/style.css

# JS 文件
curl http://localhost:8080/static/js/app.js

# API 端点
curl http://localhost:8080/api/health
```

所有请求都应该返回正确的 Content-Type 和内容。

## 🐛 故障排查

### 问题：仍然无法加载静态文件

**检查项**:
1. 确认 `static/` 目录与二进制文件在同一目录
2. 检查文件权限：`chmod -R 755 static/`
3. 查看日志输出，确认静态文件目录路径正确

### 问题：404 Not Found

**可能原因**:
- 请求的路径不正确
- 文件确实不存在
- 权限问题导致无法读取文件

**解决**: 查看服务器日志，会输出详细的文件路径信息

## 📊 性能优化建议

当前 `Cargo.toml` 已配置 Release 优化：
```toml
[profile.release]
opt-level = "z"      # 优化体积
lto = true           # 链接时优化
codegen-units = 1    # 单个代码生成单元
strip = true         # 去除符号表
panic = "abort"      #  panic 时直接中止
```

这会产生最小化的二进制文件，适合部署。
