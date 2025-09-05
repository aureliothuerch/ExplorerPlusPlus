<script lang="ts">
    import Separator from "$lib/components/ui/separator/separator.svelte";
</script>

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
