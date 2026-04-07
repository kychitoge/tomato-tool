/* ===== Tomato Novel Downloader – WebUI ===== */

let loginPromise = null;
let isDockerBuild = false;

function fetchWithCreds(url, opts) {
  return fetch(url, { credentials: 'same-origin', ...(opts || {}) });
}

// ── Theme ──────────────────────────────────────────────────────────

const THEME_KEY = 'tnd.theme';

function getStoredTheme() {
  try { return localStorage.getItem(THEME_KEY); } catch { return null; }
}

function applyTheme(theme) {
  if (theme === 'light' || theme === 'dark') {
    document.documentElement.setAttribute('data-theme', theme);
  } else {
    document.documentElement.removeAttribute('data-theme');
  }
  updateThemeButton(theme);
}

function updateThemeButton(theme) {
  const icon = document.getElementById('themeIcon');
  const label = document.getElementById('themeLabel');
  if (!icon) return;

  const isDark = theme === 'dark' ||
    (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches);

  if (isDark) {
    icon.innerHTML = '<circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>';
    if (label) label.textContent = 'Chế độ sáng';
  } else {
    icon.innerHTML = '<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>';
    if (label) label.textContent = 'Chế độ tối';
  }
}

function toggleTheme() {
  const current = document.documentElement.getAttribute('data-theme');
  let next;
  if (current === 'dark') {
    next = 'light';
  } else if (current === 'light') {
    next = 'dark';
  } else {
    // auto → opposite of system
    next = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'light' : 'dark';
  }
  try { localStorage.setItem(THEME_KEY, next); } catch {}
  applyTheme(next);
}

// Apply stored theme immediately
(function() {
  const stored = getStoredTheme();
  if (stored) applyTheme(stored);
})();

// ── Auth ───────────────────────────────────────────────────────────

function showLogin(show) {
  const modal = document.getElementById('loginModal');
  if (!modal) return;
  modal.classList.toggle('hidden', !show);
  document.body.style.overflow = show ? 'hidden' : '';
  if (show) {
    const inp = document.getElementById('loginPassword');
    if (inp) inp.focus();
  }
}

async function requireLogin() {
  if (loginPromise) return loginPromise;

  showLogin(true);
  const msg = document.getElementById('loginMsg');
  if (msg) msg.textContent = '';

  loginPromise = new Promise((resolve, reject) => {
    const form = document.getElementById('loginForm');
    if (!form) { reject(new Error('login form missing')); return; }

    const handler = async (e) => {
      e.preventDefault();
      const pw = (document.getElementById('loginPassword')?.value || '').toString();
      try {
        const res = await fetchWithCreds('/api/login', {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          body: JSON.stringify({ password: pw })
        });
        if (!res.ok) { if (msg) msg.textContent = 'Sai mật khẩu'; return; }
        showLogin(false);
        form.removeEventListener('submit', handler);
        resolve(true);
      } catch (err) {
        if (msg) msg.textContent = String(err || 'login failed');
      }
    };
    form.addEventListener('submit', handler);
  }).finally(() => { loginPromise = null; });

  return loginPromise;
}

// ── HTTP Helper ────────────────────────────────────────────────────

async function j(url, opts) {
  const res = await fetchWithCreds(url, opts);
  if (res.status === 401) {
    await requireLogin();
    const res2 = await fetchWithCreds(url, opts);
    if (!res2.ok) {
      const text = await res2.text().catch(() => '');
      throw new Error(`${res2.status} ${res2.statusText}${text ? `: ${text}` : ''}`);
    }
    const ct2 = res2.headers.get('content-type') || '';
    return ct2.includes('application/json') ? res2.json() : res2.text();
  }
  if (!res.ok) {
    const text = await res.text().catch(() => '');
    throw new Error(`${res.status} ${res.statusText}${text ? `: ${text}` : ''}`);
  }
  const ct = res.headers.get('content-type') || '';
  return ct.includes('application/json') ? res.json() : res.text();
}

// ── Utilities ──────────────────────────────────────────────────────

