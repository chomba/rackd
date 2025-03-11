import { Router } from '/ui/shared/router.js';
import { Rack } from '/ui/rack/main.js';

// extends Router(RouterElement)
export class RacksRouter extends HTMLElement {
    static observedAttributes = ['path'];

    get path() {
        return this.getAttribute('path');
    }

    set path(value) {
        this.setAttribute('path', value);
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name == 'path' && oldValue !== newValue) {
            this.router.exec(this.path);
        } 
    }

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.router = new Router()
            .route('/', () => this.render('x-racks'))
            .route('/$asn', args => {
                let rack = this.render('x-rack');
                rack.setAttribute('asn', args.asn);
                rack.setAttribute('selected-tab', 'nodes');
            })
            .route('/$asn/*', args => {
                console.log(`marched /$asn/* with:`);
                console.log(args);
                let rack = this.render('x-rack');
                rack.setAttribute('asn', args.asn);
                rack.setAttribute('selected-tab', args.path); 
            })
            // .route('/$asn/nodes', args => {
            //     let rack = this.render('x-rack');
            //     rack.asn = args.asn;
            //     rack.selectedTab = 'nodes'; 
            // })
            // .route('/$asn/nodes/$nodeName', args => {
            //     let rack = this.render('x-rack-node');
            //     rack.asn = args.asn;
            //     rack.nodeName = args.nodeName;
            // })
            .fallback(() => this.render('x-404'));
    }

    render(e) {
        let component = this.shadowRoot.querySelector(e);
        if (!component) {
            this.shadowRoot.innerHTML = '';
            component = document.createElement(e);
            this.shadowRoot.appendChild(component);
        }
        return component;
    }
}

customElements.define('x-racks-router', RacksRouter);