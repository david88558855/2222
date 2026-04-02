# MoonTV 重大升级完成

## 升级概述

本次升级完成了从多文件部署到单二进制文件、从简单登录到完整用户系统、从基础页面到全功能管理后台的重大改进。

---

## 🎯 核心升级内容

### 1. 单二进制文件部署 ✨

**问题**: 之前需要将 `static` 目录与二进制文件一起部署，容易丢失文件导致网页无法显示。

**解决方案**: 使用 `rust-embed` crate 将所有静态资源（HTML、CSS、JS）编译进单一可执行文件。

**技术实现**:
```rust
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAssets;

fn serve_embedded(path: &str) -> impl IntoResponse {
    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (StatusCode::OK, [(Content-Type, mime)], content.data.to_vec())
        }
        None => not_found()
    }
}
```

**优势**:
- ✅ 单一文件部署，无需担心静态文件丢失
- ✅ 从任意目录运行二进制文件都能正常工作
- ✅ 减少部署步骤和出错概率
- ✅ 使用 gzip 压缩减小二进制体积

**依赖更新**:
```toml
[dependencies]
rust-embed = { version = "8.0", features = ["compression"] }
mime_guess = "2.0"
```

---

### 2. 用户注册功能 🔐

**新增 API 接口**:
- `POST /api/register` - 用户注册
- `POST /api/login` - 用户登录（已升级为数据库认证）
- `POST /api/logout` - 用户登出（已添加会话失效）

**注册流程**:
1. 用户点击"立即注册"切换到注册模式
2. 输入用户名、密码、确认密码
3. 前端验证（非空、密码长度、一致性）
4. 后端验证（用户名唯一性、密码强度）
5. SHA256 加密存储密码
6. 自动创建用户偏好设置记录
7. 注册成功后提示登录

**安全特性**:
- ✅ 密码 SHA256 哈希存储
- ✅ 用户名唯一性检查
- ✅ 密码长度要求（最少 6 位）
- ✅ 密码一致性验证
- ✅ 会话令牌（UUID v4）管理
- ✅ 登出时会话失效

