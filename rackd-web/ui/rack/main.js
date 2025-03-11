import { LitElement, html, ifDefined, repeat } from '@lit';
import { RackNodes } from '/ui/rack-nodes/main.js';
import { RackWans } from '/ui/rack-wans/main.js';
import { RackLans } from '/ui/rack-lans/main.js';
import { goto } from '/ui/shared/router.js';
import { sleep } from '/ui/shared/util.js';

export class Rack extends LitElement {
    static properties = {
        rack: { state: true },
        asn: { attribute: 'asn' },
        selectedTab: { attribute: 'selected-tab' }
    };

    constructor() {
        super();
    }
    
    connectedCallback() {
        super.connectedCallback();
    }

    render() {
        return template.bind(this)();
    }

    async firstUpdated() {
        this.shadowRoot.querySelector('.tabs')?.addEventListener('click', this.onTabClick);
        this.rack = await this.getRack(this.asn);
        this.requestUpdate();
    }

    onTabClick(e) {
        let tab = e.target.classList.contains('tab') ? e.target : e.target.closest('.tab');
        if (tab && tab.hasAttribute('tab-name')) {
            let tabName = tab.getAttribute('tab-name');
            console.log(`tab clicked: ../${tabName}`);
            goto(`../${tabName}`);
        }
    }

    updated() {
        this.selectTab(this.selectedTab);
    }

    async getRack(asn) {
        await sleep(1000);
        return {
            id: 'xxx-x-x-x-',
            asn: `${asn}`,
            status: 'healthy',
            tags: {
                alias: 'PE15109',
                isp: 'Americanet',
                isp: 'Telefonica'
            }
        };
    }

    selectTab(name) {
        if (!name)
            return;
        let tab = this.shadowRoot.querySelector(`.tab[tab-name=${name}]`); 
        if (tab && !tab.classList.contains(".selected")) {
            this.shadowRoot.querySelectorAll(".tab").forEach(tab => tab.classList.remove('selected'));
            tab.classList.add('selected');
        }
    }
}

customElements.define('x-rack', Rack);

function template() {
    return html`
<link href="/ui/rack/styles.css" rel="stylesheet" />
<section class="rack-header">
    ${!this.rack ? html`
    <div class="rack-icon empty"></div>
    <div class="rack-data">
        <h1 class="asn empty"></h1>
        <ul class="settings-summary">
            <li>
                <ul class="tags empty">
                    <li></li>
                    <li></li>
                    <li></li>
                </ul>  
            </li>
        </ul>
    </div>
    <div class="actions">
        <div class="btn empty"></div>
        <div class="btn empty"></div>
    </div>
    ` : html`
    <div class="rack-icon" status="${this.rack.status}">
        <div class="graphics">
            <span></span>
            <span></span>
            <span></span>
        </div>
    </div>
    <div class="rack-data">
        <h1 class="asn">AS${this.asn}</h1>
        <ul class="settings-summary">
            <li>
                <ul class="tags">
                    <li>Alias:PE15109</li>
                    <li>ISP:Americatel</li>
                    <li>ISP:Telefonica</li>
                </ul>
            </li>
        </ul>
    </div>
    <div class="actions">
        <div class="btn go-back" @click="${(e) => goto('/racks')}">
            <span class="icon">arrow_back</span>
            <span class="txt">Racks</span>
        </div>
        <div class="btn highlight">
            <span class="txt">Add</span>
            <span class="icon">keyboard_arrow_down</span>
        </div>
    </div>
    `}
</section>
<div class="tabs">
  <div class="tab" tab-name="nodes">
    <span class="tab-icon">computer</span>
    <span class="tab-txt">Nodes</span>
  </div>
  <div class="tab-panel">
    <x-rack-nodes asn="${this.asn}"></x-rack-nodes>
  </div>
  <div class="tab" tab-name="lans">
    <span class="tab-icon">lan</span>
    <span class="tab-txt">LANs</span>
  </div>
  <div class="tab-panel">
    <x-rack-lans></x-rack-lans>
  </div>
  <div class="tab" tab-name="wans">
    <span class="tab-icon">globe_book</span>
    <span class="tab-txt">WANs</span>
  </div>
  <div class="tab-panel">
    <x-rack-wans></x-rack-wans>
  </div>
  <div class="tab" tab-name="nat">
    <span class="tab-icon">swap_horiz</span>
    <span class="tab-txt">NAT</span>
  </div>
  <div class="tab-panel"></div>
  <div class="tab" tab-name="firewall">
    <span class="tab-icon">local_police</span>
    <span class="tab-txt">Firewall</span>
  </div>
  <div class="tab-panel"></div>
  <div class="tab" tab-name="settings">
    <span class="tab-icon">settings_input_component</span>
    <span class="tab-txt">Settings</span>
  </div>
  <div class="tab-panel"></div>
  <div class="tab" tab-name="history">
    <span class="tab-icon">view_list</span>
    <span class="tab-txt">History</span>
  </div>
  <div class="tab-panel"></div>
</div>
`;
}