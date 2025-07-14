<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { records } from "$lib/dataStore";

  let currentMonth = $derived(
    $records.filter((d) => {
      return d.date.substring(0, 7) === page.params.date.substring(0, 7);
    })
  );
  let todayData = $derived(
    $records.find((d) => {
      return d.date === page.params.date;
    })
  );

  let homeCount = $derived(
    currentMonth.filter((d) => {
      return d.location === "home" && d.working === true;
    }).length
  );
  let officeCount = $derived(
    currentMonth.filter((d) => {
      return d.location !== "home" && d.working === true;
    }).length
  );

  let ratio = $derived((homeCount / (homeCount + officeCount)) * 100);
  let workDayCount = $derived(currentMonth.filter((d) => d.working).length);

  async function go_up() {
    console.log("going up");
    let currentDate = new Date(page.params.date);
    let month = currentDate.toISOString().split("T")[0].substring(0, 7);
    await goto(`/month/${month}`);
  }
  async function go_back() {
    console.log("going backward");
    let currentDate = new Date(page.params.date);
    let yesterday = new Date(page.params.date);
    yesterday.setDate(currentDate.getDate() - 1);
    let yesterdayStr = yesterday.toISOString().split("T")[0];
    console.log(yesterdayStr);
    await goto(`/day/${yesterdayStr}`);
  }
  async function go_forward() {
    console.log("going forward");
    let currentDate = new Date(page.params.date);
    let tomorrow = new Date(page.params.date);
    tomorrow.setDate(currentDate.getDate() + 1);
    let tomorrowStr = tomorrow.toISOString().split("T")[0];
    console.log(tomorrowStr);
    await goto(`/day/${tomorrowStr}`);
  }
</script>

<main class="container">
  <div class="row">
    <button class="btn-flat navbutton" onclick={go_back}>&lt</button>
    <button class="btn-flat navcenter" onclick={go_up}>{page.params.date}</button>
    <button class="btn-flat navbutton" onclick={go_forward}>&gt</button>
  </div>
  <div class="row">
    <div class="graph">
      <h1 class="rationumber">{ratio.toFixed(1)}%</h1>
    </div>
  </div>
  <div class="row">
    <div class="region">
      <p>Home {homeCount}/{workDayCount}</p>
      <p>Work {officeCount}/{workDayCount}</p>
    </div>
  </div>
  <div class="row">
    <div>
      <button class="btn-flat">pick location</button>
    </div>
  </div>
</main>

<style>
  .navcenter{
    margin-left: 5vh;
    margin-right: 5vh;
    width: 120px;
    justify-content: baseline;
    padding-top: 10px;
    height: 8vh;

  }
  .navbutton {
    width: 8vh;
    height: 8vh;
  }
  .graph {
    margin-left: 10%;
    margin-right: 10%;
    width: 80%;
    height: 20em;
    background-color: var(--secondary-color);
    position: relative;
  }
  .rationumber {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    margin: 0;
    font-size: 8vh;
  }
  .region {
    background-color: #141c28;
    color: var(--text-color);
    width: 80%;
    border-radius: 1em;
    margin-left: 10%;
    margin-right: 10%;
  }
</style>
