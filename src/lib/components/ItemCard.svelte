<script lang="ts">
    import {
        display_format_item_type,
        type ItemType,
    } from "$lib/core/commands";

    interface Props {
        uuid: string;
        name: string;
        item_type: ItemType;
        target_cents: number;
        current_cents: number;
    }

    const { uuid, name, item_type, target_cents, current_cents }: Props =
        $props();

    const target = $derived(target_cents / 100);
    const current = $derived(current_cents / 100);

    const percentage = $derived(
        ((current_cents / target_cents) * 100).toFixed(2),
    );
</script>

<div class="card border border-neutral shadow bg-base-100">
    <div class="card-body">
        <h1 class="card-title block max-w-full truncate">
            {name}
        </h1>
        <span class="text-xs text-base-content/50"
            >{display_format_item_type(item_type)}</span
        >

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
</div>
