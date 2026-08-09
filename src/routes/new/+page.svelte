<div class="mx-auto max-w-2/3">
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
            </fieldset>
        </form>
    </div>
</div>
