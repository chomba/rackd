import { LitElement, html, ifDefined } from '@lit';

export class Sidebar extends LitElement {
    constructor() {
        super();
        // this.pathMappings = new Map();
        // this.pathMappings.set('/racks', '<app-racks></app-racks>');
        // this.pathMappings.set('/racks/$rackId', '<app-rack selected-tab="${tab}"></app-rack');
    }

    _items() {
        return Array.from(this.shadowRoot.querySelectorAll(".nav-item"))
    }

    connectedCallback() {
        super.connectedCallback();
    }

    render() {
        return template.bind(this)();
    }

    createRenderRoot() {
        const root = super.createRenderRoot();
        root.addEventListener('click', this._onclick.bind(this));
        return root;
    }

    reset() {
        this._items().forEach(i => i.removeAttribute('selected'));
        document.querySelector('main').innerHTML = '';
    }

    select(item) {
        // refactor
        this.reset();
        document.querySelector('main').appendChild(document.createElement(item.dataset.view));
        item.setAttribute('selected', ''); 
    }

    _onclick(e) {
        console.log('clicked');
        console.log(e.target);
        let item = e.target.closest('.nav-item');
        console.log(item);
        if (!item || item.hasAttribute('selected') || !item.dataset.view)
            return;
        console.log(item);
        this.select(item);
    }

    _renderRacksView() {

    }
}

customElements.define("x-sidebar", Sidebar);

function template() {
    return html`
<link href="/ui/sidebar/styles.css" rel="stylesheet" />
<aside class="aside">
    <nav class="nav">
        <div class="nav-item org-wrapper" role="heading">
            <div class="org-logo">
                <span class="top"></span>
                <span class="bottom">
                    <span class="left"></span>
                    <span class="right"></span>
                </span>
            </div>
        </div>
        <div class="nav-item" role="heading" @click="${this._renderRacksView}">
            <span class="material-symbols-outlined">stacks</span>
        </div>
        <div class="nav-item" role="heading">
           <span class="material-symbols-outlined">stream</span>
        </div>
        <div class="nav-item action-needed" role="heading">
            <span class="material-symbols-outlined">grid_view</span>
        </div>
        <div class="nav-item" role="heading" style="margin-top: auto;" >
            <span class="material-symbols-outlined">terminal</span>
        </div>
    </nav>
</aside>
`;
}