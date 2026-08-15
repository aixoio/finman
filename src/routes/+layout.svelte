<script lang="ts">
    import { afterNavigate } from "$app/navigation";
    import "./layout.css";
    import Sidebar from "./Sidebar.svelte";

    const { children } = $props();

    let content: HTMLElement;

    afterNavigate(() => {
        content?.scrollTo({ top: 0, left: 0 });
    });
</script>

<div class="flex h-dvh overflow-hidden">
    <Sidebar></Sidebar>

    <main
        bind:this={content}
        class="min-w-0 flex-1 overflow-y-auto overscroll-contain"
    >
        {@render children()}
    </main>
</div>
