<div class="xl:mx-auto xl:max-w-2/3 not-xl:m-8">
    <div class="mt-10">
        <h1 class="text-3xl font-bold">new</h1>
        <div class="divider"></div>

        <form>
            <fieldset class="fieldset">
                <label for="name" class="label">name</label>
                <input
                    type="text"
                    name="name"
                    id="name"
                    class="input w-full"
                    required
                />

                <label for="comment" class="label">comment</label>
                <textarea name="comment" id="comment" class="textarea w-full"
                ></textarea>

                <label for="item_type" class="label">item type</label>
                <select name="item_type" id="item_type" class="select w-full">
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
                        />
                        <div class="grid grid-cols-3 gap-1 pt-2">
                            <button type="button" class="btn btn-sm btn-error"
                                >-50%</button
                            >
                            <button
                                type="button"
                                class="btn btn-sm btn-secondary">+50%</button
                            >
                            <button type="button" class="btn btn-sm btn-neutral"
                                >Custom %</button
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
                        />
                        <div class="grid grid-cols-3 gap-1 pt-2">
                            <button type="button" class="btn btn-sm btn-accent"
                                >10% of target</button
                            >
                            <button type="button" class="btn btn-sm btn-accent"
                                >50% of target</button
                            >
                            <button type="button" class="btn btn-sm btn-neutral"
                                >Custom %</button
                            >
                        </div>
                    </div>
                </div>
            </fieldset>
        </form>
    </div>
</div>
