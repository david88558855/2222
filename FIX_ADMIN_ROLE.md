# 管理员权限修复

## 问题描述

使用默认账号 `admin/admin` 登录后，点击"管理"菜单显示"需要管理员权限"，无法进入管理后台。

## 根本原因

**类型不匹配问题**：

1. **后端返回的角色类型**：字符串 `"admin"`
   ```rust
   // src/api/auth.rs
   let response = LoginResponse {
       token,
       user_id: 1,
       username: req.username,
       role: "admin".to_string(),  // ← 字符串类型
   };
   ```

2. **前端检查的角色类型**：数字 `1`
   ```javascript
   // static/js/app.js
   if (!token || !user || user.role !== 1) {  // ← 数字类型
       // 显示"需要管理员权限"
   }
   ```

3. **类型比较失败**：`"admin" !== 1` 永远为 `true`，导致权限检查始终失败

## 解决方案

将前端的角色判断从数字比较改为字符串比较：

### 修改文件：`static/js/app.js`

**1. showSettings 函数 - 角色显示**
```javascript
// 修改前
<span>${user ? (user.role === 1 ? '管理员' : '普通用户') : ''}</span>

// 修改后
<span>${user ? (user.role === 'admin' ? '管理员' : '普通用户') : ''}</span>
```

**2. showAdmin 函数 - 权限检查**
```javascript
// 修改前
if (!token || !user || user.role !== 1) {

// 修改后
if (!token || !user || user.role !== 'admin') {
```

## 验证方法

1. 使用 `admin/admin` 登录
2. 点击侧边栏"设置"菜单
   - ✅ 应显示"角色：管理员"
3. 点击侧边栏"管理"菜单
   - ✅ 应显示管理后台界面（用户管理、内容管理、系统设置）

## 扩展建议

如果将来需要支持多级别角色，建议使用统一的字符串枚举：

```rust
// 后端定义角色枚举
pub enum Role {
    Admin,      // "admin"
    User,       // "user"
    Moderator,  // "moderator"
}
```

```javascript
// 前端统一使用字符串比较
const isAdmin = user.role === 'admin';
const isModerator = user.role === 'moderator';
```

## 相关文件

- `src/api/auth.rs` - 登录接口，返回用户角色
- `static/js/app.js` - 前端导航和权限控制逻辑
