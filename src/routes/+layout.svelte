<script>
    let { children } = $props();
    import "../app.css";
    import DockMenu from "$components/DockMenu.svelte";
    import { afterNavigate } from "$app/navigation";

    const visibleRoutes = ["/", "/explorer", "/s3", "/ftp"]; // show dock only here
    let showDock = $state(false);

    $effect(() => {
        // set initial visibility based on current location
        showDock = visibleRoutes.includes(location.pathname);
    });

    afterNavigate(() => {
        showDock = visibleRoutes.includes(location.pathname);
    });
</script>

{#if showDock}
    <DockMenu class="fixed left-1/2 -translate-x-1/2 z-50 transition-opacity duration-300 opacity-100" />
{:else}
    <DockMenu class="fixed left-1/2 -translate-x-1/2 z-50 transition-opacity duration-300 opacity-0 pointer-events-none" />
{/if}
{@render children()}
