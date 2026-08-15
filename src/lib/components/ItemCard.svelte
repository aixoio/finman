<script lang="ts">
    import {
        display_format_item_type,
        type ItemType,
    } from "$lib/core/commands";

    interface Props {
        uuid?: string;
        name: string;
        item_type: ItemType;
        target_cents: number;
        current_cents: number;
        show_remaining?: boolean;
    }

    const {
        uuid,
        name,
        item_type,
        target_cents,
        current_cents,
        show_remaining = false,
    }: Props = $props();

    const target = $derived(target_cents / 100);
    const current = $derived(current_cents / 100);

    const percentage = $derived(
        ((current_cents / target_cents) * 100).toFixed(2),
    );

    const remaining = $derived(
        Math.max(0, (target_cents - current_cents) / 100).toFixed(2),
    );
</script>

{#snippet body()}
    <div class="card-body">
        <h1 class="card-title block max-w-full truncate">
            {name}
        </h1>
        <div class="flex justify-between w-full">
            <span class="text-xs text-base-content/50"
                >{display_format_item_type(item_type)}</span
            >

            {#if show_remaining}
                <span
                    class="px-3 py-0.5 bg-neutral-700 rounded-full shadow border border-neutral-content/25 text-base-content text-xs"
                    >${remaining} left</span
                >
            {/if}
        </div>

        <progress
            class="progress w-full h-4"
            value={current_cents}
            max={target_cents}
        ></progress>

        <div class="flex justify-between">
            <span class="text-xs text-base-content/50"
                >${current} / ${target}</span
            >

            <span class="text-xs text-base-content/65">{percentage}%</span>
        </div>
    </div>
{/snippet}

{#if uuid}
    <a
        class="card border border-neutral shadow bg-base-100 cursor-pointer hover:border-base-content/15"
        href="/item/{uuid}"
    >
        {@render body()}
    </a>
{:else}
    <div class="card border border-neutral shadow bg-base-100">
        {@render body()}
    </div>
{/if}
