// Based off: https://googlechromelabs.github.io/howto-components/howto-tabs/#demo
import { Guid } from '../../ui/shared/util.js';

export class Tabs extends HTMLElement {
    constructor() {
        super();
        this._onSlotChange = this._onSlotChange.bind(this);
        this.attachShadow({ mode: 'open' });
    }

    async connectedCallback() {
        const src = '/ui/tabs/template.html';
        this.shadowRoot.innerHTML = await (await fetch(src)).text();
        this._tabSlot = this.shadowRoot.querySelector('slot[name=tab]');
        this._panelSlot = this.shadowRoot.querySelector('slot[name=panel]');
        if (!this.hasAttribute('role'))
            this.setAttribute('role', 'tablist');

        this.addEventListener('click', this._onClick);

        // Link Panels when a new tabs are programmatically addded
        this._tabSlot.addEventListener('slotchange', this._onSlotChange);
        this._panelSlot.addEventListener('slotchange', this._onSlotChange);

        // Temporary Fix: Fires LinkPanels when an element is upgraded by the parser
        Promise.all([
            customElements.whenDefined('app-tab'),
            customElements.whenDefined('app-panel')
        ]).then(_ => this._linkPanels());
    }

    allTabs() {
        return Array.from(this.querySelectorAll('app-tab'));
    }

    allPanels() { 
        return Array.from(this.querySelectorAll('app-panel'));
    }

    disconnectedCallback() {
        this.removeEventListener('click', this._onClick);
    }

    _linkPanels() {
        const tabs = this.allTabs();
        tabs.forEach(tab => {
            const panel = tab.nextElementSibling;
            if (panel.tagName.toLowerCase() !== 'app-panel') {
                console.error(`Tab #${tab.id} is not a sibling of a <app-panel>`);
                return;
            }

            tab.setAttribute('aria-controls', panel.id);
            panel.setAttribute('aria-labelledby', tab.id);
        });

        const selectedTab = tabs.find(tab => tab.selected) || tabs[0];
        this.selectTab(selectedTab);
    }

    _onSlotChange() {
        this._linkPanels();
    }

    _onClick(e) {
        let tab = e.target.closest("app-tab");
        if (!tab || tab.getAttribute('role') !== 'tab') 
            return;
        this.selectTab(tab);
    }

    selectTab(tab) {
        console.log(`select ${tab}`);
        const panel = this._panelFor(tab);
        if (!panel)
            throw new Error(`No panel with id ${tab.getAttribute('aria-controls')}`);
        this.reset();
        tab.selected = true;
        panel.hidden = false;
        tab.focus();
    }

    reset() {
        this.allTabs().forEach(tab => tab.selected = false);
        this.allPanels().forEach(panel => panel.hidden = true);
    }

    _panelFor(tab) {
        const panelId = tab.getAttribute('aria-controls');
        return this.querySelector(`#${panelId}`);
    }
}

class Tab extends HTMLElement {
    static get observedAttributes() {
        return ['selected'];
    }

    constructor() {
        super();
    }

    get selected() {
        return this.hasAttribute('selected');
    }

    set selected(value) {
        value = Boolean(value);
        if (value) {
            this.setAttribute('selected', '');
        } else {
            this.removeAttribute('selected');
        }
    }

    get handle() {
        return this.getAttribute('handle');
    }

    set handle(value) {
        this.setAttribute('handle', value);
    }

    connectedCallback() {
        this.setAttribute('role', 'tab');
        if (!this.id)
            this.id = `app-tab-${Guid.new()}`;
        this.setAttribute('aria-selected', 'false');
        this.setAttribute('tabindex', -1);
    }

    attributeChangedCallback() {
        const isSelected = this.hasAttribute('selected');
        console.log(`is selected: ${isSelected}`)
        this.setAttribute('aria-selected', isSelected);
        this.setAttribute('tabindex', isSelected ? 0 : -1);
    }
}

class Panel extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.setAttribute('role', 'tabpanel');
        if (!this.id)
            this.id = `app-tabpanel-${Guid.new()}`;
    }
}

(function() {
    window.customElements.define("app-tabs", Tabs);
    window.customElements.define("app-tab", Tab);
    window.customElements.define("app-panel", Panel);
})();