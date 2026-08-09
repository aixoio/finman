<script lang="ts">
    import { goto } from "$app/navigation";
    import { insert_item, ItemType } from "$lib/core/commands";
    import { error } from "@sveltejs/kit";

    let loading = $state(false);

    let name: string = $state("");
    let comment: string = $state("");
    let item_type: ItemType = $state(ItemType.Savings);
    let target: number = $state(0);
    let current: number = $state(0);

    async function onsubmit(event: SubmitEvent): Promise<void> {
        event.preventDefault();

        loading = true;

        const insert_comment =
            comment.trim().length === 0 ? null : comment.trim();
        const target_cents = target * 100;
        const current_cents = current * 100;

        const result = await insert_item(
            name,
            insert_comment,
            item_type,
            target_cents,
            current_cents,
        );
        loading = false;

        if (!result.ok) error(500, result.data);
        const uuid = result.data;

        console.log(`uuid: ${uuid}`);

        // TODO: send user to `/item/${uuid}`
        goto("/");
    }

    type TargetBtnType = "-50%" | "+50%" | "Custom %";

    let target_dialog_element: HTMLDialogElement;

    function target_dialog(btn: TargetBtnType) {
        if (loading) return;

        target_dialog_element.showModal();
    }
</script>

<dialog bind:this={target_dialog_element} class="modal">
    <div class="modal-box">
        <h1 class="font-bold text-2xl">Adjust Target Amount</h1>
        <div class="flex justify-between text-sm text-base-content/50 mb-2">
            <span>Current target</span>
            <span>${current}</span>
        </div>

        <span class="text-xs">Presets</span>
        <div class="grid grid-cols-3 gap-2">
            <button class="btn btn-sm btn-error">-50%</button>
            <button class="btn btn-sm btn-error">-25%</button>
            <button class="btn btn-sm btn-error">-10%</button>
            <button class="btn btn-sm btn-secondary">+25%</button>
            <button class="btn btn-sm btn-secondary">+50%</button>
            <button class="btn btn-sm btn-neutral">Double</button>
        </div>

        <fieldset class="fieldset mt-2">
            <label for="target_precent" class="label">Percentage</label>
            <input
                type="number"
                name="target_precent"
                id="target_precent"
                class="input w-full"
            />
        </fieldset>

        <div class="modal-action">
            <button
                class="btn btn-neutral"
                onclick={() => target_dialog_element.close()}>Cancel</button
            >
            <button class="btn btn-primary">Confirm</button>
        </div>
    </div>
</dialog>

<div class="xl:mx-auto xl:max-w-2/3 not-xl:m-8">
    <div class="mt-10">
        <h1 class="text-3xl font-bold">new</h1>
        <div class="divider"></div>

        <form {onsubmit}>
            <fieldset class="fieldset">
                <label for="name" class="label">name</label>
                <input
                    type="text"
                    name="name"
                    id="name"
                    class="input w-full"
                    required
                    bind:value={name}
                    disabled={loading}
                />

                <label for="comment" class="label">comment</label>
                <textarea
                    name="comment"
                    id="comment"
                    class="textarea w-full"
                    disabled={loading}
                    bind:value={comment}></textarea>

                <label for="item_type" class="label">item type</label>
                <select
                    name="item_type"
                    id="item_type"
                    class="select w-full"
                    bind:value={item_type}
                    disabled={loading}
                >
                    <option value="Savings">Savings</option>
                    <option value="SelfLoan">Self Loan</option>
                    <option value="ExternalLoan">External Loan</option>
                </select>

                <div class="w-full grid grid-cols-2 gap-3">
                    <div>
                        <label for="target" class="label">target amount</label>
                        <input
                            type="number"
                            name="target"
                            id="target"
                            class="input w-full"
                            required
                            disabled={loading}
                            bind:value={target}
                        />
                        <div class="grid grid-cols-3 gap-1 pt-2">
                            <button
                                type="button"
                                class="btn btn-sm btn-error"
                                onclick={() => target_dialog("-50%")}
                                disabled={loading}>-50%</button
                            >
                            <button
                                type="button"
                                disabled={loading}
                                onclick={() => target_dialog("+50%")}
                                class="btn btn-sm btn-secondary">+50%</button
                            >
                            <button
                                type="button"
                                class="btn btn-sm btn-neutral"
                                onclick={() => target_dialog("Custom %")}
                                disabled={loading}>Custom %</button
                            >
                        </div>
                    </div>

                    <div>
                        <label for="starting" class="label"
                            >starting amount</label
                        >
                        <input
                            type="number"
                            name="starting"
                            id="starting"
                            class="input w-full"
                            required
                            disabled={loading}
                            bind:value={current}
                        />
                        <div class="grid grid-cols-3 gap-1 pt-2">
                            <button
                                type="button"
                                class="btn btn-sm btn-accent"
                                disabled={loading}>10% of target</button
                            >
                            <button
                                type="button"
                                class="btn btn-sm btn-accent"
                                disabled={loading}>50% of target</button
                            >
                            <button
                                type="button"
                                class="btn btn-sm btn-neutral"
                                disabled={loading}>Custom %</button
                            >
                        </div>
                    </div>
                </div>

                <div class="divider"></div>

                <button
                    type="submit"
                    class="btn btn-primary w-full"
                    disabled={loading}>create</button
                >
            </fieldset>
        </form>
    </div>
</div>
