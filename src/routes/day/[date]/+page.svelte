<div>
    <div>
        <button onclick={go_back}>&lt</button>
        <button onclick={go_up}>{page.params.date}</button>
        <button onclick={go_forward}>&gt</button>
    </div>
    <div>
        <p>Home {homeCount}/{workDayCount}</p>
        <p>Work {officeCount}/{workDayCount}</p>
    </div>
    <button>pick location</button>
</div>

<script lang="ts">

    import { goto } from "$app/navigation";
    import { page } from '$app/state'; 
    import {records} from "$lib/dataStore"

    let currentMonth = $derived( $records.filter( (d) => {
        return d.date.substring(0,7) === page.params.date.substring(0,7)
    
    }))
    let todayData = $derived($records.find( (d) => {
        return d.date === page.params.date
    }))

    let homeCount = $derived( currentMonth.filter( (d) => {
        return (d.location === "home" && d.working === true)}).length);
    let officeCount = $derived( currentMonth.filter( (d) => {
        return (d.location !== "home" && d.working === true)}).length);
    let workDayCount = $derived(
            currentMonth.filter( (d) => d.working).length

    )



    async function go_up() {
        console.log("going up")
        let currentDate = new Date(page.params.date);
        let month = currentDate.toISOString().split('T')[0].substring(0,7)
        await goto(`/month/${month}`)
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