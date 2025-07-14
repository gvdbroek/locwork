import { goto } from '$app/navigation';
import { initializeDataService } from '$lib/dataStore';
import type { PageLoad } from './$types'
import { invoke } from "@tauri-apps/api/core";
import {getDateString} from "$lib/utils"
export const load: PageLoad = async ({ params }) => {

    /// loads locdata from api
    await initializeDataService();

    const today =  getDateString(new Date());
    await goto(`/day/${today}`)

};