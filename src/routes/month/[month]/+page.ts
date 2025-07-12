import {generateMockData, type DateLoc} from '$lib/models'
import type { PageLoad } from './$types'

export const load: PageLoad = ({ params }) => {
    return {
        monthData: generateMockData(new Date().toISOString(), 30)

    };
};