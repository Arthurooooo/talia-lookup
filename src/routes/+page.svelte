<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { readText as readClipboard } from '@tauri-apps/plugin-clipboard-manager';
  import { GraduationCap, RefreshCw, Settings, Search, X, ClipboardCopy, Power, Lightbulb, Clock } from '@lucide/svelte';
  import Toast from '$lib/components/Toast.svelte';
  import UserRow from '$lib/components/UserRow.svelte';
  import RecentRow from '$lib/components/RecentRow.svelte';
  import StudentDetail from '$lib/components/StudentDetail.svelte';
  import type { BubbleUser, BubbleContract, BubbleSession, BubbleMeeting, BubbleClass, StudentBundle, RecentUserStub } from '$lib/types';
  import { loadRecents, addRecent, removeRecent as removeRecentFn, clearRecents } from '$lib/recents';

  // ------- État -------
  let token = $state('');
  let tokenDraft = $state('');
  let clipboardText = $state('');
  let queryDraft = $state('');
  let users = $state<BubbleUser[]>([]);
  let suggestions = $state<{ query: string; results: BubbleUser[] }[]>([]);
  let selectedUser = $state<BubbleUser | null>(null);
  let contracts = $state<BubbleContract[]>([]);
  let sessions = $state<BubbleSession[]>([]);
  let meetings = $state<BubbleMeeting[]>([]);
  let activeTeacher = $state<BubbleUser | undefined>(undefined);
  let activeClass = $state<BubbleClass | undefined>(undefined);
  let accountManager = $state<BubbleUser | undefined>(undefined);
  let recents = $state<RecentUserStub[]>([]);
  let isLoading = $state(false);
  let error = $state<string | null>(null);
  let showSettings = $state(false);

  onMount(async () => {
    try {
      const t = await invoke<string | null>('load_token');
      if (t) token = t;
    } catch (e) { console.error(e); }
    recents = loadRecents();
    await refreshClipboard();
    // Check for updates en arrière-plan, sans bloquer.
    checkForUpdates().catch(e => console.error('updater:', e));
  });

  async function checkForUpdates() {
    try {
      const { check } = await import('@tauri-apps/plugin-updater');
      const update = await check();
      if (!update) return;
      // Le plugin updater a un dialog natif intégré (configuré via tauri.conf.json
      // `plugins.updater.dialog = true`) : il propose à l'utilisateur d'installer
      // et redémarre tout seul. Pas besoin de UI custom ici.
      await update.downloadAndInstall();
      const { relaunch } = await import('@tauri-apps/plugin-process');
      await relaunch();
    } catch (e) {
      // Pas critique si l'update check échoue (offline, endpoint pas configuré, etc.)
      console.warn('update check failed:', e);
    }
  }

  async function refreshClipboard() {
    try {
      const raw = (await readClipboard()) ?? '';
      const cleaned = raw.length > 80 || raw.includes('\n') ? '' : raw.trim();
      clipboardText = cleaned;
      queryDraft = cleaned;
      if (cleaned && token) {
        users = []; suggestions = [];
        await search(cleaned);
      }
    } catch (e) { console.error('clipboard read', e); }
  }

  async function search(query: string) {
    isLoading = true;
    error = null;
    try {
      const result = await invoke<BubbleUser[]>('search_users', { token, query });
      users = result;
      if (result.length === 1) {
        await openUser(result[0]);
      } else if (result.length < 3) {
        await runSuggestions(query, new Set(result.map(u => u.id)));
      }
    } catch (e: any) {
      error = String(e);
    } finally {
      isLoading = false;
    }
  }

  async function runSuggestions(original: string, exclude: Set<string>) {
    const tokens = original.split(/\s+/).filter(t => t.length >= 2);
    if (tokens.length < 2) return;
    const queries = Array.from(new Set([
      ...tokens,
      tokens.slice().reverse().join(' ')
    ])).filter(q => q.toLowerCase() !== original.toLowerCase());
    const out: { query: string; results: BubbleUser[] }[] = [];
    const seenFirst = new Set<string>();
    for (const q of queries) {
      try {
        const res = await invoke<BubbleUser[]>('search_users', { token, query: q });
        const filtered = res.filter(u => !exclude.has(u.id));
        if (filtered.length && filtered[0] && !seenFirst.has(filtered[0].id)) {
          seenFirst.add(filtered[0].id);
          out.push({ query: q, results: filtered });
        }
      } catch (_) {}
    }
    suggestions = out;
  }

  function runManualSearch() {
    const q = queryDraft.trim();
    users = []; suggestions = []; error = null;
    if (!q || !token) return;
    search(q);
  }

  async function openUser(user: BubbleUser) {
    selectedUser = user;
    recents = addRecent(user);
    error = null;
    isLoading = true;
    try {
      const b = await invoke<StudentBundle>('fetch_student_bundle', { args: { token, user } });
      contracts = b.contracts;
      sessions = b.sessions;
      meetings = b.meetings;
      accountManager = b.accountManager;
      activeTeacher = b.activeTeacher;
      activeClass = b.activeClass;
    } catch (e: any) {
      error = String(e);
    } finally {
      isLoading = false;
    }
  }

  async function openRecent(stub: RecentUserStub) {
    isLoading = true;
    try {
      const fresh = await invoke<BubbleUser | null>('get_user', { token, id: stub.id });
      if (fresh) {
        await openUser(fresh);
      } else {
        selectedUser = { id: stub.id, apiFullName: stub.fullName, profilePicture: stub.avatarUrl } as BubbleUser;
      }
    } catch (e: any) {
      error = String(e);
    } finally {
      isLoading = false;
    }
  }

  function backFromDetail() {
    selectedUser = null;
    contracts = []; sessions = []; meetings = [];
    activeTeacher = undefined; activeClass = undefined; accountManager = undefined;
  }

  function removeFromRecents(stub: RecentUserStub) {
    recents = removeRecentFn(stub.id);
  }

  async function pasteTokenFromClipboard() {
    try { tokenDraft = (await readClipboard()) ?? ''; }
    catch (e) { console.error(e); }
  }

  async function saveToken() {
    try {
      await invoke('save_token', { token: tokenDraft });
      token = tokenDraft;
      tokenDraft = '';
      showSettings = false;
      clipboardText = '';
      queryDraft = '';
      users = []; suggestions = [];
      selectedUser = null;
      error = null;
    } catch (e: any) {
      error = String(e);
    }
  }

  async function clearToken() {
    await invoke('clear_token');
    token = '';
  }

  function quitApp() {
    window.close();
  }
