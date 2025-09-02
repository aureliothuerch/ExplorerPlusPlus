<script>
    let { children } = $props();
    import "../app.css";
    import DockMenu from "$components/DockMenu.svelte";
</script>

<DockMenu class="fixed left-1/2 -translate-x-1/2 z-50" />
{@render children()}
