<script lang="ts">
    import "./layout.css";
    import Sidebar from "./Sidebar.svelte";

    const { children } = $props();
</script>

<div class="flex h-dvh overflow-hidden">
    <Sidebar></Sidebar>

    <main class="min-w-0 flex-1 overflow-y-auto overscroll-contain">
        {@render children()}
    </main>
</div>
