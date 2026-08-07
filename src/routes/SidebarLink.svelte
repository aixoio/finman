<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        href: string;
        children: Snippet;
        icon: Snippet;
    }

    const { href, children, icon }: Props = $props();
</script>

<a
    {href}
    class="flex cursor-pointer items-center gap-2 rounded px-2 py-0.5 hover:bg-foreground/5 active:bg-foreground/20"
>
    {@render icon()}
    <div class="w-full"></div>
    {@render children()}
</a>
