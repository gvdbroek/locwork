<script lang="ts">
  import { generateMockData, type DateLoc } from "$lib/models";
  import CalendarCell from "./CalendarCell.svelte";
  let locData = generateMockData(new Date().toISOString(), 30);

  // sort locData based on date
  // build a 2d matrix of dates
  let cal: DateLoc[][] = [];
  let dates = locData;
  let year = 2025;
  let month = 7;

  let title = `${year}-${month}`

  for (let col = 0; col < 7; col++) {
    cal[col] = [];
    for (let row = 0; row < 5; row++) {

        const d = getDate(year, month, row, col).toISOString().split('T')[0];
        let dayData = dates.find((x)=> x.date == d );
        if(dayData == undefined){
          dayData = {date: d, location:"?", working:false}
        }
        cal[col][row] = dayData;

    }
  }


   function isThisMonth(date: Date, currentDate: Date) {
    if (date.getFullYear() != currentDate.getFullYear()) return false;
    if (date.getMonth() != currentDate.getMonth()) return false;
    return true;
  } 
  function isThisMonthClass(date: Date, currentDate: Date){
    if(isThisMonth(date, currentDate)){
      return ""
    }
  else{
    return "otherMonth"
  }
  }


  function getDate(
    year: number,
    month: number,
    weekno: number,
    weekday: number
  ) {
    const baseDate = new Date(year, month);
    const rowOffset = baseDate.getDay() + 1;
    let date = new Date();
    const dayOffset = weekday + 7 * weekno + rowOffset - 7;
    date.setDate(baseDate.getDate() + dayOffset);
    return date;
  }
</script>

<h1>{title}</h1>

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
    {#each { length: 5 }, weekno}
      <tr>
        {#each { length: 7 }, weekday}
          <!-- <td>{(weekday + (7 * (weekno)) + rowOffset) - 7}</td> -->
          <td class="cell {isThisMonthClass(getDate(year, month, weekno, weekday), new Date(year, month-1))}">
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
  .otherMonth{
    opacity: 0.5;
  }
  .cell{
    height: 2em;
    width: 2em;
  }
</style>
