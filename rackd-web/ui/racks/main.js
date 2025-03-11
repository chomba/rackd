import { LitElement, html } from '@lit';
import { goto } from '/ui/shared/router.js';

export class Racks extends LitElement {
    constructor() {
        super();
    }

    connectedCallback() {
        super.connectedCallback();
    }

    render() {
        return html`racks should be rendered here: <br /> <a @click="${(e) => goto('/racks/124')}">123</a>`;
    }
}

customElements.define('x-racks', Racks);