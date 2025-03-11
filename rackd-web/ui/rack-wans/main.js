import { LitElement, html, repeat } from '@lit';
import { sleep } from '/ui/shared/util.js';

export class RackWans extends LitElement {
    static properties = {
        asn: { attribute: true },
        isps: { state: true }
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
        this.isps = await this.getIsps(this.asn);
        this.requestUpdate();
    }

    async getIsps(asn) {
        await sleep(1000);
        return [{
            id: 'xxxxx',
            name: 'movistar',
            trunk: {
                id: 1,
                name: 'trunk1'
            },
            vlan: 1919,
            status: 'up',
            connection: {
                kind: 'IPoE',
                ip: '4+6'
            },
            prefixes: [{
                name: 'movistar-prefix0',
                raw: [0x2a02, 0, 0, 0],
                length: 64
            }, {
                name: 'movistar-prefix1',
                raw: [0x2a02, 0, 0, 0],
                length: 64
            }],
            upstream: {
                throughput: 15,
                bandwidth: 200,
                unit: 'Mbps'
            },
            downstream: {
                throughput: 20,
                bandwidth: 200,
                unit: 'Mbps'
            },
            latency: {
                value: 5.20,
                unit: 'ms'
            },
            jitter: {
                value: 4.2,
                unit: 'ms'
            }
        }, {
            id: 'xxxxx',
            name: 'claro',
            trunk: {
                id: 1,
                name: 'trunk1'
            },
            vlan: 100,
            status: 'up',
            connection: {
                kind: 'PPPoE',
                ip: '4+6'
            },
            prefixes: [{
                name: 'claro-prefix0',
                raw: [0x2a02, 0, 0, 0],
                length: 64
            }, {
                name: 'claro-prefix1',
                raw: [0x2a02, 0, 0, 0],
                length: 64
            }],
            upstream: {
                throughput: 20,
                bandwidth: 300,
                unit: 'Mbps'
            },
            downstream: {
                throughput: 35,
                bandwidth: 300,
                unit: 'Mbps'
            },
            latency: {
                value: 5.20,
                unit: 'ms'
            },
            jitter: {
                value: 4.2,
                unit: 'ms'
            }
        }];
    }
}

customElements.define("x-rack-wans", RackWans);

function template() {
    return html`
<link rel="stylesheet" href="/ui/rack-wans/styles.css" />
<table class="table">
    <thead>
        <tr>
            <th>Name</th>
            <th>Trunk</th>
            <th>VLAN</th>
            <th>IPv6 Prefixes</th>
            <th></th>
            <th>Download</th>
            <th>Upload</th>
            <th>Latency</th>
            <th>Jitter</th>
        </tr>
    </thead>
    ${!this.isps ? html`
    <tbody>
        <tr>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
    </tbody>
    ` : html`
    <tbody>
        ${repeat(this.isps, isp => isp.id, (isp, idx) => html`
        <tr>  
            <td>
                <div class="wan" up>
                    <div class="wan-icon">
                        <span class="icon">globe_book</span>
                        <span class="availability">L2</span>
                    </div>
                    <div class="wan-data">
                        <div class="wan-name">${isp.name}</div>
                        <span class="wan-kind">IPv${isp.connection.ip}, ${isp.connection.kind}</span>
                    </div>
                </div>
            </td>
            <td>
                <span class="trunk">${isp.trunk.name}</span>
            </td>
            <td>
                <span class="vlan">${isp.vlan}</span>
            </td>
            <td>
                <ul class="ipv6-prefix">
                ${isp.prefixes.map(prefix => html`
                    <li>
                        <span class="alias">@${prefix.name}</span>
                    </li>
                `)}
                </ul>
            </td> 
            <td></td>
            <td>
                <div class="bandwidth-usage">
                    <span class="bar" style="--prog: ${isp.downstream.throughput / isp.downstream.bandwidth};"></span>
                    <div class="data">
                        <span>${isp.downstream.throughput}${isp.downstream.unit}</span>
                        <span>${isp.downstream.bandwidth}${isp.downstream.unit}</span>
                    </div>
                </div>
            </td>
            <td>
                <div class="bandwidth-usage">
                    <span class="bar" style="--prog: ${isp.upstream.throughput / isp.upstream.bandwidth};"></span>
                    <div class="data">
                        <span>${isp.upstream.throughput}${isp.upstream.unit}</span>
                        <span>${isp.upstream.throughput}${isp.upstream.unit}</span>
                    </div>
                </div>
            </td>
            <td>
                <span class="latency">${isp.latency.value}${isp.latency.unit}</span>
            </td>  
            <td>
                <span class="jitter">${isp.jitter.value}${isp.jitter.unit}</span>
            </td>  
        </tr>
        `)}
    </tbody>
    `}
</table>
`;
}