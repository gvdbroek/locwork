<div>
    <div>
        <button onclick={go_back}>&lt</button>
        <button onclick={go_up}>{page.params.date}</button>
        <button onclick={go_forward}>&gt</button>
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
    import type { DateLoc } from '$lib/models'
    import { generateMockData} from "$lib/models"
    console.log(generateMockData(new Date().toISOString(), 30))

    async function go_up() {
        console.log("going up")
        let currentDate = new Date(page.params.date);
        let route = `${currentDate.getFullYear()}-${currentDate.getMonth()}`
        await goto(`/month/${route}`)
    }
    async function go_back() {
        console.log("going backward")
        let currentDate = new Date(page.params.date);
        let yesterday = new Date(page.params.date)
        yesterday.setDate(currentDate.getDate() - 1)
        let yesterdayStr = yesterday.toISOString().split('T')[0]
        console.log(yesterdayStr)
        await goto(`/day/${yesterdayStr}`)
    }
    async function go_forward() {
        console.log("going forward")
        let currentDate = new Date(page.params.date);
        let tomorrow = new Date(page.params.date)
        tomorrow.setDate(currentDate.getDate() + 1)
        let tomorrowStr = tomorrow.toISOString().split('T')[0]
        console.log(tomorrowStr)
        await goto(`/day/${tomorrowStr}`)
        
    }
</script>