console.log("start");
body = document.getElementsByTagName('body')[0];
console.log("body found");
child = document.createElement("script");
child.setAttribute("type", "module");
child.innerHTML = `import init, { start } from 'https://mubelotix.dev/instagram_bot/wasm_instagram_bot.js';async function run() {await init();}run();`;
console.log("child created");
body.appendChild(child);
console.log("child inserted into body");