function esc(s) {
  return (s ?? '').toString().replace(/[&<>"']/g, c =>
    ({ '&':'&amp;', '<':'&lt;', '>':'&gt;', '"':'&quot;', "'":'&#39;' }[c]));
}

function fmtBytes(n) {
  const x = Number(n || 0);
  if (!isFinite(x) || x <= 0) return '0 B';
  const k = 1024;
  const sizes = ['B','KB','MB','GB','TB'];
  const i = Math.floor(Math.log(x) / Math.log(k));
  return (x / Math.pow(k, i)).toFixed(i === 0 ? 0 : 1) + ' ' + sizes[i];
}

function fmtTime(ms) {
  const x = Number(ms || 0);
  if (!isFinite(x) || x <= 0) return '';
  return new Date(x).toLocaleString();
}

function encodePathSegments(path) {
  return (path || '').toString().split('/').map(seg => encodeURIComponent(seg)).join('/');
}

function parseBookId(input) {
  const trimmed = (input ?? '').toString().trim();
  if (!trimmed) return '';
  if (/^[0-9]+$/.test(trimmed)) return trimmed;

  const urlMatch = trimmed.match(/https?:\/\/\S+/i);
  const target = urlMatch ? urlMatch[0] : trimmed;

  const qs = target.match(/(?:^|[?&#])(?:book_id|bookId)=([0-9]+)/i);
  if (qs && qs[1]) return qs[1];

  const page = target.match(/\/page\/([0-9]+)/i);
  if (page && page[1]) return page[1];

  // Short link (e.g. https://changdunovel.com/t/XXXXX/) – return the URL so
  // the server can follow the redirect and extract the book ID.
  // Restrict to known share-link hosts to prevent forwarding arbitrary URLs.
  const allowedShortLinkHosts = new Set(['changdunovel.com', 'www.changdunovel.com', 'fanqienovel.com', 'www.fanqienovel.com', 'fqnovel.com', 'www.fqnovel.com']);
  try {
    const parsed = new URL(target);
    if (
      (parsed.protocol === 'http:' || parsed.protocol === 'https:') &&
      allowedShortLinkHosts.has(parsed.hostname.toLowerCase()) &&
      /^\/t\/[A-Za-z0-9]+\/?$/.test(parsed.pathname)
    ) {
      return target;
    }
  } catch (_) {
    // Not a valid absolute URL; ignore and fall through.
  }

  return '';
}

function isLikelyHeicUrl(url) {
  const s = (url || '').toString().toLowerCase();
  if (!s) return false;
  return /[\/.](heic|heif)(?:$|[?#])/i.test(s) || s.includes('format=heic') || s.includes('mime=image/heic');
}

function buildCoverCandidates(preview) {
  const list = [];
  const add = (u) => {
    const v = (u || '').toString().trim();
    if (!v) return;
    if (!(v.startsWith('http://') || v.startsWith('https://') || v.startsWith('/'))) return;
    if (!list.includes(v)) list.push(v);
  };

  add(preview?.detail_cover_url);
  add(preview?.cover_url);

  const nonHeic = list.filter(u => !isLikelyHeicUrl(u));
  const heic = list.filter(isLikelyHeicUrl);
  return [...nonHeic, ...heic];
}

// ── App Update ─────────────────────────────────────────────────────

const DISMISS_KEY = 'tnd.dismissed_release_tag';
let selfUpdatePollTimer = null;
let selfUpdateWasRunning = false;
let selfUpdateRestartWaiting = false;

function getDismissedTag() {
  try { return (localStorage.getItem(DISMISS_KEY) || '').toString(); } catch { return ''; }
}
function setDismissedTag(tag) {
  try { localStorage.setItem(DISMISS_KEY, (tag || '').toString()); } catch {}
}

function showAppUpdateBanner(show) {
  const el = document.getElementById('appUpdateBanner');
  if (el) el.classList.toggle('hidden', !show);
}

function renderSelfUpdateStatus(status) {
  const wrap = document.getElementById('selfUpdateProgressWrap');
  const stage = document.getElementById('selfUpdateStage');
  const msg = document.getElementById('selfUpdateMessage');
  const bar = document.getElementById('selfUpdateProgressBar');
  const pct = document.getElementById('selfUpdatePercent');
  if (!wrap || !stage || !msg || !bar || !pct) return;

  const st = (status?.state || 'idle').toString();
  const percent = Number(status?.percent || 0);
  const show = st !== 'idle';
  wrap.classList.toggle('hidden', !show);
  if (!show) return;

  stage.textContent = (status?.stage || 'idle').toString();
  msg.textContent = (status?.message || '').toString();
  bar.style.width = `${Math.max(0, Math.min(100, percent))}%`;
  pct.textContent = `${Math.max(0, Math.min(100, percent))}%`;
}

async function pollSelfUpdateStatus() {
  try {
    const status = await j('/api/self_update');
    renderSelfUpdateStatus(status);

    const st = (status?.state || '').toString();
    const stage = (status?.stage || '').toString();
    if (st === 'running') {
      selfUpdateWasRunning = true;
      selfUpdateRestartWaiting = false;
      if (!selfUpdatePollTimer) {
        selfUpdatePollTimer = setInterval(() => {
          pollSelfUpdateStatus().catch(() => {});
        }, 1000);
      }
    } else {
      if (selfUpdatePollTimer) {
        clearInterval(selfUpdatePollTimer);
        selfUpdatePollTimer = null;
      }
      // Detect restart scenarios
      if (selfUpdateWasRunning && !selfUpdateRestartWaiting) {
        if (st === 'done' && stage === 'restart') {
          // finish_done("restart") was called before exit — wait for new process
          startWaitingForRestart();
        } else if (st === 'idle') {
          // New process started with fresh state — page needs reload
          window.location.reload();
        }
      }
    }
  } catch {
    // Network error — if update was in progress, server likely restarted
    if (selfUpdateWasRunning && !selfUpdateRestartWaiting) {
      startWaitingForRestart();
    }
  }
}

function startWaitingForRestart() {
  selfUpdateRestartWaiting = true;
  if (selfUpdatePollTimer) {
    clearInterval(selfUpdatePollTimer);
    selfUpdatePollTimer = null;
  }

  // Push progress bar to 100%
  const wrap = document.getElementById('selfUpdateProgressWrap');
  const bar = document.getElementById('selfUpdateProgressBar');
  const pct = document.getElementById('selfUpdatePercent');
  const stageEl = document.getElementById('selfUpdateStage');
  const msgEl = document.getElementById('selfUpdateMessage');
  if (wrap) wrap.classList.remove('hidden');
  if (bar) bar.style.width = '100%';
  if (pct) pct.textContent = '100%';
  if (stageEl) stageEl.textContent = 'restart';
  if (msgEl) msgEl.textContent = 'Đang khởi động lại dịch vụ, chờ kết nối lại…';

  const hint = document.getElementById('appUpdateHint');
  if (hint) hint.textContent = 'Đã cập nhật xong, đang chờ dịch vụ khởi động lại…';

  // Poll /api/status every 2 s; reload once the new process responds
  const reconnTimer = setInterval(async () => {
    try {
      await fetchWithCreds('/api/status');
      clearInterval(reconnTimer);
      if (msgEl) msgEl.textContent = 'Dịch vụ đã khởi động lại, đang làm mới trang…';
      if (hint) hint.textContent = 'Đã cập nhật xong, đang làm mới…';
      setTimeout(() => window.location.reload(), 600);
    } catch {
      // still offline, keep waiting
    }
  }, 2000);
}

function applyDockerUpdateUi() {
  if (!isDockerBuild) return;
  const hint = document.getElementById('appUpdateHint');
  if (hint) hint.textContent = 'Bản Docker đã tắt tự cập nhật, hãy cập nhật bằng cách kéo lại image mới.';
  showAppUpdateBanner(false);
  const btn = document.getElementById('appUpdateCheck');
  if (btn) btn.disabled = true;
  const selfBtn = document.getElementById('appSelfUpdate');
  if (selfBtn) selfBtn.disabled = true;
  const dismissBtn = document.getElementById('appUpdateDismiss');
  if (dismissBtn) dismissBtn.disabled = true;
}

async function refreshAppUpdate(manual) {
  const hint = document.getElementById('appUpdateHint');
  const latestEl = document.getElementById('appUpdateLatest');
  const bodyEl = document.getElementById('appUpdateBody');
  const linkEl = document.getElementById('appUpdateLink');

  if (isDockerBuild) {
    applyDockerUpdateUi();
    if (latestEl) latestEl.textContent = '';
    if (bodyEl) bodyEl.textContent = 'Bản Docker đã tắt tự cập nhật, hãy cập nhật bằng cách kéo lại image mới.';
    if (linkEl) linkEl.style.pointerEvents = 'none';
    return { latestTag: '', hasUpdate: false, dockerBuild: true };
  }

  if (hint) hint.textContent = manual ? 'Đang kiểm tra…' : '';

  const data = await j('/api/app_update');
  const latestTag = (data.latest_tag || '').toString();
  const latestBody = (data.latest_body || '').toString();
  const latestUrl = (data.latest_url || '').toString();
  const hasUpdate = !!data.has_update;

  if (latestEl) latestEl.textContent = latestTag || '';
  if (bodyEl) bodyEl.textContent = latestBody || '';
  if (linkEl) {
    linkEl.href = latestUrl || '#';
    linkEl.style.pointerEvents = latestUrl ? '' : 'none';
  }

  const dismissed = getDismissedTag();
  const shouldShow = hasUpdate && latestTag && dismissed !== latestTag;

  if (shouldShow) {
    showAppUpdateBanner(true);
    if (hint) hint.textContent = 'Phát hiện bản mới';
  } else {
    showAppUpdateBanner(false);
    if (manual) {
      if (!hasUpdate) {
        if (hint) hint.textContent = 'Đã là bản mới nhất';
      } else if (dismissed === latestTag) {
        if (hint) hint.textContent = 'Đã bỏ qua thông báo bản này';
      }
    }
  }
  return { latestTag, hasUpdate };
}

// ── Status ─────────────────────────────────────────────────────────

let libraryPath = '';
let pendingBookNameJobId = null;
let pendingBookNameOptions = [];

async function refreshStatus() {
  const data = await j('/api/status');
  document.getElementById('version').textContent = data.version || '';
  document.getElementById('prewarm').textContent = data.prewarm_in_progress ? 'đang khởi động' : 'sẵn sàng';
  document.getElementById('saveDir').textContent = data.save_dir || '';
  document.getElementById('bind').textContent = data.bind_addr || '';
  document.getElementById('locked').textContent = data.locked ? 'đã khoá' : 'mở khoá';
  isDockerBuild = !!data.docker_build;
  applyDockerUpdateUi();
}

// ── Config ─────────────────────────────────────────────────────────

async function refreshConfig() {
  const data = await j('/api/config');
  const nf = document.getElementById('cfgNovelFormat');
  const bf = document.getElementById('cfgBulkFiles');
  const ea = document.getElementById('cfgEnableAudiobook');
  const af = document.getElementById('cfgAudiobookFormat');
  if (nf) nf.value = (data.novel_format || 'txt').toString();
  if (bf) bf.checked = !!data.bulk_files;
  if (ea) ea.checked = !!data.enable_audiobook;
  if (af) af.value = (data.audiobook_format || 'mp3').toString();
}

async function saveConfig() {
  const nf = document.getElementById('cfgNovelFormat')?.value;
  const bf = !!document.getElementById('cfgBulkFiles')?.checked;
  const ea = !!document.getElementById('cfgEnableAudiobook')?.checked;
  const af = document.getElementById('cfgAudiobookFormat')?.value;

  await j('/api/config', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ novel_format: nf, bulk_files: bf, enable_audiobook: ea, audiobook_format: af })
  });
}

async function refreshRawConfig() {
  const data = await j('/api/config/raw');
  const ta = document.getElementById('cfgRaw');
  const msg = document.getElementById('cfgRawMsg');
  if (ta) ta.value = (data.yaml || '').toString();
    if (msg) msg.textContent = data.generated ? 'Đã tạo cấu hình mặc định (không tìm thấy tệp cấu hình)' : '';
}

async function saveRawConfig() {
  const ta = document.getElementById('cfgRaw');
  const yaml = (ta?.value || '').toString();
  await j('/api/config/raw', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ yaml })
  });
}

