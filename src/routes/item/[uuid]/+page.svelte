<script lang="ts">
    import ItemCard from "$lib/components/ItemCard.svelte";
    import type { Snippet } from "svelte";
    import type { PageProps } from "./$types";
    import { update_item_with_uuid } from "$lib/core/commands";
    import { error } from "@sveltejs/kit";

    const { data }: PageProps = $props();

    let item_dialog: HTMLDialogElement;

    let item_dialog_cancel: () => Promise<void> = $state(async () => {});
    let item_dialog_confirm: () => Promise<void> = $state(async () => {});

    let item_dialog_content: Snippet | undefined = $state();

    function item_dialog_open(
        onCancel: () => Promise<void>,
        onConfirm: () => Promise<void>,
        snip: Snippet,
    ): void {
        item_dialog_cancel = onCancel;
        item_dialog_confirm = onConfirm;
        item_dialog_content = snip;

        item_dialog.showModal();
    }

    async function onCancel(): Promise<void> {
        item_dialog.close();
    }

    let item_dialog_disabled = $state(false);

    let update_item_amount: number = $state(0);

    let edit_item_name: string = $state("");
    let edit_item_target: number = $state(0);
    let edit_item_current: number = $state(0);

    $effect(() => {
        edit_item_name = data.item.name;
        edit_item_target = data.item.target_cents / 100;
        edit_item_current = data.item.current_cents / 100;
    });

    let edit_comment: string = $state("");

    $effect(() => {
        edit_comment = data.item.comment || "";
    });
</script>

<dialog bind:this={item_dialog} class="modal">
    <div class="modal-box border border-neutral">
        {@render item_dialog_content?.()}

        <div class="modal-action">
            <button
                class="btn btn-neutral"
                onclick={item_dialog_cancel}
                disabled={item_dialog_disabled}>Cancel</button
            >
            <button
                class="btn btn-primary"
                onclick={item_dialog_confirm}
                disabled={item_dialog_disabled}>Confirm</button
            >
        </div>
    </div>
</dialog>

