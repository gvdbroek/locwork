<script lang="ts">
  import { goto } from "$app/navigation";
  import { type DateLoc } from "$lib/models";
  export let cellData: DateLoc;

  let location = "unknown";
  if (cellData.working == false) {
    location = "nowork";
  } else {
    if (cellData.location == "home") {
      location = "home";
    } else {
      location = "office";
    }
  }

  async function go_to_date() {
    await goto(`/day/${cellData.date}`);
  }
</script>

<button class="cell" onclick={go_to_date}>
  <span class="dateday">{cellData.date.split("-")[2]}</span>
  <div class="loc {location}"></div>
</button>

<style>
  .cell {
    /* padding: 0.5em; */
    padding: 0%;
    width: 3em;
    height: 3em;
    display: inline-block;
    text-align: right;
    position: relative;


  }

  .notthismonth {
    opacity: 0.5;
  }
  .dateday {
    position: absolute;
    font-size: xx-small;
    top:0;
    right:0;
    padding: 5px;
    opacity: 0.5;
  }
  .loc {
    position: absolute;
    left: 50%;
    top: 60%;
    transform: translate(-50%, -50%);
    width: 1em;
    height: .75em;
    border-radius: 2px;
    /* margin-left: 10px; */
    /* margin-bottom: 5px; */
  }
  .nowork {
    background-color: rgb(56, 56, 56);
  }
  .office {
    background-color: yellow;
  }
  .home {
    background-color: yellowgreen;
  }
  .unknown {
    background-color: grey;
  }
</style>
