<script>
    import SidebarLink from "./SidebarLink.svelte";
</script>

<aside
    class="fixed left-0 top-0 h-screen w-1/4 xl:w-1/5 bg-background-dark text-foreground p-4"
>
    <h1 class="text-3xl font-bold pb-3 border-b border-b-foreground/15">
        finman
    </h1>

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
    </div>
</aside>
