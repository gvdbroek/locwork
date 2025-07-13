import { initializeDataService } from '$lib/dataStore';
import type { PageLoad } from './$types'
import { invoke } from "@tauri-apps/api/core";

export const load: PageLoad = async ({ params }) => {

    /// loads locdata from api
    await initializeDataService();

};