<script lang="ts">
    import "./layout.css";
    import Sidebar from "./Sidebar.svelte";

    const { children } = $props();
</script>

<Sidebar></Sidebar>

{@render children()}
