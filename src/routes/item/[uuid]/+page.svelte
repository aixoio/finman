<script lang="ts">
    import ItemCard from "$lib/components/ItemCard.svelte";
    import type { Snippet } from "svelte";
    import type { PageProps } from "./$types";
    import {
        delete_item_with_uuid,
        update_item_with_uuid,
    } from "$lib/core/commands";
    import {
        cents_to_dollars,
        dollars_to_cents,
        format_cents,
        format_dollars,
        is_valid_dollar_amount,
        MAX_MONEY_CENTS,
    } from "$lib/core/money";
    import { error } from "@sveltejs/kit";
    import { goto } from "$app/navigation";

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
    const update_item_amount_valid = $derived(
        is_valid_dollar_amount(update_item_amount),
    );
    const update_item_cents = $derived(
        update_item_amount_valid ? dollars_to_cents(update_item_amount) : 0,
    );
    const can_add = $derived(
        update_item_amount_valid &&
            data.item.current_cents <= MAX_MONEY_CENTS - update_item_cents,
    );
    const can_subtract = $derived(
        update_item_amount_valid &&
            update_item_cents <= data.item.current_cents,
    );

    let edit_item_name: string = $state("");
    let edit_item_target: number = $state(0);
    let edit_item_current: number = $state(0);

    $effect(() => {
        edit_item_name = data.item.name;
        edit_item_target = cents_to_dollars(data.item.target_cents);
        edit_item_current = cents_to_dollars(data.item.current_cents);
    });

    const edit_item_valid = $derived(
        edit_item_name.trim().length > 0 &&
            is_valid_dollar_amount(edit_item_target, {
                allow_zero: false,
            }) &&
            is_valid_dollar_amount(edit_item_current),
    );

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

    {#if data.item.archived}
        <div role="alert" class="alert alert-warning mt-4">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                class="bi bi-archive w-6 h-6"
                viewBox="0 0 16 16"
            >
                <path
                    d="M0 2a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1v7.5a2.5 2.5 0 0 1-2.5 2.5h-9A2.5 2.5 0 0 1 1 12.5V5a1 1 0 0 1-1-1zm2 3v7.5A1.5 1.5 0 0 0 3.5 14h9a1.5 1.5 0 0 0 1.5-1.5V5zm13-3H1v2h14zM5 7.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5"
                />
            </svg>
            <span
                >This item is archived and cannot be edited unless you unarchive
                it first.</span
            >
        </div>
    {/if}

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
                    disabled={data.item.archived}
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
                        Are you sure you want to set the current amount to ${format_dollars(
                            update_item_amount,
                        )}?
                    </p>
                {/snippet}
                <button
                    class="btn btn-sm btn-neutral"
                    disabled={data.item.archived || !update_item_amount_valid}
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
                                            amount_cents: update_item_cents,
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
                    required
                    min="0"
                    step="0.01"
                    bind:value={update_item_amount}
                    disabled={data.item.archived}
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
                                >+${format_dollars(update_item_amount)}</span
                            >
                        </div>
                        <div class="flex justify-between">
                            <span
                                >${format_cents(data.item.current_cents)}</span
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
                                >${format_cents(
                                    data.item.current_cents + update_item_cents,
                                )}</span
                            >
                        </div>
                    </div>
                {/snippet}
                <button
                    class="btn btn-primary"
                    disabled={data.item.archived || !can_add}
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
                                            amount_cents: update_item_cents,
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
                                >-${format_dollars(update_item_amount)}</span
                            >
                        </div>
                        <div class="flex justify-between">
                            <span
                                >${format_cents(data.item.current_cents)}</span
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
                                >${format_cents(
                                    data.item.current_cents - update_item_cents,
                                )}</span
                            >
                        </div>
                    </div>
                {/snippet}
                <button
                    class="btn btn-neutral"
                    disabled={data.item.archived || !can_subtract}
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
                                            amount_cents: update_item_cents,
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
                    required
                    bind:value={edit_item_name}
                    disabled={data.item.archived}
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
                            required
                            min="0.01"
                            step="0.01"
                            bind:value={edit_item_target}
                            disabled={data.item.archived}
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
                            required
                            min="0"
                            step="0.01"
                            bind:value={edit_item_current}
                            disabled={data.item.archived}
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
                    disabled={data.item.archived || !edit_item_valid}
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
                                                dollars_to_cents(
                                                    edit_item_current,
                                                ),
                                            target_cents: dollars_to_cents(
                                                edit_item_target,
                                                { allow_zero: false },
                                            ),
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

            <textarea
                class="textarea w-full"
                rows="6"
                bind:value={edit_comment}
                disabled={data.item.archived}></textarea>

            {#snippet comment_dialog()}
                <h1 class="text-2xl font-bold">Confirm</h1>

                <p>Are you sure you want to update your comment?</p>
            {/snippet}
            <button
                class="btn btn-neutral"
                disabled={data.item.archived}
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

    <div class="card border border-neutral mt-4">
        <div class="card-body">
            <h1 class="card-title">Actions</h1>

            <div class="flex justify-between w-full">
                {#snippet delete_dialog()}
                    <h1 class="text-2xl font-bold">Delete</h1>
                    <p>
                        Are you sure you want to delete this item. You cannot
                        undo this action.
                    </p>
                {/snippet}
                <button
                    class="btn btn-error"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;
                                const result = await delete_item_with_uuid(
                                    data.item.uuid,
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;

                                if (data.item.archived) {
                                    await goto("/archived");
                                } else {
                                    await goto("/");
                                }
                            },
                            delete_dialog,
                        );
                    }}>Delete</button
                >
                {#snippet archive_dialog()}
                    <h1 class="text-2xl font-bold">
                        {data.item.archived ? "Unarchive" : "Archive"}
                    </h1>
                    {#if !data.item.archived}
                        <p>
                            Are you sure you want to archive this item, doing so
                            will make it read-only unless you later unarchive
                            it.
                        </p>
                    {:else}
                        <p>
                            Are you sure you want to unarchive this item, doing
                            so will make it editable unless you later archive
                            it.
                        </p>
                    {/if}
                {/snippet}
                <button
                    class="btn btn-error btn-outline"
                    onclick={() => {
                        item_dialog_open(
                            onCancel,
                            async () => {
                                item_dialog_disabled = true;
                                const result = await update_item_with_uuid(
                                    data.item.uuid,
                                    {
                                        type: "Archive",
                                        data: {
                                            archived: !data.item.archived,
                                        },
                                    },
                                );
                                if (!result.ok) error(500, result.data);

                                item_dialog.close();
                                item_dialog_disabled = false;
                            },
                            archive_dialog,
                        );
                    }}
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="currentColor"
                        class="bi bi-archive w-4 h-4"
                        viewBox="0 0 16 16"
                    >
                        <path
                            d="M0 2a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1v7.5a2.5 2.5 0 0 1-2.5 2.5h-9A2.5 2.5 0 0 1 1 12.5V5a1 1 0 0 1-1-1zm2 3v7.5A1.5 1.5 0 0 0 3.5 14h9a1.5 1.5 0 0 0 1.5-1.5V5zm13-3H1v2h14zM5 7.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5"
                        />
                    </svg>
                    {data.item.archived ? "Unarchive" : "Archive"}</button
                >
            </div>
        </div>
    </div>
</div>
