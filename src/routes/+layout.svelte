<script lang="ts">
    import "./layout.css";
    import Sidebar from "./Sidebar.svelte";

    const { children } = $props();
</script>

<div class="flex min-h-screen">
    <Sidebar></Sidebar>

    <main class="min-w-0 flex-1">
        {@render children()}
    </main>
</div>
