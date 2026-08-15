<script>
    import SidebarLink from "./SidebarLink.svelte";
</script>

<aside
    class="h-full w-1/4 shrink-0 overflow-y-auto overscroll-contain border-r border-r-neutral p-4 text-base-content xl:w-1/5"
>
    <h1 class="text-3xl font-bold">finman</h1>

    <div class="divider"></div>

    <div class="mt-2 flex flex-col gap-2">
        <SidebarLink href="/">
            {#snippet icon()}
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="currentColor"
                    class="bi bi-house w-8 h-8"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L2 8.207V13.5A1.5 1.5 0 0 0 3.5 15h9a1.5 1.5 0 0 0 1.5-1.5V8.207l.646.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293zM13 7.207V13.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V7.207l5-5z"
                    />
                </svg>
            {/snippet}
            <span>home</span>
        </SidebarLink>
        <SidebarLink href="/new">
            {#snippet icon()}
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="currentColor"
                    class="bi bi-house w-8 h-8"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"
                    />
                </svg>
            {/snippet}
            <span>new</span>
        </SidebarLink>
        <SidebarLink href="/license">
            {#snippet icon()}
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="currentColor"
                    class="bi bi-house w-8 h-8"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M0 8a4 4 0 0 1 7.465-2H14a.5.5 0 0 1 .354.146l1.5 1.5a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0L13 9.207l-.646.647a.5.5 0 0 1-.708 0L11 9.207l-.646.647a.5.5 0 0 1-.708 0L9 9.207l-.646.647A.5.5 0 0 1 8 10h-.535A4 4 0 0 1 0 8m4-3a3 3 0 1 0 2.712 4.285A.5.5 0 0 1 7.163 9h.63l.853-.854a.5.5 0 0 1 .708 0l.646.647.646-.647a.5.5 0 0 1 .708 0l.646.647.646-.647a.5.5 0 0 1 .708 0l.646.647.793-.793-1-1h-6.63a.5.5 0 0 1-.451-.285A3 3 0 0 0 4 5"
                    />
                    <path d="M4 8a1 1 0 1 1-2 0 1 1 0 0 1 2 0" />
                </svg>
            {/snippet}
            <span>license</span>
        </SidebarLink>
    </div>
</aside>
