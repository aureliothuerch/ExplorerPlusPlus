<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import {
    DropdownMenu, DropdownMenuTrigger, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator
  } from '$lib/components/ui/dropdown-menu';
  import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from '$lib/components/ui/breadcrumb';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import { Separator } from '$lib/components/ui/separator';
  import { ChevronLeft, ChevronRight, RotateCw, LayoutList, LayoutGrid, Search, Folder, File, ChevronDown } from 'lucide-svelte';

  type Entry = { name: string; isDir: boolean };
  type PathFile = { name: string; is_folder: boolean };
  
  let entries: Entry[] = [];
  let filtered: Entry[] = [];
  let loading = false;

  let currentPath = ['./'];
  let viewMode: 'list' | 'grid' = 'list';
  let query = '';
  let sortBy: 'name-asc' | 'name-desc' = 'name-asc';

  async function loadDir() {
    loading = true;
    try {
      const dir_files = await invoke<PathFile[]>('list_files_dir');
      entries = dir_files.map((f) => ({ name: f.name, isDir: f.is_folder }));
      applyFilters();
    } catch (err) {
      console.error('loadDir failed', err);
    } finally {
      loading = false;
    }
  }

  function applyFilters() {
    const q = query.trim().toLowerCase();
    filtered = entries
      .filter((e) => (q ? e.name.toLowerCase().includes(q) : true))
      .sort((a, b) => {
        if (sortBy === 'name-asc') return a.name.localeCompare(b.name);
        if (sortBy === 'name-desc') return b.name.localeCompare(a.name);
        return 0;
      });
  }

  function toggleView(mode: 'list' | 'grid') {
    viewMode = mode;
  }

  function setSort(val: 'name-asc' | 'name-desc') {
    sortBy = val;
    applyFilters();
  }

  function openEntry(item: Entry) {
    // Placeholder; wire up opener plugin or a Rust command that opens files/dirs cross-platform.
    // Example with tauri-plugin-opener in Rust already present in your app.
    console.log('open', item.name);
  }

  function navigateBreadcrumb(index: number) {
    currentPath = currentPath.slice(0, index + 1);
    // call into Rust to list that path; for demo we reuse current dir
    loadDir();
  }

  function refresh() {
    loadDir();
  }

  $: applyFilters(); // reactive when query/sort change

  onMount(loadDir);
</script>

<!-- App Shell -->
<div class="flex h-screen w-full bg-background text-foreground">
  <!-- Sidebar -->
  <aside class="hidden md:flex w-64 flex-col border-r bg-muted/30">
    <div class="p-3">
      <div class="text-sm font-semibold tracking-tight">Quick Access</div>
    </div>
    <Separator />
    <nav class="p-2 space-y-1">
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Home</button>
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Desktop</button>
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Documents</button>
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Downloads</button>
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Pictures</button>
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Music</button>
      <button class="w-full px-3 py-2 rounded-xl hover:bg-muted text-left">Videos</button>
    </nav>
  </aside>

  <!-- Main -->
  <section class="flex-1 flex flex-col">
    <!-- Title Bar / Toolbar -->
    <div class="flex items-center gap-2 px-3 py-2 border-b bg-card">
      <Button variant="ghost" size="icon" class="rounded-xl"><ChevronLeft class="w-4 h-4" /></Button>
      <Button variant="ghost" size="icon" class="rounded-xl"><ChevronRight class="w-4 h-4" /></Button>

      <Breadcrumb class="mx-2 hidden lg:flex">
        {#each currentPath as seg, i}
          <BreadcrumbItem>
            <BreadcrumbLink on:click={() => navigateBreadcrumb(i)} class="cursor-pointer">{seg}</BreadcrumbLink>
          </BreadcrumbItem>
          {#if i < currentPath.length - 1}
            <BreadcrumbSeparator />
          {/if}
        {/each}
      </Breadcrumb>

      <div class="ml-auto flex items-center gap-2">
        <div class="relative">
          <Search class="w-4 h-4 absolute left-2 top-1/2 -translate-y-1/2 opacity-60" />
          <Input
            placeholder="Search"
            class="pl-8 w-56"
            bind:value={query}
            on:input={() => applyFilters()} />
        </div>

        <DropdownMenu>
          <DropdownMenuTrigger>
            <Button variant="outline" class="rounded-xl">
              Sort <ChevronDown class="w-4 h-4 ml-1" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" class="w-44">
            <DropdownMenuLabel>Sort by</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
              <button type="button" class="w-full text-left" on:click={() => setSort('name-asc')}>Name (A-Z)</button>
            </DropdownMenuItem>
            <DropdownMenuItem>
              <button type="button" class="w-full text-left" on:click={() => setSort('name-desc')}>Name (Z-A)</button>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        <div class="flex rounded-xl overflow-hidden border">
          <Button variant={viewMode === 'list' ? 'secondary' : 'ghost'} size="icon" class="rounded-none" on:click={() => toggleView('list')}>
            <LayoutList class="w-4 h-4" />
          </Button>
          <Button variant={viewMode === 'grid' ? 'secondary' : 'ghost'} size="icon" class="rounded-none" on:click={() => toggleView('grid')}>
            <LayoutGrid class="w-4 h-4" />
          </Button>
        </div>

        <Button variant="ghost" size="icon" class="rounded-xl" on:click={refresh} disabled={loading}>
          <RotateCw class="w-4 h-4 {loading ? 'animate-spin' : ''}" />
        </Button>
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1">
      {#if loading}
        <div class="h-full flex items-center justify-center text-sm opacity-80">Loading…</div>
      {:else if filtered.length === 0}
        <div class="h-full flex items-center justify-center text-sm opacity-80">No items</div>
      {:else}
        {#if viewMode === 'list'}
          <ScrollArea class="h-full">
            <table class="w-full text-sm">
              <thead class="sticky top-0 bg-background border-b">
                <tr class="[&>th]:text-left [&>th]:font-medium">
                  <th class="px-4 py-2">Name</th>
                </tr>
              </thead>
              <tbody>
                {#each filtered as item}
                  <tr class="border-b hover:bg-muted/50 cursor-default" on:dblclick={() => openEntry(item)}>
                    <td class="px-4 py-2 flex items-center gap-2">
                      {#if item.isDir}<Folder class="w-4 h-4 opacity-80" />{:else}<File class="w-4 h-4 opacity-80" />{/if}
                      <span class="truncate">{item.name}</span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </ScrollArea>
        {:else}
          <ScrollArea class="h-full">
            <div class="p-3 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3">
              {#each filtered as item}
                <button class="group w-full aspect-square rounded-2xl border bg-card hover:bg-accent transition-colors p-3 flex flex-col items-center justify-center"
                        on:dblclick={() => openEntry(item)}>
                  <div class="mb-2">
                    {#if item.isDir}<Folder class="w-10 h-10 opacity-80 group-hover:opacity-100" />
                    {:else}<File class="w-10 h-10 opacity-80 group-hover:opacity-100" />{/if}
                  </div>
                  <div class="text-xs text-center line-clamp-2">{item.name}</div>
                </button>
              {/each}
            </div>
          </ScrollArea>
        {/if}
      {/if}
    </div>
  </section>
</div>