**数据库变更**:
```sql
-- Users table (已有)
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT DEFAULT 'user',
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Sessions table (已有)
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    user_id INTEGER,
    created_at INTEGER,
    expires_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

---

### 3. 完整管理后台 🎛️

#### 3.1 用户管理

**功能**:
- ✅ 查看所有用户列表（ID、用户名、角色、注册时间）
- ✅ 区分管理员和普通用户（彩色标签）
- ✅ 删除普通用户（admin 账号不可删除保护）
- ✅ 批量操作支持（预留）

**API**:
- `GET /api/admin/users` - 获取用户列表
- `DELETE /api/admin/users/:id` - 删除用户

**界面特性**:
- 表格展示，支持横向滚动（移动端）
- 角色标签（管理员红色、普通用户蓝色）
- 删除二次确认对话框
- 操作反馈提示

#### 3.2 内容管理

**功能**:
- ✅ 查看缓存的视频内容列表
- ✅ 删除视频缓存（开发中）
- ✅ 分类管理（预留）

**API**:
- `GET /api/admin/videos` - 获取视频列表
- `DELETE /api/admin/videos/:id` - 删除视频

#### 3.3 系统设置

**功能**:
- ✅ 网站名称配置
- ✅ 注册开关控制
- ✅ 最大搜索结果数限制
- ✅ 设置持久化（开发中）

**API**:
- `GET /api/admin/settings` - 获取系统设置
- `POST /api/admin/settings` - 更新系统设置

**界面特性**:
- 表单式配置界面
- 实时保存反馈
- 输入验证

---

### 4. 多端响应式适配 📱💻📺

#### 4.1 安卓手机（<768px）

**适配要点**:
- 隐藏侧边栏（改为底部导航或汉堡菜单 - 待实现）
- 搜索框独占一行
- 视频网格 2 列布局
- 管理表格支持横向滚动
- 表单字段垂直排列
- 按钮和触摸目标加大（最小 44x44px）
- 字体大小适中（14-16px）

**CSS**:
```css
@media (max-width: 768px) {
    .sidebar { display: none; }
    .video-grid { grid-template-columns: repeat(2, 1fr); }
    .admin-tabs { flex-direction: column; }
    .setting-item { flex-direction: column; }
}
```

#### 4.2 PC 电脑（769px - 1919px）

**适配要点**:
- 标准侧边栏布局
- 视频网格自适应（3-5 列）
- 完整的管理后台表格
- 表单水平布局

**平板优化（769px - 1024px）**:
```css
@media (min-width: 769px) and (max-width: 1024px) {
    .video-grid { grid-template-columns: repeat(3, 1fr); }
    .sidebar { width: 180px; }
}
```

#### 4.3 TV 大屏（≥1920px）

**适配要点**:
- 基础字体放大到 18px
- 侧边栏加宽到 280px
- 视频网格更大间距（2rem）
- 管理表格行高增加
- 所有元素适当放大

**CSS**:
```css
@media (min-width: 1920px) {
    body { font-size: 18px; }
    .sidebar { width: 280px; }
    .video-grid { 
        grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); 
        gap: 2rem;
    }
}
```

#### 4.4 辅助功能

- **焦点状态**: 所有交互元素有清晰的焦点轮廓
- **打印优化**: 打印时隐藏导航和播放器控件
- **暗色主题**: 默认暗色主题，适合 TV 观看环境

---

## 📁 文件变更清单

### 后端文件

| 文件 | 变更类型 | 说明 |
|------|---------|------|
| `Cargo.toml` | 修改 | 添加 rust-embed、mime_guess 依赖 |
| `src/main.rs` | 重写 | 集成 rust-embed，添加管理员 API 路由 |
| `src/api/mod.rs` | 修改 | 添加 admin 模块声明 |
| `src/api/admin.rs` | 新建 | 完整的管理后台 API 实现 |
| `src/api/auth.rs` | 重写 | 添加注册功能，升级为数据库认证 |
| `src/db/mod.rs` | 修改 | 添加用户管理、会话管理方法 |

### 前端文件

| 文件 | 变更类型 | 说明 |
|------|---------|------|
| `static/index.html` | 修改 | 添加注册表单字段 |
| `static/js/app.js` | 重写 | 添加注册、管理后台完整逻辑 |
| `static/css/style.css` | 重写 | 添加管理后台样式和完整响应式布局 |

### 文档文件

| 文件 | 变更类型 | 说明 |
|------|---------|------|
| `UPGRADE_COMPLETE.md` | 新建 | 本次升级的完整说明文档 |

---

## 🚀 使用说明

### 编译构建

```bash
# 本地调试编译
cargo build

# 发布版本编译（推荐）
cargo build --release
```

编译后的单一可执行文件位置：
- Linux/macOS: `target/release/moontv`
- Windows: `target/release/moontv.exe`

### 运行

```bash
# 直接运行（从任意目录）
./moontv

