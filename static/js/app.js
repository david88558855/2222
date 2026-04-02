// MoonTV Frontend

const API_BASE = window.location.origin;

// State
let token = localStorage.getItem('moontv_token');
let user = JSON.parse(localStorage.getItem('moontv_user') || 'null');
let currentVideo = null;
let isRegisterMode = false;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    updateAuthUI();
    loadConfig();
    bindEvents();
    handleHashRoute();
});

function bindEvents() {
    // Search
    document.getElementById('search-btn').addEventListener('click', doSearch);
    document.getElementById('search-input').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') doSearch();
    });
    
    // Login/Register modal
    document.getElementById('login-btn').addEventListener('click', () => {
        document.getElementById('login-modal').style.display = 'flex';
        setLoginMode();
    });
    document.getElementById('login-cancel').addEventListener('click', () => {
        document.getElementById('login-modal').style.display = 'none';
    });
    document.getElementById('login-submit').addEventListener('click', handleAuthSubmit);
    
    // Toggle login/register mode
    document.getElementById('toggle-register').addEventListener('click', (e) => {
        e.preventDefault();
        toggleAuthMode();
    });
    
    // Logout
    document.getElementById('logout-btn').addEventListener('click', doLogout);
    
    // Sidebar navigation
    document.querySelectorAll('.sidebar a').forEach(link => {
        link.addEventListener('click', handleNavigation);
    });
    
    // Hash change event
    window.addEventListener('hashchange', handleHashRoute);
    
    // Keyboard shortcuts
    document.addEventListener('keypress', (e) => {
        if (e.key === 'Escape') {
            document.getElementById('login-modal').style.display = 'none';
        }
    });
}

function setLoginMode() {
    isRegisterMode = false;
    document.getElementById('modal-title').textContent = '登录';
    document.getElementById('login-confirm').style.display = 'none';
    document.getElementById('login-submit').textContent = '登录';
    document.getElementById('toggle-register-text').textContent = '还没有账号？';
    document.getElementById('toggle-register').textContent = '立即注册';
}

function setRegisterMode() {
    isRegisterMode = true;
    document.getElementById('modal-title').textContent = '注册';
    document.getElementById('login-confirm').style.display = 'block';
    document.getElementById('login-submit').textContent = '注册';
    document.getElementById('toggle-register-text').textContent = '已有账号？';
    document.getElementById('toggle-register').textContent = '立即登录';
}

function toggleAuthMode() {
    isRegisterMode = !isRegisterMode;
    if (isRegisterMode) {
        setRegisterMode();
    } else {
        setLoginMode();
    }
}

function handleAuthSubmit() {
    if (isRegisterMode) {
        doRegister();
    } else {
        doLogin();
    }
}

function updateAuthUI() {
    const loginBtn = document.getElementById('login-btn');
    const logoutBtn = document.getElementById('logout-btn');
    const userName = document.getElementById('user-name');
    
    if (token && user) {
        loginBtn.style.display = 'none';
        logoutBtn.style.display = 'block';
        userName.textContent = user.username;
    } else {
        loginBtn.style.display = 'block';
        logoutBtn.style.display = 'none';
        userName.textContent = '';
    }
}

async function loadConfig() {
    try {
        const res = await fetch(`${API_BASE}/api/config`);
        const data = await res.json();
        console.log('Config loaded:', data);
    } catch (e) {
        console.error('Failed to load config:', e);
    }
}

// Handle hash route
function handleHashRoute() {
    const hash = window.location.hash || '/';
    const sidebarLinks = document.querySelectorAll('.sidebar a');
    sidebarLinks.forEach(link => {
        const href = link.getAttribute('href');
        if (href === hash || (hash !== '/' && href === hash.split('?')[0])) {
            link.classList.add('active');
        } else {
            link.classList.remove('active');
        }
    });
    
    // Hide all content sections
    document.getElementById('video-list').style.display = 'none';
    document.getElementById('video-detail').style.display = 'none';
    document.getElementById('player').style.display = 'none';
    
    // Handle different routes
    if (hash === '/' || hash === '') {
        document.getElementById('video-list').style.display = 'block';
        loadVideos();
    } else if (hash === '#favorites') {
        document.getElementById('video-list').style.display = 'block';
        loadFavorites();
    } else if (hash === '#history') {
        document.getElementById('video-list').style.display = 'block';
        loadHistory();
    } else if (hash === '#settings') {
        showSettings();
    } else if (hash === '#admin') {
        showAdmin();
    }
}

// Handle sidebar navigation
function handleNavigation(e) {
    const href = e.currentTarget.getAttribute('href');
    window.location.hash = href;
    e.preventDefault();
}

