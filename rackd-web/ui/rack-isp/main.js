class RackIsp extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    async connectedCallback() {
        const src = "./components/rack-isp/template.html";
        this.shadowRoot.innerHTML = await (await fetch(src)).text();
    }
}

customElements.define("app-rack-isp", RackIsp);