// ── Full Config ────────────────────────────────────────────────────

let currentFullConfig = null;

const FULL_CONFIG_SCHEMA = [
  {
    title: 'Cơ bản và định dạng',
    fields: [
      { key: 'save_path', label: 'Đường dẫn lưu', type: 'text' },
      { key: 'novel_format', label: 'Định dạng sách', type: 'select', options: [
        { value: 'txt', label: 'txt' }, { value: 'epub', label: 'epub' }
      ] },
      { key: 'first_line_indent_em', label: 'Thụt dòng đầu (em)', type: 'number', parse: 'float', step: '0.1', min: '0' },
      { key: 'bulk_files', label: 'Lưu file rời', type: 'bool' },
      { key: 'auto_clear_dump', label: 'Tự dọn cache', type: 'bool' },
      { key: 'auto_open_downloaded_files', label: 'Tự mở file sau khi tải', type: 'bool' },
      { key: 'allow_overwrite_files', label: 'Cho phép ghi đè file đã tồn tại', type: 'bool' },
      { key: 'preferred_book_name_field', label: 'Trường tên sách ưu tiên', type: 'select', options: [
        { value: 'book_name', label: 'Tên mặc định' },
        { value: 'original_book_name', label: 'Tên gốc' },
        { value: 'book_short_name', label: 'Tên rút gọn' },
        { value: 'ask_after_download', label: 'Hỏi sau khi tải xong' }
      ] },
      { key: 'old_cli', label: 'Giao diện CLI cũ', type: 'bool' },
    ]
  },
  {
    title: 'Mạng và điều phối',
    fields: [
      { key: 'max_workers', label: 'Số luồng tối đa', type: 'number', parse: 'int', min: '1' },
      { key: 'request_timeout', label: 'Timeout request (s)', type: 'number', parse: 'int', min: '1' },
      { key: 'max_retries', label: 'Số lần thử lại tối đa', type: 'number', parse: 'int', min: '0' },
      { key: 'min_connect_timeout', label: 'Timeout kết nối tối thiểu (s)', type: 'number', parse: 'float', step: '0.1', min: '0' },
      { key: 'min_wait_time', label: 'Thời gian chờ tối thiểu (ms)', type: 'number', parse: 'int', min: '0' },
      { key: 'max_wait_time', label: 'Thời gian chờ tối đa (ms)', type: 'number', parse: 'int', min: '0' },
    ]
  },
  {
    title: 'API',
    fields: [
      { key: 'use_official_api', label: 'Dùng API chính thức', type: 'bool' },
      { key: 'api_endpoints', label: 'Danh sách API', type: 'list', placeholder: 'Mỗi dòng một mục hoặc phân tách bằng dấu phẩy' },
    ]
  },
  {
    title: 'Bình luận theo đoạn',
    fields: [
      { key: 'enable_segment_comments', label: 'Bật bình luận theo đoạn', type: 'bool' },
      { key: 'segment_comments_top_n', label: 'Số bình luận tối đa mỗi đoạn', type: 'number', parse: 'int', min: '1' },
      { key: 'segment_comments_workers', label: 'Số luồng bình luận theo đoạn', type: 'number', parse: 'int', min: '1' },
    ]
  },
  {
    title: 'Tải media',
    fields: [
      { key: 'download_comment_images', label: 'Tải ảnh bình luận', type: 'bool' },
      { key: 'download_comment_avatars', label: 'Tải avatar bình luận', type: 'bool' },
      { key: 'media_download_workers', label: 'Số luồng tải media', type: 'number', parse: 'int', min: '1' },
      { key: 'blocked_media_domains', label: 'Tên miền ảnh bị chặn', type: 'list', placeholder: 'Mỗi dòng một tên miền' },
      { key: 'force_convert_images_to_jpeg', label: 'Ép chuyển sang JPEG', type: 'bool' },
      { key: 'jpeg_retry_convert', label: 'Thử lại rồi chuyển JPEG khi lỗi', type: 'bool' },
      { key: 'jpeg_quality', label: 'Chất lượng JPEG (0-100)', type: 'number', parse: 'int', min: '0', max: '100' },
      { key: 'convert_heic_to_jpeg', label: 'Chuyển HEIC sang JPEG', type: 'bool' },
      { key: 'keep_heic_original', label: 'Giữ nguyên ảnh HEIC gốc', type: 'bool' },
      { key: 'media_limit_per_chapter', label: 'Giới hạn media mỗi chương', type: 'number', parse: 'int', min: '0' },
      { key: 'media_max_dimension_px', label: 'Kích thước media tối đa (px)', type: 'number', parse: 'int', min: '0' },
    ]
  },
  {
    title: 'Sách nói',
    fields: [
      { key: 'enable_audiobook', label: 'Bật sách nói', type: 'bool' },
      { key: 'audiobook_voice', label: 'Giọng đọc', type: 'voice' },
      { key: 'audiobook_tts_provider', label: 'Loại dịch vụ TTS', type: 'select', options: [
        { value: 'edge', label: 'edge' }, { value: 'third_party', label: 'third_party' }
      ] },
      { key: 'audiobook_tts_api_url', label: 'Địa chỉ API TTS bên thứ ba', type: 'text' },
      { key: 'audiobook_tts_api_token', label: 'Token TTS bên thứ ba', type: 'text' },
      { key: 'audiobook_tts_model', label: 'Mô hình TTS bên thứ ba', type: 'text' },
      { key: 'audiobook_rate', label: 'Chỉnh tốc độ đọc', type: 'text' },
      { key: 'audiobook_volume', label: 'Chỉnh âm lượng', type: 'text' },
      { key: 'audiobook_pitch', label: 'Chỉnh cao độ', type: 'text' },
      { key: 'audiobook_format', label: 'Định dạng xuất', type: 'select', options: [
        { value: 'mp3', label: 'mp3' }, { value: 'wav', label: 'wav' }
      ] },
      { key: 'audiobook_concurrency', label: 'Số chương tạo đồng thời', type: 'number', parse: 'int', min: '1' },
    ]
  },
];

