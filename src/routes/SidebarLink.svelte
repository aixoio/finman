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
        class="flex cursor-pointer active:cursor-pointer items-center gap-2 text-base-content/90 rounded-box px-2 py-0.5 bg-base-100 hover:bg-base-content/15 active:bg-base-content/20 select-none"
    >
        {@render content()}
    </a>
{:else}
    <a
        {href}
        class="flex cursor-pointer active:cursor-pointer items-center gap-2 text-base-content/90 rounded-box px-2 py-0.5 hover:bg-base-content/5 active:bg-base-content/20 select-none"
    >
        {@render content()}
    </a>
{/if}
