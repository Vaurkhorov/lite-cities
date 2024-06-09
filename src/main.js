import { toggleSidebar } from "./components/sidebar.js";
import Canvas from "./components/canvasElements.js";

const { invoke } = window.__TAURI__.tauri;


window.addEventListener("DOMContentLoaded", () => {
	document.querySelector("#sidebar-toggle").addEventListener("click", () => {
		toggleSidebar();
	});

	const canvas = new Canvas();
	gameLoop(canvas);
});

let sampleResponse = {
	queue: [
		{
			command: "staticElement",
			id: "title",
			element: "h1",
			content: "Hello, world!"
		},
		{
			command: "interactiveElement",
			id: "button",
			element: "button",
			content: "Click me!",
			event: "click",
		},
		{
			command: "exit"
		}
	]
}

async function gameLoop(canvas) {
	let active = true;
	let state = {
		element_id: "#canvas",
		returned_data: null
	};
	while (active) {
		let promises = [];

		invoke("game_loop", { state: state })
			.then((response) => {
				for (let item of response.queue) {
					switch (item.command) {
						case "staticElement":
							canvas.staticElement(item.id, item.element, item.content);
							break;
						case "interactiveElement":
							promises.push(new Promise(
								(resolve) => {
									canvas.interactiveElement(item.id, item.element, item.content, item.event, resolve);
								}
							));
							break;
						case "inputElement":
							promises.push(new Promise(
								(resolve) => {
									canvas.inputElement(item.id, item.element, item.content, item.event, resolve);
								}
							));
							break;
						case "exit":
							active = false;
							break;
					}
				}
		});

		let sharedPromise = Promise.any(promises);
		await sharedPromise.then((response) => {
			state = response;
		});
	}
}

