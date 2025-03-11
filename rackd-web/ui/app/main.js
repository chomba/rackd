import { LitElement, html } from '@lit';
import { Router } from '/ui/shared/router.js';
import { Rack } from '/ui/rack/main.js';
import { Racks } from '/ui/racks/main.js';

export class App extends LitElement {
    static properties = {
        route: { state: true }
    };

    constructor() {
        super();
        this.router = new Router()
            .route('/racks/', () => this.load('x-racks'))
            .redirect('/racks/$asn/', '/racks/$asn/nodes')
            .route('/racks/$asn/$tab', (args) => {
                let rack = this.load('x-rack');
                rack.setAttribute('asn', args.asn);
                rack.setAttribute('selected-tab', args.tab);
            })
            .fallback(() => {
                console.log(this.shadowRoot);
                this.shadowRoot.querySelector('.main-wrapper > main').innerHTML = "NOT FOUND";
            });
    }

    get main() {
        return this.shadowRoot.querySelector('.main-wrapper > main');
    }

    connectedCallback() {
        super.connectedCallback();
    }

    render() {
        return template.bind(this)();
    }

    firstUpdated() {
        this.router.run();
    }

    load(e) {
        let main = this.main;
        let component = main.querySelector(e);
        if (!component) {
            main.innerHTML = '';
            component = document.createElement(e);
            main.appendChild(component);
        }
        return component;
    }
}

customElements.define('x-app', App);

function template() {
    return html`
<link rel="stylesheet" href="/ui/app/styles.css" />
<link rel="stylesheet" href="/ui/app/sidebar.css" />
<div class="app-wrapper">
    <section class="aside-wrapper">
        <aside class="aside">
            <nav class="nav" @click="${this.selectNavItem}">
                <div class="nav-item org-wrapper" role="heading">
                    <div class="org-logo">
                        <span class="top"></span>
                        <span class="bottom">
                            <span class="left"></span>
                            <span class="right"></span>
                        </span>
                    </div>
                </div>
                <div class="nav-item" role="heading">
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
    </section>
    <section class="topbar-wrapper">
        <div class="topbar">
            <div class="search">
                <span class="material-symbols-outlined">search</span>
                <span class="placeholder">Search by name or IP address (Cmd + B)</span>
            </div>
            <div class="shortcuts">
                <div class="notifications material-symbols-outlined attention">chat</div>
                <div class="settings material-symbols-outlined">settings</div>
                <span class="sep"></span>
                <div class="user">
                    <span class="avatar">
                        <span class="initials">JC</span>
                    </span>
                    <span class="btn-options"></span>
                </div>
            </div>
        </div>
    </section>
    <section class="main-wrapper">
        <main class="main"></main>
    </section>
</div>    
`;
}