async function loadVideos() {
    try {
        const res = await fetch(`${API_BASE}/api/home`);
        const data = await res.json();
        if (data.code === 0) {
            renderVideoList(data.data.list);
        }
    } catch (e) {
        console.error('Failed to load videos:', e);
    }
}

async function loadFavorites() {
    if (!token) {
        document.getElementById('video-list').innerHTML = '<p class="empty">请先登录</p>';
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/api/favorites`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        const data = await res.json();
        if (data.code === 0) {
            renderVideoList(data.data.list);
        } else {
            document.getElementById('video-list').innerHTML = '<p class="empty">加载失败</p>';
        }
    } catch (e) {
        console.error('Failed to load favorites:', e);
        document.getElementById('video-list').innerHTML = '<p class="empty">加载失败</p>';
    }
}

async function loadHistory() {
    if (!token) {
        document.getElementById('video-list').innerHTML = '<p class="empty">请先登录</p>';
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/api/playrecords`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        const data = await res.json();
        if (data.code === 0) {
            renderVideoList(data.data.list || []);
        } else {
            document.getElementById('video-list').innerHTML = '<p class="empty">加载失败</p>';
        }
    } catch (e) {
        console.error('Failed to load history:', e);
        document.getElementById('video-list').innerHTML = '<p class="empty">加载失败</p>';
    }
}

function showSettings() {
    if (!token) {
        document.getElementById('video-list').innerHTML = '<p class="empty">请先登录</p>';
        document.getElementById('video-list').style.display = 'block';
        return;
    }
    
    document.getElementById('video-list').innerHTML = `
        <div class="settings-panel">
            <h2>设置</h2>
            <div class="setting-item">
                <label>用户�?/label>
                <span>${user ? user.username : ''}</span>
            </div>
            <div class="setting-item">
                <label>用户 ID</label>
                <span>${user ? user.id : ''}</span>
            </div>
            <div class="setting-item">
                <label>角色</label>
                <span>${user ? (user.role === 'admin' ? '管理�? : '普通用�?) : ''}</span>
            </div>
        </div>
    `;
    document.getElementById('video-list').style.display = 'block';
}

function showAdmin() {
    if (!token || !user || user.role !== 'admin') {
        document.getElementById('video-list').innerHTML = '<p class="empty">需要管理员权限或未登录</p>';
        document.getElementById('video-list').style.display = 'block';
        return;
    }
    
    document.getElementById('video-list').innerHTML = `
        <div class="admin-panel">
            <h2>管理后台</h2>
            <div class="admin-tabs">
                <button class="admin-tab active" onclick="showAdminTab('users')">用户管理</button>
                <button class="admin-tab" onclick="showAdminTab('videos')">内容管理</button>
                <button class="admin-tab" onclick="showAdminTab('settings')">系统设置</button>
            </div>
            <div id="admin-content" class="admin-content">
                ${renderUserManagement()}
            </div>
        </div>
    `;
    document.getElementById('video-list').style.display = 'block';
    loadAdminUsers();
}

function showAdminTab(tab) {
    // Update tab active state
    document.querySelectorAll('.admin-tab').forEach(t => t.classList.remove('active'));
    event.target.classList.add('active');
    
    // Render content
    const content = document.getElementById('admin-content');
    if (tab === 'users') {
        content.innerHTML = renderUserManagement();
        loadAdminUsers();
    } else if (tab === 'videos') {
        content.innerHTML = renderVideoManagement();
    } else if (tab === 'settings') {
        content.innerHTML = renderSystemSettings();
        loadAdminSettings();
    }
}

function renderUserManagement() {
    return `
        <div class="admin-section">
            <h3>用户管理</h3>
            <div class="admin-table-wrapper">
                <table class="admin-table">
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>用户�?/th>
                            <th>角色</th>
                            <th>注册时间</th>
                            <th>操作</th>
                        </tr>
                    </thead>
                    <tbody id="admin-users-body">
                        <tr><td colspan="5" style="text-align:center">加载�?..</td></tr>
                    </tbody>
                </table>
            </div>
        </div>
    `;
}

function renderVideoManagement() {
    return `
        <div class="admin-section">
            <h3>内容管理</h3>
            <p>管理视频内容和分类（功能开发中�?/p>
            <div class="admin-table-wrapper">
                <table class="admin-table">
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>名称</th>
                            <th>来源</th>
                            <th>缓存时间</th>
                            <th>操作</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr><td colspan="5" style="text-align:center">暂无数据</td></tr>
                    </tbody>
                </table>
            </div>
        </div>
    `;
}

