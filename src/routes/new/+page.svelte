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

        goto(`/item/${uuid}`);
    }

    type TargetBtnType = "-50%" | "+50%" | "Custom %";

    let target_dialog_percentage: number = $state(10);
    let preview_target_dialog = $derived(
        target + target * (target_dialog_percentage / 100),
    );

    let target_dialog_element: HTMLDialogElement;

    function target_dialog(btn: TargetBtnType) {
        if (loading) return;

        switch (btn) {
            case "-50%":
                target_dialog_percentage = -50;
                break;
            case "+50%":
                target_dialog_percentage = 50;
                break;
            case "Custom %":
                target_dialog_percentage = 10;
                break;

            default:
                break;
        }

        target_dialog_element.showModal();
    }

    type StartingBtnType = "10% of target" | "50% of target" | "Custom %";

    let starting_dialog_percentage: number = $state(5);
    let preview_starting_dialog = $derived(
        target * (starting_dialog_percentage / 100),
    );

    let starting_dialog_element: HTMLDialogElement;

    function starting_dialog(btn: StartingBtnType) {
        if (loading) return;

        switch (btn) {
            case "10% of target":
                starting_dialog_percentage = 10;
                break;
            case "50% of target":
                starting_dialog_percentage = 50;
                break;
            case "Custom %":
                starting_dialog_percentage = 5;
                break;

            default:
                break;
        }

        starting_dialog_element.showModal();
    }
</script>

<dialog bind:this={target_dialog_element} class="modal">
    <div class="modal-box border border-neutral">
        <h1 class="font-bold text-2xl">Adjust Target Amount</h1>
        <div class="flex justify-between text-sm text-base-content/50 mb-2">
            <span>Current target</span>
            <span>${target}</span>
        </div>

        <span class="text-xs">Presets</span>
        <div class="grid grid-cols-3 gap-2">
            <button
                onclick={() => (target_dialog_percentage = -50)}
                class="btn btn-sm btn-error">-50%</button
            >
            <button
                onclick={() => (target_dialog_percentage = -25)}
                class="btn btn-sm btn-error">-25%</button
            >
            <button
                onclick={() => (target_dialog_percentage = -10)}
                class="btn btn-sm btn-error">-10%</button
            >
            <button
                onclick={() => (target_dialog_percentage = 25)}
                class="btn btn-sm btn-secondary">+25%</button
            >
            <button
                onclick={() => (target_dialog_percentage = 50)}
                class="btn btn-sm btn-secondary">+50%</button
            >
            <button
                onclick={() => (target_dialog_percentage = 100)}
                class="btn btn-sm btn-neutral">Double</button
            >
        </div>

        <fieldset class="fieldset mt-2">
            <label for="target_precent" class="label">Percentage</label>
            <input
                type="number"
                name="target_precent"
                id="target_precent"
                class="input w-full"
                bind:value={target_dialog_percentage}
            />
        </fieldset>

        <div class="mt-4 border border-neutral rounded-box shadow p-4">
            <span class="text-md font-semibold mb-3">Preview</span>
            <div class="flex justify-between">
                <span>${target}</span>
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
                <span>${preview_target_dialog.toFixed(2)}</span>
            </div>
        </div>

        <div class="modal-action">
            <button
                class="btn btn-neutral"
                onclick={() => target_dialog_element.close()}>Cancel</button
            >
            <button
                class="btn btn-primary"
                onclick={() => {
                    target = Number(preview_target_dialog.toFixed(2));
                    target_dialog_element.close();
                }}>Confirm</button
            >
        </div>
    </div>
</dialog>

<dialog bind:this={starting_dialog_element} class="modal">
    <div class="modal-box border border-neutral">
        <h1 class="font-bold text-2xl">Adjust Starting Amount</h1>
        <div class="flex justify-between text-sm text-base-content/50 mb-2">
            <span>Current starting amount</span>
            <span>${current}</span>
        </div>

        <span class="text-xs">Presets</span>
        <div class="grid grid-cols-3 gap-2">
            <button
                onclick={() => (starting_dialog_percentage = 5)}
                class="btn btn-sm btn-accent">5% of target</button
            >
            <button
                onclick={() => (starting_dialog_percentage = 10)}
                class="btn btn-sm btn-accent">10% of target</button
            >
            <button
                onclick={() => (starting_dialog_percentage = 50)}
                class="btn btn-sm btn-accent">50% of target</button
            >
        </div>

        <fieldset class="fieldset mt-2">
            <label for="starting_percent" class="label">
                <div class="flex justify-between w-full">
                    <span>Percentage</span>
                    <span>{starting_dialog_percentage}%</span>
                </div>
            </label>
            <input
                type="range"
                name="starting_percent"
                id="starting_percent"
                class="range w-full range-primary"
                min="0"
                max="100"
                step="1"
                bind:value={starting_dialog_percentage}
            />
        </fieldset>

        <div class="mt-4 border border-neutral rounded-box shadow p-4">
            <span class="text-md font-semibold mb-3">Preview</span>
            <div class="flex justify-between">
                <span>{starting_dialog_percentage}% of ${target}</span>
                <span>is</span>
                <span>${preview_starting_dialog.toFixed(2)}</span>
            </div>
        </div>

        <div class="modal-action">
            <button
                class="btn btn-neutral"
                onclick={() => starting_dialog_element.close()}>Cancel</button
            >
            <button
                class="btn btn-primary"
                onclick={() => {
                    current = Number(preview_starting_dialog.toFixed(2));
                    starting_dialog_element.close();
                }}>Confirm</button
            >
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
                            step="0.01"
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
                            step="0.01"
                            disabled={loading}
                            bind:value={current}
                        />
                        <div class="grid grid-cols-3 gap-1 pt-2">
                            <button
                                type="button"
                                class="btn btn-sm btn-accent"
                                onclick={() => starting_dialog("10% of target")}
                                disabled={loading}>10% of target</button
                            >
                            <button
                                type="button"
                                class="btn btn-sm btn-accent"
                                onclick={() => starting_dialog("50% of target")}
                                disabled={loading}>50% of target</button
                            >
                            <button
                                type="button"
                                class="btn btn-sm btn-neutral"
                                onclick={() => starting_dialog("Custom %")}
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
