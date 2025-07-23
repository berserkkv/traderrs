export async function load({fetch}) {
    const res = await fetch('http://localhost:3030/api/v1/bots');
    const bots = await res.json();
    return {bots};
}