const AUDIOBOOK_VOICE_PRESETS = [
  { value: 'zh-CN-XiaoxiaoNeural', label: 'zh-CN-XiaoxiaoNeural (Nữ)' },
  { value: 'zh-CN-XiaoyiNeural', label: 'zh-CN-XiaoyiNeural (Nữ)' },
  { value: 'zh-CN-YunjianNeural', label: 'zh-CN-YunjianNeural (Nam)' },
  { value: 'zh-CN-YunxiNeural', label: 'zh-CN-YunxiNeural (Nam)' },
  { value: 'zh-CN-YunxiaNeural', label: 'zh-CN-YunxiaNeural (Nam)' },
  { value: 'zh-CN-YunyangNeural', label: 'zh-CN-YunyangNeural (Nam)' },
  { value: 'zh-CN-liaoning-XiaobeiNeural', label: 'zh-CN-liaoning-XiaobeiNeural (Nữ)' },
  { value: 'zh-CN-shaanxi-XiaoniNeural', label: 'zh-CN-shaanxi-XiaoniNeural (Nữ)' },
  { value: 'zh-HK-HiuGaaiNeural', label: 'zh-HK-HiuGaaiNeural (Nữ)' },
  { value: 'zh-HK-HiuMaanNeural', label: 'zh-HK-HiuMaanNeural (Nữ)' },
  { value: 'zh-HK-WanLungNeural', label: 'zh-HK-WanLungNeural (Nam)' },
  { value: 'zh-TW-HsiaoChenNeural', label: 'zh-TW-HsiaoChenNeural (Nữ)' },
];

function renderFullConfigForm(cfg) {
  const body = document.getElementById('configFullBody');
  if (!body) return;
  body.innerHTML = '';

  for (const section of FULL_CONFIG_SCHEMA) {
    const sec = document.createElement('div');
    sec.className = 'configSection';
    sec.innerHTML = `<h4>${esc(section.title)}</h4>`;
    body.appendChild(sec);

    for (const field of section.fields) {
      const row = document.createElement('div');
      row.className = 'config-field';

      const label = document.createElement('span');
      label.className = 'field-label';
      label.textContent = field.label;
      row.appendChild(label);

      let input;
      if (field.type === 'bool') {
        input = document.createElement('input');
        input.type = 'checkbox';
        input.checked = !!cfg[field.key];
      } else if (field.type === 'voice') {
        input = document.createElement('div');
        input.className = 'voiceRow';
        const select = document.createElement('select');
        const emptyOpt = document.createElement('option');
        emptyOpt.value = '';
        emptyOpt.textContent = 'Tùy chỉnh...';
        select.appendChild(emptyOpt);
        for (const opt of AUDIOBOOK_VOICE_PRESETS) {
          const o = document.createElement('option');
          o.value = opt.value;
          o.textContent = opt.label;
          select.appendChild(o);
        }
        const text = document.createElement('input');
        text.type = 'text';
        text.value = (cfg[field.key] ?? '').toString();
        text.placeholder = 'Nhập hoặc chọn giọng đọc';
        text.dataset.key = field.key;
        text.dataset.type = 'text';
        text.dataset.voiceInput = '1';

        const current = (cfg[field.key] ?? '').toString();
        const preset = AUDIOBOOK_VOICE_PRESETS.find(p => p.value === current);
        select.value = preset ? preset.value : '';

        select.addEventListener('change', () => { if (select.value) text.value = select.value; });
        input.appendChild(select);
        input.appendChild(text);
      } else if (field.type === 'select') {
        input = document.createElement('select');
        for (const opt of field.options || []) {
          const o = document.createElement('option');
          o.value = opt.value;
          o.textContent = opt.label;
          input.appendChild(o);
        }
        input.value = (cfg[field.key] ?? '').toString();
      } else if (field.type === 'list') {
        input = document.createElement('textarea');
        input.value = Array.isArray(cfg[field.key]) ? cfg[field.key].join('\n') : '';
        input.placeholder = field.placeholder || '';
        input.classList.add('cfgList');
      } else if (field.type === 'number') {
        input = document.createElement('input');
        input.type = 'number';
        if (field.step) input.step = field.step;
        if (field.min) input.min = field.min;
        if (field.max) input.max = field.max;
        input.value = (cfg[field.key] ?? '').toString();
      } else {
        input = document.createElement('input');
        input.type = 'text';
        input.value = (cfg[field.key] ?? '').toString();
        if (field.placeholder) input.placeholder = field.placeholder;
      }

      if (field.type !== 'voice') {
        input.dataset.key = field.key;
        input.dataset.type = field.type;
        if (field.parse) input.dataset.parse = field.parse;
      }

      row.appendChild(input);
      sec.appendChild(row);
    }
  }
}

async function loadFullConfigPanel() {
  const msg = document.getElementById('cfgFullMsg');
  if (msg) msg.textContent = 'Đang tải…';
  try {
    const cfg = await j('/api/config/full');
    currentFullConfig = cfg || {};
    renderFullConfigForm(currentFullConfig);
    if (msg) msg.textContent = '';
  } catch (err) {
    if (msg) msg.textContent = 'Tải thất bại';
  }
}

function collectFullConfig() {
  const out = { ...(currentFullConfig || {}) };
  const body = document.getElementById('configFullBody');
  if (!body) return out;
  const inputs = body.querySelectorAll('[data-key]');
  for (const el of inputs) {
    const key = el.dataset.key;
    const type = el.dataset.type;
    if (!key || !type) continue;
    if (type === 'bool') {
      out[key] = !!el.checked;
    } else if (type === 'list') {
      out[key] = (el.value || '').toString().split(/[\n,;]/).map(s => s.trim()).filter(s => s.length > 0);
    } else if (type === 'number') {
      const raw = (el.value || '').toString().trim();
      if (!raw) continue;
      const parse = el.dataset.parse || 'int';
      const val = parse === 'float' ? parseFloat(raw) : parseInt(raw, 10);
      if (!Number.isNaN(val)) out[key] = val;
    } else {
      out[key] = (el.value || '').toString();
    }
  }
  return out;
}

async function saveFullConfig() {
  const cfg = collectFullConfig();
  await j('/api/config/full', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(cfg)
  });
}

// ── Library ────────────────────────────────────────────────────────

