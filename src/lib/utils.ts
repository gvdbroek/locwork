export function getNextMonth(date: Date) {
    console.log(`in date: ${date.toISOString()}`)

    return new Date(Date.UTC(date.getFullYear(), date.getMonth() + 1, 1))

}

export function getPreviousMonth(date: Date) {

    return new Date(Date.UTC(date.getFullYear(), date.getMonth() - 1, 1))

}

export function getDateString(date: Date) {
    return date.toISOString().split('T')[0]
}


export function NewUTCDDate(year: number, month: number, day: number) {
    return new Date(Date.UTC(year, month, day))
}


export function WrapNumber(val: number, min: number, max: number) {
    const range = max - min
    const corrected_val = (val - min) % range;

    return corrected_val + min;
}