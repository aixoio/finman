<script lang="ts">
    import ItemCard from "$lib/components/ItemCard.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<div class="xl:mx-auto xl:max-w-2/3 not-xl:m-8">
    <ItemCard
        name={data.item.name}
        item_type={data.item.item_type}
        target_cents={data.item.target_cents}
        current_cents={data.item.current_cents}
        show_remaining
    ></ItemCard>
</div>