# 指定配置文件
MOONTV_CONFIG=/path/to/config.json ./moontv
```

### 默认管理员账号

- **用户名**: `admin`
- **密码**: `admin`
- **角色**: 管理员

⚠️ **重要**: 首次登录后请立即修改默认密码！

### 用户注册

1. 点击右上角"登录"按钮
2. 点击"立即注册"链接
3. 填写用户名、密码、确认密码
4. 点击"注册"按钮
5. 注册成功后返回登录

### 访问管理后台

1. 使用管理员账号登录
2. 点击侧边栏"管理"菜单
3. 切换不同标签页查看：
   - **用户管理**: 查看和删除用户
   - **内容管理**: 管理视频内容
   - **系统设置**: 配置系统参数

---

## 🔧 API 接口完整列表

### 公开接口

| 方法 | 路径 | 说明 | 认证 |
|------|------|------|------|
| GET | `/api/health` | 健康检查 | ❌ |
| GET | `/api/config` | 获取配置 | ❌ |
| GET | `/api/search?keyword=` | 搜索视频 | ❌ |
| GET | `/api/detail?id=` | 视频详情 | ❌ |
| GET | `/api/play?id=&episode=` | 播放视频 | ❌ |
| GET | `/api/tvbox` | TVBox 配置 | ❌ |

### 用户接口

| 方法 | 路径 | 说明 | 认证 |
|------|------|------|------|
| POST | `/api/login` | 用户登录 | ❌ |
| POST | `/api/register` | 用户注册 | ❌ |
| POST | `/api/logout` | 用户登出 | ✅ |
| GET | `/api/favorites` | 获取收藏 | ✅ |
| POST | `/api/favorites` | 添加收藏 | ✅ |
| DELETE | `/api/favorites` | 删除收藏 | ✅ |
| GET | `/api/playrecords` | 历史记录 | ✅ |
| POST | `/api/playrecords` | 添加记录 | ✅ |
| GET | `/api/user/preferences` | 用户偏好 | ✅ |
| POST | `/api/user/preferences` | 设置偏好 | ✅ |

### 管理员接口

| 方法 | 路径 | 说明 | 认证 | 权限 |
|------|------|------|------|------|
| GET | `/api/admin/users` | 用户列表 | ✅ | Admin |
| DELETE | `/api/admin/users/:id` | 删除用户 | ✅ | Admin |
| GET | `/api/admin/videos` | 视频列表 | ✅ | Admin |
| DELETE | `/api/admin/videos/:id` | 删除视频 | ✅ | Admin |
| GET | `/api/admin/settings` | 系统设置 | ✅ | Admin |
| POST | `/api/admin/settings` | 更新设置 | ✅ | Admin |

---

## 🎨 界面预览

### 登录/注册模态框
- 支持登录/注册模式切换
- 实时表单验证
- 错误提示友好

### 管理后台
- 标签页切换
- 数据表格展示
- 操作按钮清晰
- 响应式布局

### 响应式效果
- 手机端：紧凑布局，易于触摸
- PC 端：标准布局，信息丰富
- TV 端：放大元素，远距离可视

---

## ⚠️ 注意事项

### 安全性

1. **默认密码**: 必须修改 `admin/admin` 默认密码
2. **生产环境**: 建议启用 HTTPS
3. **数据库备份**: 定期备份 `moontv.db`
4. **会话过期**: 当前会话 30 天过期，可根据需要调整

### 性能优化

1. **静态资源压缩**: rust-embed 已启用 gzip 压缩
2. **数据库索引**: 关键表已创建索引
3. **API 缓存**: 支持 API 响应缓存（待完善）

### 已知限制

1. **移动端导航**: 当前隐藏侧边栏，建议添加汉堡菜单
2. **内容管理**: 视频管理功能尚未完全实现
3. **批量操作**: 用户管理暂不支持批量操作
4. **设置持久化**: 系统设置暂未保存到数据库

---

## 📋 后续开发计划

### 短期（v0.2.0）

- [ ] 移动端汉堡菜单
- [ ] 视频缓存管理完整实现
- [ ] 系统设置持久化到数据库
- [ ] 用户头像上传
- [ ] 密码修改功能

### 中期（v0.3.0）

- [ ] 弹幕功能
- [ ] 视频评论
- [ ] 观看进度同步
- [ ] 多数据源管理
- [ ] 视频分类标签

### 长期（v1.0.0）

- [ ] 用户等级系统
- [ ] VIP 会员制度
- [ ] 视频推荐算法
- [ ] 数据统计分析
- [ ] 多语言支持

---

## 📞 技术支持

如有问题请查看：
- 项目 README.md
- 之前的修复文档（FIX_*.md）
- Cargo.toml 中的依赖版本文档

---

**升级完成时间**: 2026-04-02  
**版本号**: v0.1.0 → v0.2.0  
**开发者**: 蔡大伟