async function refreshLibrary() {
  const qs = libraryPath ? `?path=${encodeURIComponent(libraryPath)}` : '';
  const data = await j(`/api/library${qs}`);
  const items = data.items || [];
  libraryPath = (data.path || '').toString();

  const pathLabel = document.getElementById('libPath');
  const backBtn = document.getElementById('libBack');
  if (pathLabel) pathLabel.textContent = libraryPath ? `/${libraryPath}` : '/';
  if (backBtn) backBtn.disabled = !libraryPath;

  const tbody = document.getElementById('libraryBody');
  tbody.innerHTML = '';
  for (const it of items) {
    const tr = document.createElement('tr');
    const kind = it.kind || 'file';
    const rel = it.rel_path || '';
    const name = it.name || rel;
    const encodedRel = encodePathSegments(rel);
    const hrefFile = `/download/${encodedRel}`;
    const hrefZip = `/download-zip/${encodedRel}`;
    const sizeText = kind === 'dir'
      ? `${fmtBytes(it.size)} (${Number(it.file_count || 0)} file)`
      : fmtBytes(it.size);
    const timeText = fmtTime(it.modified_ms);

    if (kind === 'dir') {
      tr.innerHTML = `
        <td><button class="openDir sm" data-path="${esc(rel)}">Mở</button> ${esc(name)} <span class="badge">Thư mục</span></td>
        <td>${esc(sizeText)}</td>
        <td>${esc(timeText)}</td>
        <td><a href="${hrefZip}">Tải zip</a></td>
      `;
    } else {
      tr.innerHTML = `
        <td><a href="${hrefFile}">${esc(name)}</a> <span class="badge">${esc(it.ext || '')}</span></td>
        <td>${esc(sizeText)}</td>
        <td>${esc(timeText)}</td>
        <td><a href="${hrefFile}">Tải xuống</a></td>
      `;
    }
    tbody.appendChild(tr);
  }
  if (items.length === 0) {
    tbody.innerHTML = '<tr class="empty-row"><td colspan="4">Chưa có file nào, hãy tải một cuốn sách trước</td></tr>';
  }
}

// ── Search ─────────────────────────────────────────────────────────

