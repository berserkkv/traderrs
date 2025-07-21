export async function load({fetch}) {
    const res = await fetch('http://0.0.0.0:8080/api/v1/bots');
    const bots = await res.json();
    return {bots};
}

