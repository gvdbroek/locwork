<script lang="ts">
  import type { PageProps } from "./$types";
  import Calendar from "$lib/components/Calendar.svelte";
  import { page } from "$app/state";
  import { NewUTCDDate } from "$lib/utils";

  // date contains data loaded from backend
  let viewData = $derived(parseDate(page.params.month));
  let { data }: PageProps = $props();

  function parseDate(param: string) {
    // date is provided as YYYY-MM
    console.log(`Parsing date: ${param}`)
    const parts = param.split("-");
    const year = Number.parseInt(parts[0]);
    const month = Number.parseInt(parts[1]) - 1;

    const date = NewUTCDDate(year, month, 1);
    console.log(`Parsed date: ${date.toUTCString()}`)
    console.log(date);

    return date;
  }
</script>

<Calendar viewDate={viewData}></Calendar>
