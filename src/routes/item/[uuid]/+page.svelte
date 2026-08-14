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

    <div class="divider"></div>

    <div class="card border border-neutral shadow bg-base-100">
        <div class="card-body">
            <h1 class="card-title">Update amount</h1>

            <div class="grid grid-cols-2 gap-2">
                <button class="btn btn-sm btn-success">Complete goal</button>
                <button class="btn btn-sm btn-neutral">Set exact</button>
            </div>

            <div class="flex gap-1 mt-2">
                <input
                    type="text"
                    class="input flex-1 w-full"
                    placeholder="amount"
                />
                <button class="btn btn-primary">Add</button>
                <button class="btn btn-neutral">Subtract</button>
            </div>
        </div>
    </div>
</div>
