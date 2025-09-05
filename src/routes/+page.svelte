<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import Sidebar from "$components/Sidebar.svelte";
  import {
    Folder,
    File
  } from "lucide-svelte";
  import Toolbar from "$components/Toolbar.svelte";
  import { START_PATH } from "$env/static/private";

  type Entry = { name: string; isDir: boolean };

  let viewMode = $state<"list" | "grid">("list");
  let sortBy = $state<"name-asc" | "name-desc">("name-asc");
  let query = $state("");
  let loading = $state(false);
  let currentPath = $state([START_PATH]);

  async function loadDir() {
    console.log("Loading dir...");
    loading = true;
    try {
      const base =
        currentPath[currentPath.length - 1] ?? "C:\\Projects\\st\\Thuer";
      const dir_files = await invoke<Entry[]>("list_files", { path: base });
      entries = dir_files.map((f) => ({ name: f.name, isDir: f.isDir }));
    } catch (err) {
      console.error("loadDir failed", err);
    } finally {
      loading = false;
    }
    console.log("Finished loading dir!");
  }

    function navigateBreadcrumb(index: number) {
    currentPath = currentPath.slice(0, index + 1);
    // call into Rust to list that path; for demo we reuse current dir
    loadDir();
  }

  function toggleView(m: "list" | "grid") { viewMode = m; }
  function setSort(s: "name-asc" | "name-desc") { sortBy = s; }

    function refresh() {
    console.log("Refreshing...");
    loadDir();
  }

  let entries = $state<Entry[]>([]);
  const filtered = $derived.by(() => {
    const q = query.trim().toLowerCase();
    return entries
      .filter((e) => (q ? e.name.toLowerCase().includes(q) : true))
      .sort((a, b) => {
        if (sortBy === "name-asc") return a.name.localeCompare(b.name);
        if (sortBy === "name-desc") return b.name.localeCompare(a.name);
        return 0;
      });
  });

  async function openEntry(item: Entry) {
    // open with opener
  }

  onMount(loadDir);
</script>

<!-- App Shell -->
<div class="flex h-screen w-full bg-background text-foreground">
  <Sidebar />

  <!-- Main -->
  <section class="flex-1 flex flex-col">
    <!-- Title Bar / Toolbar -->
    <Toolbar
      {currentPath}
      {viewMode}
      {sortBy}
      {query}
      {loading}
      onNavigateBreadcrumb={(i) => { currentPath = currentPath.slice(0, i + 1); loadDir(); }}
      onToggleView={(m) => viewMode = m}
      onSetSort={(s) => sortBy = s}
      onRefresh={() => loadDir()}
      onQueryChange={(v) => query = v}
    />
    
    <!-- Content -->
    <div class="flex-1">
      {#if loading}
        <div class="h-full flex items-center justify-center text-sm opacity-80">
          Loading…
        </div>
      {:else if filtered.length === 0}
        <div class="h-full flex items-center justify-center text-sm opacity-80">
          No items
        </div>
      {:else if viewMode === "list"}
        <ScrollArea class="h-full">
          <table class="w-full text-sm">
            <thead class="sticky top-0 bg-background border-b">
              <tr class="[&>th]:text-left [&>th]:font-medium">
                <th class="px-4 py-2">Name</th>
              </tr>
            </thead>
            <tbody>
              {#each filtered as item (item.name)}
                <tr
                  class="border-b hover:bg-muted/100 cursor-default"
                  ondblclick={() => openEntry(item)}
                >
                  <td class="px-4 py-2 flex items-center gap-2">
                    {#if item.isDir}<Folder class="w-4 h-4 opacity-80" />
                    {:else}<File class="w-4 h-4 opacity-80" />
                    {/if}
                    <span class="truncate">{item.name}</span>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </ScrollArea>
      {:else}
        <ScrollArea class="h-full">
          <div
            class="p-3 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3"
          >
            {#each filtered as item (item.name)}
              <button
                class="group w-full aspect-square rounded-2xl border bg-card hover:bg-accent transition-colors p-3 flex flex-col items-center justify-center"
                ondblclick={() => openEntry(item)}
              >
                <div class="mb-2">
                  {#if item.isDir}<Folder
                      class="w-10 h-10 opacity-80 group-hover:opacity-100"
                    />
                  {:else}<File
                      class="w-10 h-10 opacity-80 group-hover:opacity-100"
                    />
                  {/if}
                </div>
                <div class="text-xs text-center line-clamp-2">{item.name}</div>
              </button>
            {/each}
          </div>
        </ScrollArea>
      {/if}
    </div>
  </section>
</div>
