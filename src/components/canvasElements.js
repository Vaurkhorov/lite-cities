export default class Canvas {
    constructor() {
        this.root = document.querySelector("#canvas");
    }

    clear() {
        this.root.innerHTML = "";
    }

    staticElement(id, element, content, parentElement) {
        if (parentElement == null) {
            parentElement = this.root;
        }

        const e = document.createElement(element, { id: id });
        e.textContent = content;
        parentElement.appendChild(e);

        return id;
    }
    
    interactiveElement(id, element, content, event, callback, parentElement = this.root) {
        if (parentElement == null) {
            parentElement = this.root;
        }

        const e = document.createElement(element, { id: id });
        e.textContent = content;
        e.addEventListener(event, callback);
        parentElement.appendChild(e);

        return id
    }

    inputElement(id, element, content, event, callback, parentElement = this.root) {
        if (parentElement == null) {
            parentElement = this.root;
        }

        content = JSON.parse(content);

        const e = document.createElement(element, { id: id });
        e.placeholder = content["prompt"];
        parentElement.appendChild(e);

        const b = document.createElement("button", { id: id + "-button" });
        b.textContent = content["button_text"];
        b.addEventListener(event, callback);
        parentElement.appendChild(b);

        return id;
    }
}