<script lang="ts">
    import { page } from "$app/state";
    import type { Snippet } from "svelte";

    interface Props {
        href: string;
        children: Snippet;
        icon: Snippet;
    }

    const { href, children, icon }: Props = $props();
</script>

{#snippet content()}
    {@render icon()}
    <div class="w-full"></div>
    {@render children()}
{/snippet}

{#if page.url.pathname == href}
    <a
        {href}
        class="flex cursor-pointer active:cursor-pointer items-center gap-2 rounded-lg px-2 py-0.5 bg-sidebar-white/10 hover:bg-sidebar-white/15 active:bg-sidebar-white/20 select-none"
    >
        {@render content()}
    </a>
{:else}
    <a
        {href}
        class="flex cursor-pointer active:cursor-pointer items-center gap-2 rounded-lg px-2 py-0.5 hover:bg-sidebar-white/5 active:bg-sidebar-white/20 select-none"
    >
        {@render content()}
    </a>
{/if}