</script>

<Toast />

<div class="app">
  <header class="header">
    <GraduationCap size={18} color="var(--accent)" />
    <h1>Talia Lookup</h1>
    {#if token}
      <button class="icon-btn" onclick={refreshClipboard} title="Relire le presse-papier"><RefreshCw size={15} /></button>
    {/if}
    <button class="icon-btn" onclick={() => (showSettings = !showSettings)} title="Réglages"><Settings size={15} /></button>
  </header>
  <div class="divider"></div>

  {#if showSettings || !token}
    <div style="display: flex; flex-direction: column; gap: 10px;">
      <div>
        <div class="fw-bold text-sm">Token Bubble Data API</div>
        <div class="text-2xs text-secondary">Settings → API → Generate token. Stocké dans le credential manager OS.</div>
      </div>
      <div class="input-row">
        <input type="password" class="text-input" placeholder="Token" bind:value={tokenDraft} />
        <button class="btn" onclick={pasteTokenFromClipboard}>Coller</button>
      </div>
      <div class="flex items-center gap-8">
        <button class="btn btn-primary" disabled={!tokenDraft} onclick={saveToken}>Enregistrer</button>
        {#if token}<button class="btn btn-danger" onclick={clearToken}>Effacer</button>{/if}
        <div class="flex-1"></div>
        {#if token}<span class="text-2xs mono text-secondary">{token.slice(0, 6)}…{token.slice(-4)}</span>{/if}
      </div>
      <div class="divider" style="margin-top: 8px;"></div>
      <div class="flex items-center gap-8">
        <button class="btn btn-danger" onclick={quitApp}>
          <Power size={12} /> Quitter Talia Lookup
        </button>
        <div class="flex-1"></div>
        <span class="text-2xs text-tertiary">v0.1.0</span>
      </div>
    </div>
  {:else if selectedUser}
    <StudentDetail
      user={selectedUser}
      {contracts}
      {sessions}
      {meetings}
      {accountManager}
      {activeTeacher}
      {activeClass}
      {isLoading}
      onBack={backFromDetail}
    />
  {:else}
    <div class="input-row">
      <Search size={14} />
      <input
        type="text"
        class="text-input"
        placeholder="Nom d'élève…"
        bind:value={queryDraft}
        onkeydown={(e) => { if (e.key === 'Enter') runManualSearch(); }}
      />
      {#if queryDraft}
        <button class="icon-btn" onclick={() => { queryDraft = ''; users = []; suggestions = []; }}><X size={13} /></button>
      {/if}
      {#if queryDraft !== clipboardText && clipboardText}
        <button class="icon-btn" style="color: var(--blue);" onclick={() => { queryDraft = clipboardText; runManualSearch(); }} title="Recoller le presse-papier ({clipboardText})">
          <ClipboardCopy size={13} />
        </button>
      {/if}
    </div>

    <div class="flex items-center gap-6" style="min-height: 16px;">
      {#if isLoading}<span class="text-2xs text-secondary">Recherche…</span>
      {:else if queryDraft}<span class="text-2xs text-tertiary">{users.length} résultat{users.length > 1 ? 's' : ''}</span>{/if}
    </div>

    <div class="scroll" style="flex: 1;">
      {#if users.length}
        <div style="display: flex; flex-direction: column; gap: 4px;">
          {#each users as u (u.id)}
            <UserRow user={u} onSelect={openUser} />
          {/each}
        </div>
      {/if}

      {#if !users.length && queryDraft && !isLoading}
        <div class="text-secondary text-sm" style="margin: 8px 0;">Aucun résultat exact pour « {queryDraft} »</div>
      {/if}

      {#if suggestions.length}
        <div class="suggestion">
          <div class="section-h" style="font-size: 13px;">
            <Lightbulb class="icon" color="var(--yellow)" />
            Recherches alternatives
          </div>
          {#each suggestions as s (s.query)}
            <div style="margin-top: 8px;">
              <div class="flex items-center gap-6" style="margin-bottom: 4px;">
                <span class="text-sm fw-semibold">« {s.query} »</span>
                <span class="text-2xs text-secondary">{s.results.length} résultat{s.results.length > 1 ? 's' : ''}</span>
                <div class="flex-1"></div>
                <button class="btn-link" onclick={() => { queryDraft = s.query; users = s.results; suggestions = []; }}>Utiliser</button>
              </div>
              {#each s.results.slice(0, 3) as u (u.id)}
                <UserRow user={u} onSelect={openUser} />
              {/each}
              {#if s.results.length > 3}
                <div class="text-2xs text-tertiary" style="padding-left: 8px;">+ {s.results.length - 3} autres — clique « Utiliser »</div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}

      {#if !users.length && !queryDraft && recents.length}
        <div>
          <div class="section-h" style="font-size: 13px;">
            <Clock class="icon" color="var(--indigo)" />
            Récemment consultés
            <div class="flex-1"></div>
            <button class="btn-link" onclick={() => { clearRecents(); recents = []; }}>Vider</button>
          </div>
          <div style="display: flex; flex-direction: column; gap: 2px; margin-top: 4px;">
            {#each recents as r (r.id)}
              <RecentRow stub={r} onTap={openRecent} onRemove={removeFromRecents} />
            {/each}
          </div>
        </div>
      {/if}

      {#if !users.length && !queryDraft && !recents.length}
        <div class="text-secondary text-sm" style="margin-top: 20px;">
          Copie un nom d'élève puis ouvre, ou tape ci-dessus.
        </div>
      {/if}
    </div>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}
</div>
