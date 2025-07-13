<script lang="ts">
  import { generateMockData, type DateLoc } from "$lib/models";
  import { records } from "$lib/dataStore";
  import {
    getNextMonth,
    getPreviousMonth,
    getDateString,
    NewUTCDDate,
    WrapNumber
  } from "$lib/utils";

  import CalendarCell from "./CalendarCell.svelte";
  import { goto } from "$app/navigation";
  // export let viewDate: Date;

  let { viewDate } = $props();

  let dates = $records;
  let year = $derived(viewDate.getFullYear());
  let month = $derived(viewDate.getMonth());

  let title = $derived(`${viewDate.getFullYear()}-${viewDate.getMonth() + 1}`); // just for display really
  let cal = $derived(buildCalendarMatrix(viewDate));

  function isSameMonth(_left: Date, _right: Date) {
    // console.log(
    //   `Comparing ${_left.toISOString()} <<-->> ${_right.toISOString()}`
    // );
    if (_left.getFullYear() === _right.getFullYear()) {
      if (_left.getMonth() === _right.getMonth()) {
        return true;
      }
    }
    return false;
  }

  function isThisMonthClass(_left: Date, _right: Date) {
    if (isSameMonth(_left, _right)) {
      return "";
    } else {
      return "otherMonth";
    }
  }

  function buildCalendarMatrix(_date: Date) {
    console.log(`Building calendar matrix for ${_date}`);
    let _cal: DateLoc[][] = [];
    for (let col = 0; col < 7; col++) {
      _cal[col] = [];
      for (let row = 0; row < 6; row++) {
        const _cell_date = getDateString(
          getCellDate(_date.getFullYear(), _date.getMonth(), row, col)
        );

        let dayData = dates.find((x) => x.date === _cell_date);
        if (dayData == undefined) {
          dayData = { date: _cell_date, location: "?", working: false };
        }
        _cal[col][row] = dayData;
      }
    }
    console.log(`Returning cal data: ${JSON.stringify(_cal)}`);
    return _cal;
  }
  async function prevMonth() {
    let mth = getDateString(getPreviousMonth(viewDate));
    console.log(`going to prev month: ${mth}`);
    await goto(`/month/${mth}`);
  }
  async function nextMonth() {
    const mth = getDateString(getNextMonth(viewDate));
    console.log(`going to next month: ${mth}`);
    await goto(`/month/${mth}`);
  }
  function getCellDate(
    year: number,
    month: number,
    weekno: number,
    weekday: number
  ) {
    const row = weekno;
    const col = weekday;
    const baseDate = NewUTCDDate(year, month, 1)
    const startOfMonthOffset = baseDate.getUTCDay() + 6 
    const bump = baseDate.getUTCDay() === 0 ? 1: 0
    console.log(`bump: ${bump}`)
    const dayOfMonth = (7 * (row - bump)) + col - startOfMonthOffset + (7)

    // weekday assumes that 0 = monday
    // so if we get a 0, we're checking MONDAY
    // const dayOffset = sundayOffset + weekday + (7 * weekno) + rowOffset - 7;
    const dayOffset = dayOfMonth

    return NewUTCDDate(year, month, dayOffset + 1);
  }
</script>

<button onclick={prevMonth}>&lt;</button>
<h1>{title}</h1>
<button onclick={nextMonth}>&gt;</button>

<table>
  <thead>
    <tr>
      <th>M</th>
      <th>T</th>
      <th>W</th>
      <th>T</th>
      <th>F</th>
      <th>S</th>
      <th>S</th>
    </tr>
  </thead>
  <tbody>
    {#each { length: 6 }, weekno}
      <tr>
        {#each { length: 7 }, weekday}
          <!-- <td>{(weekday + (7 * (weekno)) + rowOffset) - 7}</td> -->
          <td
            class="cell {isThisMonthClass(
              getCellDate(year, month, weekno, weekday),
              viewDate
            )}"
          >
            <CalendarCell cellData={cal[weekday][weekno]}></CalendarCell>
          </td>
          <!-- <td>{getDate(year, month, weekno, weekday)}</td> -->
        {/each}
      </tr>
    {/each}
  </tbody>
</table>

<style>
  td {
    /* padding: 0.75em; */
  }
  .otherMonth {
    opacity: 0.1;
  }
  .cell {
    height: 2em;
    width: 2em;
  }
</style>
