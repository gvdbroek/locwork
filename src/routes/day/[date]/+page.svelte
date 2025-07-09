<div>
    <div>
        <button onclick={() => go_back()}>&lt</button>
        <p>{day}</p>
        <button onclick={() => go_forward()}>&gt</button>
    </div>
    <div>
        <p>Home 6/10</p>
        <p>Work 6/10</p>
    </div>
    <button>pick location</button>
</div>

<script lang="ts">

    import { goto } from "$app/navigation";
    import { page } from '$app/state'; 

    $: day = page.params.date;


    function go_back() {
        goto("/day/2025-01-01")
        
    }
    function go_forward() {
        
    }
</script>