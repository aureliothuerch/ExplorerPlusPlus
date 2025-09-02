<script lang="ts">
  import { Motion, useMotionValue } from "svelte-motion";
  import { cn } from "$lib/utils";
  import DockItem from "./DockItem.svelte";
  import { AlbumIcon, HomeIcon, MonitorIcon } from "lucide-svelte";

  type DockEntry = {
    id: string;
    href?: string;
    icon?: {
      component: any;
      props?: Record<string, any>;
    };
  };

  const icons: Record<string, DockEntry["icon"]> = {
    homeIcon: {
      component: HomeIcon,
      props: {
        size: 32,
      },
    },
    albumIcon: {
      component: AlbumIcon,
      props: {
        size: 32,
      },
    },
    monitorIcon: {
      component: MonitorIcon,
      props: {
        size: 32,
      },
    },
  };

  export let side: "top" | "bottom" = "bottom";
  export let className: string;
  export { className as class };
  export const items: DockEntry[] = [
    { id: "1", href: "/home", icon: icons["homeIcon"] },
    { id: "2", href: "/albums", icon: icons["albumIcon"] },
    { id: "3", href: "/monitor", icon: icons["monitorIcon"] },
  ];

  const mouseX = useMotionValue(Infinity);
  const containerX = useMotionValue(0);

  let containerRef: HTMLDivElement;
</script>

<div class={cn(side === "top" ? "top-4" : "bottom-4", className)} {...$$restProps}>
  <Motion let:motion>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      use:motion
      bind:this={containerRef}
      class="h-16 items-end gap-4 rounded-full bg-neutral-950 border border-neutral-800 px-3 pb-2 flex shadow-inner shadow-neutral-300/5"
      on:mouseleave={() => mouseX.set(Infinity)}
      on:mousemove={(e) => {
        const rect = containerRef.getBoundingClientRect();
        if (rect) {
          mouseX.set(e.clientX - rect.left);
          containerX.set(rect.x);
        }
      }}
    >
      {#each items as dockItem}
        <DockItem {containerX} {mouseX} href={dockItem.href}>
          {#if dockItem?.icon}
            <svelte:component
              this={dockItem.icon.component}
              {...dockItem.icon.props}
            />
          {/if}
        </DockItem>
      {/each}
    </div>
  </Motion>
</div>