function renderSystemSettings() {
    return `
        <div class="admin-section">
            <h3>系统设置</h3>
            <form id="admin-settings-form" onsubmit="saveAdminSettings(event)">
                <div class="setting-item">
                    <label>网站名称</label>
                    <input type="text" id="setting-site-name" value="MoonTV" />
                </div>
                <div class="setting-item">
                    <label>允许注册</label>
                    <select id="setting-allow-register">
                        <option value="true">�?/option>
                        <option value="false">�?/option>
                    </select>
                </div>
                <div class="setting-item">
                    <label>最大搜索结果数</label>
                    <input type="number" id="setting-max-results" value="20" min="1" max="100" />
                </div>
                <div class="setting-item">
                    <button type="submit" class="btn-primary">保存设置</button>
                </div>
            </form>
        </div>
    `;
}

async function loadAdminUsers() {
    try {
        const res = await fetch(`${API_BASE}/api/admin/users`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        const data = await res.json();
        
        if (data.code === 0) {
            renderAdminUsers(data.data);
        } else {
            document.getElementById('admin-users-body').innerHTML = 
                '<tr><td colspan="5" style="text-align:center">加载失败</td></tr>';
        }
    } catch (e) {
        console.error('Failed to load users:', e);
        document.getElementById('admin-users-body').innerHTML = 
            '<tr><td colspan="5" style="text-align:center">加载失败</td></tr>';
    }
}

function renderAdminUsers(users) {
    const tbody = document.getElementById('admin-users-body');
    if (!users || users.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5" style="text-align:center">暂无用户</td></tr>';
        return;
    }
    
    tbody.innerHTML = users.map(u => `
        <tr>
            <td>${u.id}</td>
            <td>${u.username}</td>
            <td>${u.role === 'admin' ? '<span class="badge badge-admin">管理�?/span>' : '<span class="badge badge-user">普通用�?/span>'}</td>
            <td>${new Date(parseInt(u.created_at) * 1000).toLocaleString('zh-CN')}</td>
            <td>
                ${u.id !== 1 ? `<button class="btn-danger btn-sm" onclick="deleteUser(${u.id})">删除</button>` : '<span style="color:#666">不可删除</span>'}
            </td>
        </tr>
    `).join('');
}

async function deleteUser(id) {
    if (!confirm(`确定要删除用�?${id} 吗？此操作不可恢复！`)) {
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/api/admin/users/${id}`, {
            method: 'DELETE',
            headers: { 'Authorization': `Bearer ${token}` }
        });
        const data = await res.json();
        
        if (data.code === 0) {
            alert('用户已删�?);
            loadAdminUsers();
        } else {
            alert('删除失败�? + (data.message || '未知错误'));
        }
    } catch (e) {
        alert('删除失败�? + e.message);
    }
}

async function loadAdminSettings() {
    try {
        const res = await fetch(`${API_BASE}/api/admin/settings`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        const data = await res.json();
        
        if (data.code === 0) {
            document.getElementById('setting-site-name').value = data.data.site_name || 'MoonTV';
            document.getElementById('setting-allow-register').value = String(data.data.allow_register);
            document.getElementById('setting-max-results').value = data.data.max_search_results || 20;
        }
    } catch (e) {
        console.error('Failed to load settings:', e);
    }
}

async function saveAdminSettings(event) {
    event.preventDefault();
    
    const siteName = document.getElementById('setting-site-name').value;
    const allowRegister = document.getElementById('setting-allow-register').value === 'true';
    const maxResults = parseInt(document.getElementById('setting-max-results').value);
    
    try {
        const res = await fetch(`${API_BASE}/api/admin/settings`, {
            method: 'POST',
            headers: { 
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify({
                site_name: siteName,
                allow_register: allowRegister,
                max_search_results: maxResults
            })
        });
        const data = await res.json();
        
        if (data.code === 0) {
            alert('设置已保�?);
        } else {
            alert('保存失败�? + (data.message || '未知错误'));
        }
    } catch (e) {
        alert('保存失败�? + e.message);
    }
}

async function doLogin() {
    const username = document.getElementById('login-username').value.trim();
    const password = document.getElementById('login-password').value;
    
    if (!username || !password) {
        alert('请输入用户名和密�?);
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/api/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        });
        const data = await res.json();
        
        if (data.code === 0) {
            token = data.data.token;
            user = {
                id: data.data.user_id,
                username: data.data.username,
                role: data.data.role
            };
            localStorage.setItem('moontv_token', token);
            localStorage.setItem('moontv_user', JSON.stringify(user));
            updateAuthUI();
            document.getElementById('login-modal').style.display = 'none';
            // Reload current page content
            handleHashRoute();
        } else {
            alert(data.message || '登录失败');
        }
    } catch (e) {
        alert('登录错误�? + e.message);
    }
}

async function doRegister() {
    const username = document.getElementById('login-username').value.trim();
    const password = document.getElementById('login-password').value;
    const confirmPassword = document.getElementById('login-confirm').value;
    
    if (!username || !password) {
        alert('请输入用户名和密�?);
        return;
    }
    
    if (password.length < 6) {
        alert('密码长度至少�?6 �?);
        return;
    }
    
    if (password !== confirmPassword) {
        alert('两次输入的密码不一�?);
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/api/register`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password, confirm_password: confirmPassword })
        });
        const data = await res.json();
        
        if (data.code === 0) {
            alert('注册成功！请登录');
            setLoginMode();
        } else {
            alert(data.message || '注册失败');
        }
    } catch (e) {
        alert('注册错误�? + e.message);
    }
}

