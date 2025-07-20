import {base} from '$app/paths';

export async function load({fetch}) {

    // Adjust the base depending on DEV or PROD mode
    let app_base: String = base;
    if (!import.meta.env.PROD) {
        app_base = "http://localhost:3030"
    }

    const url = app_base + "/hello?name=PageLoadFunction";
    const opts = {
        method: "get",
        headers: {"Content-Type": "application/json"}
    };
    const response = await fetch(url, opts);
    const hello = await response.text();

    return {
        hello
    };

}