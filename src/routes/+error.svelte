<script lang="ts">
    import { page } from "$app/state";
</script>

<div class="xl:mx-auto xl:max-w-2/3 not-xl:m-8">
    <div class="alert alert-error mt-10" role="alert">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6 shrink-0 stroke-current"
            fill="none"
            viewBox="0 0 24 24"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
        </svg>
        <span class="font-bold">
            {page.status} error
        </span>
    </div>
    <p class="w-full max-w-full wrap-anywhere flex justify-center mt-5">
        {page.error?.message}
    </p>
</div>
