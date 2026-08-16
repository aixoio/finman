<script lang="ts">
    import ItemCard from "$lib/components/ItemCard.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

{#if data.items.length === 0}
    <span class="text-base-content/75 flex justify-center m-20"
        >no items to show</span
    >
{:else}
    <div class="grid grid-cols-2 gap-4 m-4">
        {#each data.items as item (item.uuid)}
            <ItemCard
                uuid={item.uuid}
                name={item.name}
                item_type={item.item_type}
                target_cents={item.target_cents}
                current_cents={item.current_cents}
            ></ItemCard>
        {/each}
    </div>
{/if}
