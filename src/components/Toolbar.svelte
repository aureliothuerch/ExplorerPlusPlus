<script lang="ts">
  const props = $props<{
    currentPath: string;
    viewMode: "list" | "grid";
    sortBy: "name-asc" | "name-desc";
    query: string;
    loading: boolean;

    onNavigateBreadcrumb?: (i: number) => void;
    onToggleView?: (m: "list" | "grid") => void;
    onSetSort?: (s: "name-asc" | "name-desc") => void;
    onRefresh?: () => void;
    onQueryChange?: (v: string) => void;
  }>();

  let {
    currentPath,
    viewMode,
    sortBy,
    query,
    loading,
    onNavigateBreadcrumb,
    onToggleView,
    onSetSort,
    onRefresh,
    onQueryChange
  } = props;

  function navigateBreadcrumb(i: number) { onNavigateBreadcrumb?.(i); }
  function toggleView(m: "list" | "grid") { onToggleView?.(m); }
  function setSort(s: "name-asc" | "name-desc") { onSetSort?.(s); }
  function refresh() { onRefresh?.(); }
  function handleQueryInput(e: Event) {
    const v = (e.target as HTMLInputElement).value;
    onQueryChange?.(v);
  }

  // local UI state
  let showSort = $state(false);

  // derive path segments so breadcrumb renders correctly
  const segments = $derived.by(() =>
    (currentPath ?? "")
      .split(/[\\\/]+/)
      .filter(Boolean)
  );
</script>

<div class="flex items-center gap-2 px-3 py-2 border-b bg-card/80 backdrop-blur supports-[backdrop-filter]:bg-card/70">
  <!-- back/forward, wenn du Actions brauchst -> hier callbacks -->
  <!-- Breadcrumb -->
  <nav class="mx-2 hidden lg:flex items-center gap-1 select-none">
    {#each segments as seg, i}
      <button class="px-1 text-sm rounded hover:bg-muted"
        title={seg}
        onclick={() => navigateBreadcrumb(i)}>{seg}</button>
      {#if i < segments.length - 1}<span class="px-1 opacity-60">/</span>{/if}
    {/each}
  </nav>

  <div class="ml-auto flex items-center gap-2">
    <div class="relative">
      <svg class="w-4 h-4 absolute left-2 top-1/2 -translate-y-1/2 opacity-60" viewBox="0 0 24 24" fill="none"><path d="M21 21l-4.3-4.3M10.5 18a7.5 7.5 0 1 1 0-15 7.5 7.5 0 0 1 0 15Z" stroke="currentColor" stroke-width="2"/></svg>
      <input
        class="pl-8 w-64 h-9 rounded-xl border bg-background/90 shadow-sm focus:outline-none focus:ring-2 focus:ring-ring focus:border-input"
        placeholder="Search"
        value={query}
        oninput={handleQueryInput}
      />
    </div>

    <div class="relative" tabindex="0">
      <button class="rounded-xl border px-3 py-2" onclick={() => (showSort = !showSort)} aria-expanded={showSort} aria-haspopup="menu">Sort</button>
      <div class={`absolute right-0 mt-2 w-44 border rounded-xl bg-card shadow-md z-50 ${showSort ? '' : 'hidden'}`} role="menu">
        <button class="w-full text-left px-3 py-2 hover:bg-muted" role="menuitem" onclick={() => { setSort("name-asc"); showSort = false; }}>Name (A-Z)</button>
        <button class="w-full text-left px-3 py-2 hover:bg-muted" role="menuitem" onclick={() => { setSort("name-desc"); showSort = false; }}>Name (Z-A)</button>
      </div>
    </div>

    <div class="flex rounded-xl overflow-hidden border">
      <button class="px-2 py-2" aria-pressed={viewMode === "list"} onclick={() => toggleView("list")}>List</button>
      <button class="px-2 py-2" aria-pressed={viewMode === "grid"} onclick={() => toggleView("grid")}>Grid</button>
    </div>

    <button class="rounded-xl px-2 py-2" onclick={refresh} disabled={loading}>
      <span class={`inline-block ${loading ? 'animate-spin' : ''}`}>⟲</span>
    </button>
  </div>
</div>