async function doSearch(q) {
  const out = document.getElementById('searchResults');
  out.innerHTML = '';
  if (!q) return;
  const data = await j(`/api/search?q=${encodeURIComponent(q)}`);
  const items = data.items || [];
  if (items.length === 0) {
    out.innerHTML = '<tr class="empty-row"><td colspan="4">Không có kết quả</td></tr>';
    return;
  }
  for (const b of items) {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>${esc(b.title ?? '')}</td>
      <td>${esc(b.author ?? '')}</td>
      <td><code>${esc(b.book_id)}</code></td>
      <td><button data-bookid="${esc(b.book_id)}" class="startDownload sm primary">Tải</button></td>
    `;
    out.appendChild(tr);
  }
}

// ── Preview ────────────────────────────────────────────────────────

let currentPreviewBookId = null;
let currentPreviewData = null;

function showPreviewModal(show) {
  const modal = document.getElementById('previewModal');
  if (!modal) return;
  modal.classList.toggle('hidden', !show);
  document.body.style.overflow = show ? 'hidden' : '';
  if (!show) {
    // 关闭预览时，清理服务端因预览产生的封面缓存文件夹
    if (currentPreviewBookId) {
      fetchWithCreds(`/api/preview/${encodeURIComponent(currentPreviewBookId)}/cleanup`, {
        method: 'POST',
      }).catch(() => {}); // fire-and-forget
    }
    currentPreviewBookId = null;
    currentPreviewData = null;
  }
}

async function openPreview(bookId) {
  currentPreviewBookId = bookId;
  currentPreviewData = null;
  showPreviewModal(true);

  const loading = document.getElementById('previewLoading');
  const data = document.getElementById('previewData');
  const rangeInput = document.getElementById('previewRangeInput');
  const rangeHint = document.getElementById('previewRangeHint');

  if (loading) loading.classList.remove('hidden');
  if (data) data.classList.add('hidden');
  if (rangeInput) rangeInput.value = '';
  if (rangeHint) { rangeHint.textContent = ''; rangeHint.classList.remove('error'); }

  try {
    const preview = await j(`/api/preview/${encodeURIComponent(bookId)}`);
    currentPreviewData = preview;

    if (loading) loading.classList.add('hidden');
    if (data) data.classList.remove('hidden');

    const title = document.getElementById('previewTitle');
    const origTitle = document.getElementById('previewOrigTitle');
    const author = document.getElementById('previewAuthor');
    const stats = document.getElementById('previewStats');
    const desc = document.getElementById('previewDesc');
    const tags = document.getElementById('previewTags');
    const chapters = document.getElementById('previewChapters');
    const cover = document.getElementById('previewCover');

    if (title) title.textContent = preview.book_name || 'Tên sách không xác định';

    if (origTitle) {
      if (preview.original_book_name && preview.original_book_name !== preview.book_name) {
        origTitle.textContent = `Tên gốc: ${preview.original_book_name}`;
        origTitle.classList.remove('hidden');
      } else {
        origTitle.classList.add('hidden');
      }
    }

    if (author) author.textContent = preview.author ? `Tác giả: ${preview.author}` : 'Tác giả: không xác định';

    if (stats) {
      const parts = [];
      if (preview.chapter_count) parts.push(`Chương: ${preview.chapter_count}`);
      if (preview.finished !== null && preview.finished !== undefined) {
        parts.push(`Trạng thái: ${preview.finished ? 'Hoàn thành' : 'Đang đăng'}`);
      }
      if (preview.word_count) {
        const words = Number(preview.word_count);
        parts.push(`Số chữ: ${words >= 10000 ? (words / 10000).toFixed(1) + ' vạn' : words} chữ`);
      }
      if (preview.score != null) parts.push(`Đánh giá: ${preview.score.toFixed(1)}`);
      if (preview.read_count_text || preview.read_count) {
        parts.push(`Lượt đọc: ${preview.read_count_text || preview.read_count}`);
      }
      stats.innerHTML = '';
      parts.forEach(p => {
        const span = document.createElement('span');
        span.textContent = p;
        stats.appendChild(span);
      });
    }

    if (desc) desc.textContent = preview.description || 'Chưa có giới thiệu';

    if (tags) {
      if (preview.tags && preview.tags.length > 0) {
        tags.innerHTML = '';
        preview.tags.forEach(t => {
          const badge = document.createElement('span');
          badge.className = 'badge';
          badge.textContent = t;
          tags.appendChild(badge);
        });
        tags.classList.remove('hidden');
      } else {
        tags.classList.add('hidden');
      }
    }

    if (chapters) {
      const chapterInfo = [];
      if (preview.chapter_count) chapterInfo.push(`Tổng số chương: ${preview.chapter_count}`);
      if (preview.first_chapter_title) chapterInfo.push(`Chương đầu: ${preview.first_chapter_title}`);
      if (preview.last_chapter_title) chapterInfo.push(`Chương cuối: ${preview.last_chapter_title}`);
      if (preview.category) chapterInfo.push(`Thể loại: ${preview.category}`);
      chapters.innerHTML = '';
      chapterInfo.forEach(info => {
        const div = document.createElement('div');
        div.textContent = info;
        chapters.appendChild(div);
      });
    }

    if (cover) {
      const candidates = buildCoverCandidates(preview);
      let coverIdx = 0;

      const loadCandidate = () => {
        if (coverIdx >= candidates.length) {
          cover.removeAttribute('src');
          cover.classList.add('hidden');
          cover.onerror = null;
          return;
        }
        cover.src = candidates[coverIdx++];
        cover.classList.remove('hidden');
      };

      cover.onerror = () => loadCandidate();
      if (candidates.length > 0) {
        loadCandidate();
      } else {
        cover.removeAttribute('src');
        cover.classList.add('hidden');
        cover.onerror = null;
      }
    }

    if (rangeHint && preview.chapter_count) {
      rangeHint.textContent = `Ví dụ: 1-10 tải từ chương 1 đến 10, 1-${preview.chapter_count} tải toàn bộ`;
    }
  } catch (err) {
    if (loading) loading.textContent = `Tải thất bại: ${err}`;
    console.error('Preview load error:', err);
  }
}

async function confirmPreview() {
  if (!currentPreviewBookId || !currentPreviewData) { showPreviewModal(false); return; }

  const bookId = currentPreviewBookId;
  const rangeInput = document.getElementById('previewRangeInput');
  const rangeHint = document.getElementById('previewRangeHint');
  const rangeText = rangeInput ? rangeInput.value.trim() : '';

  let rangeStart = null;
  let rangeEnd = null;

  if (rangeText) {
    const total = currentPreviewData.chapter_count || 0;
    if (total === 0) {
      if (rangeHint) { rangeHint.textContent = 'Không biết số chương, không thể tải theo phạm vi'; rangeHint.classList.add('error'); }
      return;
    }
    const parts = rangeText.split('-').map(p => p.trim());
    if (parts.length === 2) {
      const start = parts[0] === '' ? 1 : parseInt(parts[0], 10);
      const end = parts[1] === '' ? total : parseInt(parts[1], 10);
      if (isNaN(start) || isNaN(end) || start < 1 || end < 1 || start > end || end > total) {
        if (rangeHint) { rangeHint.textContent = `Phạm vi không hợp lệ (1-${total})`; rangeHint.classList.add('error'); }
        return;
      }
      rangeStart = start;
      rangeEnd = end;
    } else {
      if (rangeHint) { rangeHint.textContent = 'Định dạng phải là start-end, ví dụ 1-10'; rangeHint.classList.add('error'); }
      return;
    }
  }

  if (rangeHint) rangeHint.classList.remove('error');
  showPreviewModal(false);

  try {
    const payload = { book_id: bookId };
    if (rangeStart !== null && rangeEnd !== null) {
      payload.range_start = rangeStart;
      payload.range_end = rangeEnd;
    }
    await j('/api/jobs', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(payload)
    });
    await refreshJobs();
    window.location.hash = '#jobs';
    const hint = document.getElementById('searchHint');
    if (hint) {
      hint.textContent = rangeStart && rangeEnd
        ? `Đã tạo tác vụ tải: ${bookId} (chương ${rangeStart}-${rangeEnd})`
        : `Đã tạo tác vụ tải: ${bookId}`;
    }
  } catch (err) {
    alert(`Tạo tác vụ thất bại: ${err}`);
  }
}

async function startDownload(bookId) {
  await openPreview(bookId);
  return null;
}

async function startDownloadDirect(bookId) {
  const job = await j('/api/jobs', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ book_id: bookId })
  });
  await refreshJobs();
  return job;
}

// ── Jobs ───────────────────────────────────────────────────────────

async function refreshJobs() {
  const data = await j('/api/jobs');
  const tbody = document.getElementById('jobsBody');
  tbody.innerHTML = '';
  for (const it of data.items || []) {
    const tr = document.createElement('tr');
    const saved = it.progress ? it.progress.saved_chapters : 0;
    const total = it.progress ? it.progress.chapter_total : 0;
    const pct = total > 0 ? Math.min(100, Math.round((saved / total) * 100)) : 0;
    const progressText = it.progress ? `${saved}/${total}` : '';
    const title = it.title || it.book_id || '';

    // Determine effective visual state
    let vState = (it.state || '').toLowerCase();
    if (vState === 'done' && total > 0 && saved < total) vState = 'partial';

    // Row class & CSS custom property for progress gradient
    tr.className = 'job-row state-' + vState;
    if (vState === 'running' || vState === 'queued') {
      tr.style.setProperty('--progress', pct + '%');
    }

    // State badge
    let stateHtml;
    switch (vState) {
      case 'running': stateHtml = `<span class="badge info">${pct}%</span>`; break;
      case 'queued':  stateHtml = '<span class="badge">Đang xếp hàng</span>'; break;
      case 'done':    stateHtml = '<span class="badge success">Hoàn tất</span>'; break;
      case 'failed':  stateHtml = '<span class="badge danger">Thất bại</span>'; break;
      case 'partial': stateHtml = '<span class="badge warning">Thất bại một phần</span>'; break;
      case 'canceled':stateHtml = '<span class="badge">Đã huỷ</span>'; break;
      default:        stateHtml = esc(it.state || '');
    }

    // Action button
    let btnHtml;
    switch (vState) {
      case 'done':
        btnHtml = `<button data-jobid="${esc(it.id)}" data-title="${esc(title)}" class="goLibrary sm success">Xong</button>`;
        break;
      case 'failed':
      case 'partial':
        btnHtml = `<button data-jobid="${esc(it.id)}" data-bookid="${esc(it.book_id)}" class="retryJob sm warning">Thử lại</button>`;
        break;
      case 'canceled':
        btnHtml = `<button data-jobid="${esc(it.id)}" data-bookid="${esc(it.book_id)}" class="retryJob sm">Thử lại</button>`;
        break;
      default: // running / queued
        btnHtml = `<button data-jobid="${esc(it.id)}" class="cancelJob sm">Huỷ</button>`;
    }

    tr.innerHTML = `
      <td><span class="badge">${esc(it.id)}</span></td>
      <td>${esc(title)}</td>
      <td>${stateHtml}</td>
      <td>${esc(progressText)}</td>
      <td>${btnHtml}</td>
    `;
    tbody.appendChild(tr);
  }
  if ((data.items || []).length === 0) {
    tbody.innerHTML = '<tr class="empty-row"><td colspan="5">Chưa có tác vụ nào</td></tr>';
  }

  const pending = (data.items || []).find(it => (it.book_name_options || []).length > 0);
  if (pending && !isBookNameModalOpen()) showBookNameModal(pending);
}

// ── History ───────────────────────────────────────────────────────

async function refreshHistory() {
  const hint = document.getElementById('historyHint');
  const body = document.getElementById('historyBody');
  const kw = (document.getElementById('historyKeyword')?.value || '').toString().trim();
  if (!body) return;

  if (hint) hint.textContent = 'Đang tải…';
  body.innerHTML = '<tr class="empty-row"><td colspan="6">Đang tải…</td></tr>';

  const qs = new URLSearchParams();
  qs.set('limit', '200');
  if (kw) qs.set('q', kw);

  const data = await j(`/api/history?${qs.toString()}`);
  const items = data.items || [];

  body.innerHTML = '';
  for (const it of items) {
    const tr = document.createElement('tr');
    const status = (it.status || '').toString().toLowerCase();
    const badge = status === 'success'
      ? '<span class="badge success">Thành công</span>'
      : '<span class="badge danger">Thất bại</span>';
    tr.innerHTML = `
      <td>${esc(it.timestamp || '')}</td>
      <td>${esc(it.book_name || '')}</td>
      <td>${esc(it.author || '')}</td>
      <td><code>${esc(it.book_id || '')}</code></td>
      <td>${esc(it.progress || '')}</td>
      <td>${badge}</td>
    `;
    body.appendChild(tr);
  }

  if (items.length === 0) {
    body.innerHTML = '<tr class="empty-row"><td colspan="6">Chưa có lịch sử</td></tr>';
  }
  if (hint) hint.textContent = `Tổng ${items.length} mục`;
}

// ── Updates ────────────────────────────────────────────────────────

async function refreshUpdates() {
  const hint = document.getElementById('updatesHint');
  const tbody = document.getElementById('updatesBody');
  if (!tbody) return;

  if (hint) hint.textContent = 'Đang quét…';
  tbody.innerHTML = '<tr class="empty-row"><td colspan="7">Đang tải…</td></tr>';

  const data = await j('/api/updates');
  const updates = data.updates || [];
  const noUpdates = data.no_updates || [];
  const total = updates.length + noUpdates.length;

  if (hint) hint.textContent = `Có thể cập nhật ${updates.length} cuốn / không có cập nhật ${noUpdates.length} cuốn / tổng ${total} cuốn`;

  tbody.innerHTML = '';
  for (const it of updates) {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>${esc(it.book_name || '')}</td>
      <td><code>${esc(it.book_id || '')}</code></td>
      <td>${esc(Number(it.local_total || 0))}</td>
      <td>${esc(Number(it.remote_total || 0))}</td>
      <td>${esc(Number(it.new_count || 0))}</td>
      <td>${esc(Number(it.local_failed || 0))}</td>
      <td><button data-bookid="${esc(it.book_id || '')}" class="startDownload sm primary">Cập nhật</button></td>
    `;
    tbody.appendChild(tr);
  }
  if (updates.length === 0) {
    tbody.innerHTML = '<tr class="empty-row"><td colspan="7">Không có sách nào cần cập nhật</td></tr>';
  }
}

