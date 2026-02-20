pub const DATA: &str = r#"
* {
user-select: none;
-webkit-user-select: none;
cursor: default;
}

:root {
--bg: #0f1014;
--surface: #1b1c21;
--surface-hover: #25262c;
--primary: #4f46e5;
--text: #e2e8f0;
--text-muted: #94a3b8;
--border: #2d3748;
--danger: #ef4444;

--radius: 12px;
--container-width: 600px;
}

* { box-sizing: border-box; -webkit-tap-highlight-color: transparent; }

body {
margin: 0;
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
background-color: var(--bg);
color: var(--text);
height: 100vh;
display: flex;
justify-content: center;
align-items: center;
overflow: hidden;
}

.container {
width: 100%;
max-width: var(--container-width);
padding: 20px;
display: flex;
flex-direction: column;
gap: 30px;
z-index: 1;
}

.app-header { text-align: center; }
h1 {
font-family: 'GNF', sans-serif;
font-size: 3rem;
margin: 0;
background: linear-gradient(to right, #fff, #94a3b8);
-webkit-background-clip: text;
}
.subtitle {
margin: 8px 0 0;
color: var(--text-muted);
font-size: 0.9rem;
text-transform: uppercase;
letter-spacing: 2px;
}

.grid-menu {
display: grid;
grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
gap: 15px;
}

.card-btn {
background: var(--surface);
border: 1px solid var(--border);
border-radius: var(--radius);
padding: 20px;
display: flex;
align-items: center;
gap: 15px;
cursor: pointer;
transition: all 0.2s cubic-bezier(0.2, 0, 0, 1);
text-align: left;
color: var(--text);
position: relative;
overflow: hidden;
}

.card-btn:hover {
background: var(--surface-hover);
border-color: var(--primary);
transform: translateY(-2px);
box-shadow: 0 4px 20px rgba(0,0,0,0.3);
}

.card-btn:active { transform: scale(0.98); }

.btn-icon { font-size: 1.5rem; }
.btn-content { display: flex; flex-direction: column; }
.btn-title { font-weight: 600; font-size: 1rem; }
.btn-desc { font-size: 0.8rem; color: var(--text-muted); margin-top: 2px; }

.icon-btn {
background: none;
border: none;
color: var(--text-muted);
cursor: pointer;
padding: 10px;
border-radius: 50%;
transition: color 0.2s;
display: flex;
align-items: center;
justify-content: center;
}

.icon-btn:hover {
color: var(--text);
background: rgba(255,255,255,0.05);
}

.icon-btn:hover svg {
animation: spin 2s ease-in-out;
}


@keyframes spin {
0% { transform: rotate(0deg); }
100% { transform: rotate(360deg); }
}

.settings-trigger {
position: fixed;
top: 20px;
right: 20px;
z-index: 100;
background: var(--surface);
border: 1px solid var(--border);
}

.settings-trigger {
position: fixed;
top: 20px;
right: 20px;
z-index: 100;
background: var(--surface);
border: 1px solid var(--border);
}

.overlay {
position: fixed;
inset: 0;
background: rgba(0,0,0,0.5);
opacity: 0;
pointer-events: none;
transition: opacity 0.3s;
z-index: 998;
backdrop-filter: blur(2px);
}

.settings-drawer {
position: fixed;
top: 0;
right: 0;
bottom: 0;
width: 320px;
background: var(--bg);
border-left: 1px solid var(--border);
transform: translateX(100%);
transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
z-index: 999;
display: flex;
flex-direction: column;
padding: 25px;
box-shadow: -10px 0 30px rgba(0,0,0,0.5);
}

body.settings-open .overlay { opacity: 1; pointer-events: auto; }
body.settings-open .settings-drawer { transform: translateX(0); }

.drawer-header {
display: flex;
justify-content: space-between;
align-items: center;
margin-bottom: 30px;
}
.drawer-header h2 { margin: 0; font-size: 1.5rem; }

.setting-group { margin-bottom: 25px; }
.setting-group label {
display: block;
color: var(--text-muted);
font-size: 0.85rem;
text-transform: uppercase;
letter-spacing: 1px;
margin-bottom: 10px;
}

.theme-grid { display: flex; flex-wrap: wrap; gap: 8px; }
.theme-chip {
background: var(--surface);
border: 1px solid var(--border);
color: var(--text);
padding: 6px 12px;
border-radius: 20px;
font-size: 0.9rem;
cursor: pointer;
transition: 0.2s;
}
.theme-chip:hover { border-color: var(--text-muted); }
.theme-chip.active { background: var(--primary); border-color: var(--primary); color: white; }

.styled-select {
width: 100%;
background: var(--surface);
color: var(--text);
border: 1px solid var(--border);
padding: 10px;
border-radius: 8px;
font-size: 1rem;
outline: none;
appearance: none;
background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%2394a3b8%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E");
background-repeat: no-repeat;
background-position: right 12px top 50%;
background-size: 10px auto;
}
.styled-select:focus { border-color: var(--primary); }

.setting-info {
margin-top: auto;
font-size: 0.8rem;
color: var(--text-muted);
padding-top: 15px;
}
.path-hint { display: block; margin-top: 5px; opacity: 0.5; font-family: monospace; }


.developer-links {
display: flex;
flex-direction: column;
gap: 10px;
margin-top: 10px;
}

.dev-link {
display: flex;
align-items: center;
gap: 12px;
padding: 12px 16px;
background: var(--surface);
border: 1px solid var(--border);
border-radius: var(--radius);
color: var(--text);
text-decoration: none;
font-size: 0.95rem;
transition: all 0.2s ease;
}

.dev-link:hover {
background: var(--surface-hover);
border-color: var(--primary);
transform: translateX(4px);
}

.dev-icon {
font-size: 1.2rem;
filter: grayscale(1);
transition: filter 0.2s ease;
}

.dev-link:hover .dev-icon {
filter: grayscale(0);
}

.toast {
position: fixed;
bottom: 20px;
left: 50%;
transform: translateX(-50%) translateY(20px);
background: var(--surface);
border: 1px solid var(--primary);
padding: 10px 20px;
border-radius: 50px;
opacity: 0;
transition: 0.3s;
font-size: 0.9rem;
pointer-events: none;
z-index: 2000;
}
.toast.visible { transform: translateX(-50%) translateY(0); opacity: 1; }
.toast.error { border-color: var(--danger); color: var(--danger); }


.tweak-card input[type="checkbox"] {
position: absolute;
opacity: 0;
width: 0;
height: 0;
margin: 0;
pointer-events: none;
}

.tweaker-modal {
display: none;
position: fixed;
top: 50%;
left: 50%;
transform: translate(-50%, -50%);
background-color: var(--surface);
border: 1px solid var(--border);
border-radius: var(--radius);
padding: 25px;
z-index: 2000;
width: 90%;
max-width: 400px;
box-shadow: 0 20px 40px rgba(0,0,0,0.5);
}

.tweaker-modal.active {
display: block;
animation: modalFadeIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes modalFadeIn {
from { opacity: 0; transform: translate(-50%, -45%) scale(0.95); }
to { opacity: 1; transform: translate(-50%, -50%) scale(1); }
}

.tweak-list {
display: flex;
flex-direction: column;
gap: 12px;
margin-top: 20px;
}

.tweak-card {
display: flex;
align-items: center;
justify-content: space-between;
padding: 14px 18px;
background: rgba(255, 255, 255, 0.03);
border: 1px solid var(--border);
border-radius: 10px;
cursor: pointer;
transition: all 0.2s ease;
user-select: none;
}

.tweak-card:hover {
background: var(--surface-hover);
border-color: var(--primary);
transform: translateY(4px);
}

.tweak-card:active { transform: scale(0.98); }

.tweak-title {
font-size: 0.95rem;
font-weight: 500;
color: var(--text);
}

.switch-ui {
width: 40px;
height: 20px;
background: #333;
border-radius: 20px;
position: relative;
transition: background-color 0.3s ease;
}

.tweak-card input:checked + .switch-ui {
background: var(--primary);
}

.switch-ui::after {
content: '';
position: absolute;
top: 2px;
left: 2px;
width: 16px;
height: 16px;
background: white;
border-radius: 50%;
transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.tweak-card input:checked + .switch-ui::after {
transform: translateX(20px);
}

.tweaker-footer {
display: flex;
justify-content: center;
width: 100%;
margin-top: 15px;
padding-bottom: 15px;
}

.mini-simple-btn {
display: flex;
align-items: center;
gap: 8px;
padding: 8px 16px;
background: var(--surface);
border: 1px solid var(--border);
border-radius: 20px;
color: var(--text-muted);
cursor: pointer;
transition: all 0.2s ease;
box-shadow: none;
}

.mini-simple-btn:hover {
border-color: var(--primary);
color: var(--text);
background: var(--surface-hover);
}

.mini-simple-btn:active {
transform: scale(0.98);
}

@media (max-width: 480px) {
.container { padding: 15px; }
h1 { font-size: 2rem; }
.grid-menu { grid-template-columns: 1fr; }

.settings-drawer { width: 100%; max-width: none; }
.close-btn { font-size: 1.5rem; padding: 15px; }
}

@media (max-width: 360px) {
.developer-links {
flex-direction: row;
flex-wrap: wrap;
}

.dev-link {
flex: 1;
justify-content: center;
padding: 10px;
font-size: 0.8rem;
}
}
"#;