async function doLogout() {
    if (!token) return;
    
    try {
        await fetch(`${API_BASE}/api/logout`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ token })
        });
    } catch (e) {
        console.error('Logout error:', e);
    }
    
    token = null;
    user = null;
    localStorage.removeItem('moontv_token');
    localStorage.removeItem('moontv_user');
    updateAuthUI();
    window.location.hash = '/';
}

async function doSearch() {
    const keyword = document.getElementById('search-input').value.trim();
    if (!keyword) return;
    
    try {
        const res = await fetch(`${API_BASE}/api/search?keyword=${encodeURIComponent(keyword)}`);
        const data = await res.json();
        
        if (data.code === 0) {
            renderVideoList(data.data.list);
        } else {
            alert(data.message || '搜索失败');
        }
    } catch (e) {
        alert('搜索错误�? + e.message);
    }
}

function renderVideoList(videos) {
    const container = document.getElementById('video-list');
    
    if (!videos || videos.length === 0) {
        container.innerHTML = '<p class="empty">没有找到结果</p>';
        return;
    }
    
    container.innerHTML = videos.map(video => `
        <div class="video-card" onclick="showDetail('${video.id}', '${video.source_site || ''}')">
            <img src="${video.pic || '/static/img/placeholder.png'}" alt="${video.name}">
            <div class="info">
                <div class="title">${video.name}</div>
            </div>
        </div>
    `).join('');
}

async function showDetail(id, site) {
    try {
        const url = `${API_BASE}/api/detail?id=${id}${site ? '&site=' + site : ''}`;
        const res = await fetch(url);
        const data = await res.json();
        
        if (data.code === 0) {
            renderVideoDetail(data.data);
        }
    } catch (e) {
        console.error('Failed to load detail:', e);
    }
}

function renderVideoDetail(detail) {
    const container = document.getElementById('video-detail');
    container.style.display = 'block';
    
    container.innerHTML = `
        <div class="detail-header">
            <img src="${detail.pic}" alt="${detail.name}">
            <div class="detail-info">
                <h2>${detail.name}</h2>
                <p>${detail.detail || ''}</p>
                <div class="episodes">
                    ${detail.episodes.map((ep, i) => `
                        <button onclick="playVideo('${detail.id}', ${i}, '${detail.source_site || ''}')">${ep.name}</button>
                    `).join('')}
                </div>
            </div>
        </div>
    `;
}

async function playVideo(id, episode, site) {
    try {
        const url = `${API_BASE}/api/play?id=${id}&episode=${episode}${site ? '&site=' + site : ''}`;
        const res = await fetch(url);
        const data = await res.json();
        
        if (data.code === 0) {
            const player = document.getElementById('player');
            const video = document.getElementById('video-player');
            player.style.display = 'block';
            video.src = data.data.play_url;
            video.play();
            
            // Save play record
            if (token) {
                try {
                    await fetch(`${API_BASE}/api/playrecords`, {
                        method: 'POST',
                        headers: { 
                            'Content-Type': 'application/json',
                            'Authorization': `Bearer ${token}`
                        },
                        body: JSON.stringify({
                            video_id: id,
                            video_name: detail?.name || '',
                            episode_index: episode,
                            source_site: site
                        })
                    });
                } catch (e) {
                    console.error('Failed to save play record:', e);
                }
            }
        }
    } catch (e) {
        console.error('Failed to play:', e);
    }
}

// Video player controls
document.getElementById('add-fav').addEventListener('click', async () => {
    if (!token || !currentVideo) return;
    // TODO: Add to favorites via API
});

// Expose functions globally for onclick handlers
window.showDetail = showDetail;
window.playVideo = playVideo;
window.showAdminTab = showAdminTab;
window.deleteUser = deleteUser;
window.saveAdminSettings = saveAdminSettings;
