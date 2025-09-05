<script lang="ts">
  const props = $props<{
    currentPath: string[];
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
</script>

<div class="flex items-center gap-2 px-3 py-2 border-b bg-card">
  <!-- back/forward, wenn du Actions brauchst -> hier callbacks -->
  <!-- Breadcrumb -->
  <div class="mx-2 hidden lg:flex">
    {#each currentPath as seg, i}
      <button class="px-1 text-sm hover:underline" onclick={() => navigateBreadcrumb(i)}>{seg}</button>
      {#if i < currentPath.length - 1}<span class="px-1">/</span>{/if}
    {/each}
  </div>

  <div class="ml-auto flex items-center gap-2">
    <div class="relative">
      <svg class="w-4 h-4 absolute left-2 top-1/2 -translate-y-1/2 opacity-60" viewBox="0 0 24 24" fill="none"><path d="M21 21l-4.3-4.3M10.5 18a7.5 7.5 0 1 1 0-15 7.5 7.5 0 0 1 0 15Z" stroke="currentColor" stroke-width="2"/></svg>
      <input class="pl-8 w-56 input" placeholder="Search" value={query} oninput={handleQueryInput} />
    </div>

    <div class="relative">
      <button class="rounded-xl border px-3 py-2">Sort</button>
      <div class="absolute right-0 mt-2 w-44 border rounded-xl bg-card shadow">
        <button class="w-full text-left px-3 py-2 hover:bg-muted" onclick={() => setSort("name-asc")}>Name (A-Z)</button>
        <button class="w-full text-left px-3 py-2 hover:bg-muted" onclick={() => setSort("name-desc")}>Name (Z-A)</button>
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
