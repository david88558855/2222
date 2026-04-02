// MoonTV Frontend

const API_BASE = window.location.origin;

// State
let token = localStorage.getItem('moontv_token');
let user = JSON.parse(localStorage.getItem('moontv_user') || 'null');
let currentVideo = null;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    updateAuthUI();
    loadConfig();
    bindEvents();
});

function bindEvents() {
    // Search
    document.getElementById('search-btn').addEventListener('click', doSearch);
    document.getElementById('search-input').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') doSearch();
    });
    
    // Login
    document.getElementById('login-btn').addEventListener('click', () => {
        document.getElementById('login-modal').style.display = 'flex';
    });
    document.getElementById('login-cancel').addEventListener('click', () => {
        document.getElementById('login-modal').style.display = 'none';
    });
    document.getElementById('login-submit').addEventListener('click', doLogin);
    
    // Logout
    document.getElementById('logout-btn').addEventListener('click', doLogout);
    
    // Keyboard shortcuts
    document.addEventListener('keypress', (e) => {
        if (e.key === 'Escape') {
            document.getElementById('login-modal').style.display = 'none';
        }
    });
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

async function doLogin() {
    const username = document.getElementById('login-username').value;
    const password = document.getElementById('login-password').value;
    
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
        } else {
            alert(data.message || 'Login failed');
        }
    } catch (e) {
        alert('Login error: ' + e.message);
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
            alert(data.message || 'Search failed');
        }
    } catch (e) {
        alert('Search error: ' + e.message);
    }
}

function renderVideoList(videos) {
    const container = document.getElementById('video-list');
    
    if (!videos || videos.length === 0) {
        container.innerHTML = '<p class="empty">No results found</p>';
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
                <p>${detail.detail}</p>
                <div class="episodes">
                    ${detail.episodes.map((ep, i) => `
                        <button onclick="playVideo('${detail.id}', ${i}, '${detail.source_site}')">${ep.name}</button>
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