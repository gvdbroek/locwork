import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { DateLoc } from './models';
import { invoke } from "@tauri-apps/api/core";


export const records: Writable<DateLoc[]> = writable();
export const locations: Writable<string[]> = writable();


export const dataService ={
    loadInitialData: (recordData: DateLoc[]) => {
        let locationSet: Set<string> = new Set();
        
        recordData.forEach((val, _, __) =>{locationSet.add(val.location)})
        
        locations.set([...locationSet]);
        records.set(recordData);
    },


}

export const initializeDataService = async () =>{

    console.log("Initializing data service");
    let datelocData:DateLoc[];
    if(true){
        datelocData = await invoke("get_mock_data");
    }
    else{
        console.warn("Data retrieval from API is not implemented yet")
        datelocData = [];

    }
    console.log(`Loaded ${datelocData.length} record(s)`)
    dataService.loadInitialData(datelocData);
    console.log("Finished")
}
