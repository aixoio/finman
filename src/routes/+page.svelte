<script lang="ts">
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

{#if data.items.length === 0}
    <span class="text-base-content/75 flex justify-center m-20"
        >no items to show</span
    >
{/if}