<div class="xl:mx-auto xl:max-w-2/3 not-xl:m-8 xl:mt-8">
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
                {#snippet complete_goal_dialog()}
                    <h1 class="text-2xl font-bold">Confirm</h1>
                    <p>Are you sure you want to complete this goal?</p>
                {/snippet}
                <button
                    class="btn btn-sm btn-success"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;
                                const result = await update_item_with_uuid(
                                    data.item.uuid,
                                    {
                                        type: "CompleteGoal",
                                    },
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;

                                update_item_amount = 0;
                            },
                            complete_goal_dialog,
                        );
                    }}>Complete goal</button
                >
                {#snippet set_exact_dialog()}
                    <h1 class="text-2xl font-bold">Confirm</h1>
                    <p>
                        Are you sure you want to set the current amount to ${update_item_amount.toFixed(
                            2,
                        )}?
                    </p>
                {/snippet}
                <button
                    class="btn btn-sm btn-neutral"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;

                                const result = await update_item_with_uuid(
                                    data.item.uuid,
                                    {
                                        type: "SetExact",
                                        data: {
                                            amount_cents:
                                                update_item_amount * 100,
                                        },
                                    },
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;

                                update_item_amount = 0;
                            },
                            set_exact_dialog,
                        );
                    }}>Set exact</button
                >
            </div>

            <div class="flex gap-1 mt-2">
                <input
                    type="number"
                    class="input flex-1 w-full"
                    placeholder="amount"
                    min="0"
                    step="0.01"
                    bind:value={update_item_amount}
                />
                {#snippet add_dialog()}
                    <h1 class="text-2xl font-bold">Confirm</h1>
                    <p>Are you sure you want to update this goal?</p>
                    <div
                        class="mt-4 border border-neutral rounded-box shadow p-4"
                    >
                        <div class="flex justify-between">
                            <span class="text-md font-semibold mb-3"
                                >Preview</span
                            >
                            <span class="text-xs text-success/75 mb-3"
                                >+${update_item_amount.toFixed(2)}</span
                            >
                        </div>
                        <div class="flex justify-between">
                            <span
                                >${(data.item.current_cents / 100).toFixed(
                                    2,
                                )}</span
                            >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="currentColor"
                                class="bi bi-arrow-right w-6 h-6"
                                viewBox="0 0 16 16"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M1 8a.5.5 0 0 1 .5-.5h11.793l-3.147-3.146a.5.5 0 0 1 .708-.708l4 4a.5.5 0 0 1 0 .708l-4 4a.5.5 0 0 1-.708-.708L13.293 8.5H1.5A.5.5 0 0 1 1 8"
                                />
                            </svg>
                            <span
                                >${(
                                    (data.item.current_cents +
                                        update_item_amount * 100) /
                                    100
                                ).toFixed(2)}</span
                            >
                        </div>
                    </div>
                {/snippet}
                <button
                    class="btn btn-primary"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;

                                const result = await update_item_with_uuid(
                                    data.item.uuid,
                                    {
                                        type: "Add",
                                        data: {
                                            amount_cents:
                                                update_item_amount * 100,
                                        },
                                    },
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;

                                update_item_amount = 0;
                            },
                            add_dialog,
                        );
                    }}>Add</button
                >
                {#snippet subtract_dialog()}
                    <h1 class="text-2xl font-bold">Confirm</h1>
                    <p>Are you sure you want to update this goal?</p>
                    <div
                        class="mt-4 border border-neutral rounded-box shadow p-4"
                    >
                        <div class="flex justify-between">
                            <span class="text-md font-semibold mb-3"
                                >Preview</span
                            >
                            <span class="text-xs text-error/75 mb-3"
                                >-${update_item_amount.toFixed(2)}</span
                            >
                        </div>
                        <div class="flex justify-between">
                            <span
                                >${(data.item.current_cents / 100).toFixed(
                                    2,
                                )}</span
                            >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="currentColor"
                                class="bi bi-arrow-right w-6 h-6"
                                viewBox="0 0 16 16"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M1 8a.5.5 0 0 1 .5-.5h11.793l-3.147-3.146a.5.5 0 0 1 .708-.708l4 4a.5.5 0 0 1 0 .708l-4 4a.5.5 0 0 1-.708-.708L13.293 8.5H1.5A.5.5 0 0 1 1 8"
                                />
                            </svg>
                            <span
                                >${(
                                    (data.item.current_cents -
                                        update_item_amount * 100) /
                                    100
                                ).toFixed(2)}</span
                            >
                        </div>
                    </div>
                {/snippet}
                <button
                    class="btn btn-neutral"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;

                                const result = await update_item_with_uuid(
                                    data.item.uuid,
                                    {
                                        type: "Subtract",
                                        data: {
                                            amount_cents:
                                                update_item_amount * 100,
                                        },
                                    },
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;

                                update_item_amount = 0;
                            },
                            subtract_dialog,
                        );
                    }}>Subtract</button
                >
            </div>
        </div>
    </div>

    <div class="card border border-neutral shadow bg-base-100 mt-4">
        <div class="card-body">
            <h1 class="card-title">Edit goal</h1>

            <fieldset class="fieldset">
                <label class="label" for="item_name">Name</label>
                <input
                    type="text"
                    class="input w-full"
                    name="item_name"
                    id="item_name"
                    bind:value={edit_item_name}
                />

                <div class="flex gap-2">
                    <div class="flex-1">
                        <label class="label" for="item_target"
                            >Target Amount</label
                        >
                        <input
                            type="number"
                            class="input w-full"
                            name="item_target"
                            id="item_target"
                            min="0"
                            step="0.01"
                            bind:value={edit_item_target}
                        />
                    </div>

                    <div class="flex-1">
                        <label class="label" for="item_current"
                            >Current Amount</label
                        >
                        <input
                            type="number"
                            class="input w-full"
                            name="item_current"
                            id="item_current"
                            min="0"
                            step="0.01"
                            bind:value={edit_item_current}
                        />
                    </div>
                </div>

                {#snippet apply_dialog()}
                    <h1 class="text-2xl font-bold">Confirm</h1>

                    <p>
                        Are you sure you want to update the name, current amount
                        and target amount of this goal?
                    </p>
                {/snippet}
                <button
                    class="btn btn-accent btn-sm"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;
                                const result = await update_item_with_uuid(
                                    data.item.uuid,
                                    {
                                        type: "Edit",
                                        data: {
                                            name: edit_item_name,
                                            current_cents:
                                                edit_item_current * 100,
                                            target_cents:
                                                edit_item_target * 100,
                                        },
                                    },
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;
                            },
                            apply_dialog,
                        );
                    }}>Apply</button
                >
            </fieldset>
        </div>
    </div>

    <div class="card border border-neutral shadow bg-base-100 mt-4">
        <div class="card-body">
            <h1 class="card-title">Comment</h1>

            <textarea class="textarea w-full" rows="6" bind:value={edit_comment}
            ></textarea>

            {#snippet comment_dialog()}
                <h1 class="text-2xl font-bold">Confirm</h1>

                <p>Are you sure you want to update your comment?</p>
            {/snippet}
            <button
                class="btn btn-neutral"
                onclick={() => {
                    item_dialog_open(
                        onCancel,
                        async () => {
                            item_dialog_disabled = true;

                            const update_comment = edit_comment.trim() || null;

                            const result = await update_item_with_uuid(
                                data.item.uuid,
                                {
                                    type: "Comment",
                                    data: {
                                        comment: update_comment,
                                    },
                                },
                            );
                            if (!result.ok) error(500, result.data);

                            item_dialog.close();
                            item_dialog_disabled = false;
                        },
                        comment_dialog,
                    );
                }}>Save</button
            >
        </div>
    </div>
</div>
