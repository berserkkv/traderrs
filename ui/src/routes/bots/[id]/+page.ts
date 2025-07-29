import {API_BASE} from "$lib/config";

export async function load({params}) {
    const id = params.id;
    return {id};
}

