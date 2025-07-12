<Calendar></Calendar>
<script lang="ts">
    import type {PageProps} from "./$types";
    import Calendar from "$lib/components/Calendar.svelte";

    // date contains data loaded from backend
    let {data} : PageProps = $props();

</script>