async function cancelJob(id) {
  await j(`/api/jobs/${encodeURIComponent(id)}/cancel`, { method: 'POST' });
  await refreshJobs();
}

async function clearJob(id) {
  await j(`/api/jobs/${encodeURIComponent(id)}`, { method: 'DELETE' });
}

// ── Book Name Modal ────────────────────────────────────────────────

function isBookNameModalOpen() {
  const modal = document.getElementById('bookNameModal');
  return modal && !modal.classList.contains('hidden');
}

function showBookNameModal(job) {
  pendingBookNameJobId = job.id;
  pendingBookNameOptions = job.book_name_options || [];
  const modal = document.getElementById('bookNameModal');
  const hint = document.getElementById('bookNameJobHint');
  const options = document.getElementById('bookNameOptions');
  if (!modal || !options) return;

  if (hint) {
    const title = job.title || job.book_id || '';
    hint.textContent = title ? `《${title}》` : '';
  }

  options.innerHTML = '';
  pendingBookNameOptions.forEach((opt, idx) => {
    const id = `bookNameOpt_${idx}`;
    const row = document.createElement('label');
    row.className = 'row';
    row.innerHTML = `
      <input type="radio" name="bookNameOpt" id="${id}" value="${esc(opt.value)}" ${idx === 0 ? 'checked' : ''} />
      <span>${esc(opt.label)}: ${esc(opt.value)}</span>
    `;
    options.appendChild(row);
  });
  modal.classList.remove('hidden');
}

async function submitBookNameChoice(value) {
  if (!pendingBookNameJobId) return;
  await j(`/api/jobs/${encodeURIComponent(pendingBookNameJobId)}/book_name`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ value })
  });
  pendingBookNameJobId = null;
  pendingBookNameOptions = [];
  const modal = document.getElementById('bookNameModal');
  if (modal) modal.classList.add('hidden');
  await refreshJobs();
}

// ── Wire ───────────────────────────────────────────────────────────

