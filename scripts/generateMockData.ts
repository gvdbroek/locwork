
import fs from 'fs'

export interface DateLoc {
    date: string;
    location: string;
    working: boolean;
}

export function generateMockDateLoc(date: Date, locations: string[]) {
    let workday = true;
    if (date.getDay() < 1) {
        workday = false;
    }
    if (date.getDay() > 5) {
        workday = false;
    }

    const idx = Math.floor((Math.random() * 1000) % 3)

    const newData: DateLoc = {
        date: date.toISOString().split('T')[0],
        location: locations[idx],
        working: workday

    }
    return newData;

}

export function generateMockData(startDate: string, numDays: number): DateLoc[] {
    const locations = ["home", "office", "otherwork"]

    let mockData: DateLoc[] = [];
    for (let index = 0; index < numDays; index++) {

        let date = new Date(startDate)
        date.setDate(date.getDate() - index)
        const newData = generateMockDateLoc(date, locations);
        mockData.push(newData);

    }
    return mockData;

}

function main(){
    const count = 365;
    const p = "./mock/records.json"
    console.log(`Generating ${count} elements of mock data in ${p}`)

    let data = generateMockData("2025-08-10", count)
    fs.writeFileSync(p, JSON.stringify(data))

}
main();