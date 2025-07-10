export interface DateLoc {
    date: string;
    location: string;
    working: boolean;
}



export function generateMockData(startDate:string, numDays:30) : DateLoc[]{
    const locations = ["home" , "work" , "otherwork"]
    
    let mockData: DateLoc[] = [];
    for (let index = 0; index < numDays; index++) {

        let date = new Date(startDate)
        date.setDate(date.getDate() - index)
        let workday = true;
        if (date.getDay() < 1){
            workday = false;
        }
        if (date.getDay() > 5){
            workday = false;
        }

        const idx = Math.floor((Math.random() * 1000) % 3)

        const newData : DateLoc = {
            date: date.toISOString().split('T')[0],
            location: locations[idx],
            working: workday

        }
        mockData.push(newData);
        
    }
    return mockData;

}