function wire() {
  // -- Navigation --
  const navLinks = document.querySelectorAll('.nav a');
  const sections = document.querySelectorAll('.section');

  function switchSection(hash) {
    if (!hash) hash = '#status';
    navLinks.forEach(link => {
      link.classList.toggle('active', link.getAttribute('href') === hash);
    });
    sections.forEach(sec => {
      sec.classList.toggle('active', '#' + sec.id === hash);
    });
  }

  window.addEventListener('hashchange', () => switchSection(window.location.hash));
  switchSection(window.location.hash);

  // -- Theme Toggle --
  const themeBtn = document.getElementById('themeToggle');
  if (themeBtn) themeBtn.addEventListener('click', toggleTheme);
  updateThemeButton(getStoredTheme());

  // -- Config Tabs --
  const configTabs = document.querySelectorAll('.config-tab');
  const configPanels = {
    quick: document.getElementById('configPanelQuick'),
    full: document.getElementById('configPanelFull'),
    yaml: document.getElementById('configPanelYaml'),
  };
  let fullConfigLoaded = false;

  configTabs.forEach(tab => {
    tab.addEventListener('click', async () => {
      const target = tab.dataset.tab;
      configTabs.forEach(t => t.classList.toggle('active', t === tab));
      Object.entries(configPanels).forEach(([k, panel]) => {
        if (panel) panel.classList.toggle('active', k === target);
      });

      // Lazy-load full config on first switch
      if (target === 'full' && !fullConfigLoaded) {
        fullConfigLoaded = true;
        await loadFullConfigPanel();
      }
    });
  });

  // -- Library Back --
  const backBtn = document.getElementById('libBack');
  if (backBtn) {
    backBtn.addEventListener('click', async () => {
      const parts = (libraryPath || '').split('/').filter(Boolean);
      parts.pop();
      libraryPath = parts.join('/');
      try { await refreshLibrary(); } catch (err) { alert(err); }
    });
  }

  // -- Search --
  const searchForm = document.getElementById('searchForm');
  if (searchForm) {
    searchForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      const q = document.getElementById('q').value.trim();
      const hint = document.getElementById('searchHint');
      if (hint) hint.textContent = '';

      const bookId = parseBookId(q);
      if (bookId) {
        try {
          await startDownload(bookId);
          if (hint) hint.textContent = `Đã tạo tác vụ tải: ${bookId}`;
          const out = document.getElementById('searchResults');
          if (out) out.innerHTML = '<tr class="empty-row"><td colspan="4">Đã thêm vào hàng đợi tác vụ, có thể xem tiến độ ở trang "Tác vụ"</td></tr>';
        } catch (err) {
          if (hint) hint.textContent = 'Tạo tác vụ thất bại';
          alert(err);
        }
        return;
      }
      try { await doSearch(q); } catch (err) { alert(err); }
    });
  }

  // -- Updates --
  const updBtn = document.getElementById('updatesRefresh');
  if (updBtn) updBtn.addEventListener('click', async () => {
    try { await refreshUpdates(); } catch (err) { alert(err); }
  });

  // -- App Update --
  const appUpdBtn = document.getElementById('appUpdateCheck');
  if (appUpdBtn) appUpdBtn.addEventListener('click', async () => {
    try { await refreshAppUpdate(true); } catch (err) { alert(err); }
  });

  const dismissBtn = document.getElementById('appUpdateDismiss');
  if (dismissBtn) dismissBtn.addEventListener('click', async () => {
    try {
      const { latestTag } = await refreshAppUpdate(false);
      if (latestTag) {
        setDismissedTag(latestTag);
        showAppUpdateBanner(false);
        const hint = document.getElementById('appUpdateHint');
        if (hint) hint.textContent = 'Đã đặt không nhắc lại';
      }
    } catch (err) { alert(err); }
  });

  const selfUpdBtn = document.getElementById('appSelfUpdate');
  if (selfUpdBtn) selfUpdBtn.addEventListener('click', async () => {
    const hint = document.getElementById('appUpdateHint');
    if (hint) hint.textContent = 'Đang khởi động tự cập nhật…';
    try {
      await j('/api/self_update', { method: 'POST' });
      if (hint) hint.textContent = 'Tác vụ tự cập nhật đã bắt đầu';
      await pollSelfUpdateStatus();
    } catch (err) {
      if (hint) hint.textContent = 'Kích hoạt tự cập nhật thất bại';
      alert(err);
    }
  });

  const historyRefresh = document.getElementById('historyRefresh');
  if (historyRefresh) historyRefresh.addEventListener('click', async () => {
    try { await refreshHistory(); } catch (err) { alert(err); }
  });

  const historyKeyword = document.getElementById('historyKeyword');
  if (historyKeyword) historyKeyword.addEventListener('keydown', async (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      try { await refreshHistory(); } catch (err) { alert(err); }
    }
  });

  // -- Quick Config Save --
  const cfgForm = document.getElementById('configForm');
  if (cfgForm) cfgForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const msg = document.getElementById('configMsg');
    if (msg) msg.textContent = 'Đang lưu…';
    try {
      await saveConfig();
      if (msg) msg.textContent = 'Đã lưu';
    } catch (err) {
      if (msg) msg.textContent = 'Lưu thất bại';
      alert(err);
    }
  });

  // -- Full Config Save --
  const cfgFullSave = document.getElementById('cfgFullSave');
  if (cfgFullSave) cfgFullSave.addEventListener('click', async () => {
    const msg = document.getElementById('cfgFullMsg');
    if (msg) msg.textContent = 'Đang lưu…';
    try {
      await saveFullConfig();
      await refreshConfig();
      await refreshRawConfig();
      if (msg) msg.textContent = 'Đã lưu';
    } catch (err) {
      if (msg) msg.textContent = 'Lưu thất bại';
      alert(err);
    }
  });

  // -- YAML Config --
  const cfgRawReload = document.getElementById('cfgRawReload');
  if (cfgRawReload) cfgRawReload.addEventListener('click', async () => {
    const msg = document.getElementById('cfgRawMsg');
    if (msg) msg.textContent = 'Đang tải…';
    try {
      await refreshRawConfig();
      if (msg) msg.textContent = 'Đã tải';
    } catch (err) {
      if (msg) msg.textContent = 'Tải thất bại';
      alert(err);
    }
  });

  const cfgRawSave = document.getElementById('cfgRawSave');
  if (cfgRawSave) cfgRawSave.addEventListener('click', async () => {
    const msg = document.getElementById('cfgRawMsg');
    if (msg) msg.textContent = 'Đang lưu…';
    try {
      await saveRawConfig();
      await refreshConfig();
      await refreshRawConfig();
      if (msg) msg.textContent = 'Đã lưu';
    } catch (err) {
      if (msg) msg.textContent = 'Lưu thất bại';
      alert(err);
    }
  });

  // -- Delegated Click Handlers --
  document.addEventListener('click', async (e) => {
    const t = e.target;
    if (!t || !t.classList) return;

    if (t.classList.contains('startDownload')) {
      const bookId = t.getAttribute('data-bookid');
      try { await startDownload(bookId); } catch (err) { alert(err); }
    }
    if (t.classList.contains('cancelJob')) {
      const id = t.getAttribute('data-jobid');
      if (!confirm('Bạn có chắc muốn huỷ tác vụ này và xoá khỏi danh sách không?')) return;
      try { await cancelJob(id); } catch (err) { alert(err); }
    }
    if (t.classList.contains('retryJob')) {
      const bookId = t.getAttribute('data-bookid');
      const jobId = t.getAttribute('data-jobid');
      try {
        await startDownloadDirect(bookId);
        if (jobId) {
          await clearJob(jobId).catch(() => {});
        }
        await refreshJobs();
      } catch (err) { alert(err); }
    }
    if (t.classList.contains('goLibrary')) {
      const title = t.getAttribute('data-title') || '';
      const jobId = t.getAttribute('data-jobid');
      if (jobId) {
        await clearJob(jobId).catch(() => {});
        await refreshJobs().catch(() => {});
      }
      libraryPath = '';
      window.location.hash = '#library';
      await refreshLibrary();
      highlightLibraryItem(title);
    }
    if (t.classList.contains('openDir')) {
      const p = (t.getAttribute('data-path') || '').toString();
      libraryPath = p;
      try { await refreshLibrary(); } catch (err) { alert(err); }
    }
  });

  // -- Escape Key for Modals --
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      const previewModal = document.getElementById('previewModal');
      if (previewModal && !previewModal.classList.contains('hidden')) {
        showPreviewModal(false);
        return;
      }
      const loginModal = document.getElementById('loginModal');
      if (loginModal && !loginModal.classList.contains('hidden')) {
        showLogin(false);
      }
    }
  });

  // -- Preview Modal Buttons --
  const previewConfirm = document.getElementById('previewConfirm');
  if (previewConfirm) previewConfirm.addEventListener('click', async () => {
    try { await confirmPreview(); } catch (err) { alert(err); }
  });

  const previewCancel = document.getElementById('previewCancel');
  if (previewCancel) previewCancel.addEventListener('click', () => showPreviewModal(false));

  const previewClose = document.getElementById('previewClose');
  if (previewClose) previewClose.addEventListener('click', () => showPreviewModal(false));

  // -- Book Name Modal --
  const bookNameConfirm = document.getElementById('bookNameConfirm');
  if (bookNameConfirm) bookNameConfirm.addEventListener('click', async () => {
    const selected = document.querySelector('input[name="bookNameOpt"]:checked');
    if (!selected) { alert('Vui lòng chọn một tên sách'); return; }
    await submitBookNameChoice(selected.value);
  });
}

function highlightLibraryItem(title) {
  if (!title) return;
  const rows = document.querySelectorAll('#libraryBody tr');
  for (const row of rows) {
    const firstTd = row.querySelector('td');
    if (firstTd && firstTd.textContent.includes(title)) {
      row.classList.add('lib-highlight');
      row.scrollIntoView({ behavior: 'smooth', block: 'center' });
      setTimeout(() => row.classList.remove('lib-highlight'), 3000);
      break;
    }
  }
}

// ── Boot ───────────────────────────────────────────────────────────

async function boot() {
  wire();
  await refreshStatus();
  if (!isDockerBuild) await refreshAppUpdate(false).catch(() => {});
  await refreshConfig();
  await refreshRawConfig();
  await refreshUpdates();
  await refreshJobs();
  await refreshHistory();
  await refreshLibrary();
  await pollSelfUpdateStatus();
  setInterval(() => refreshJobs().catch(() => {}), 1500);
  setInterval(() => refreshStatus().catch(() => {}), 5000);
  if (!isDockerBuild) {
    setInterval(() => refreshAppUpdate(false).catch(() => {}), 6 * 60 * 60 * 1000);
  }
}

boot().catch(err => console.error(err));
