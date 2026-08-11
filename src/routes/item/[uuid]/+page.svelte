<script lang="ts">
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<h1>item page</h1>
<h1>{JSON.stringify(data.item)}</h1>
