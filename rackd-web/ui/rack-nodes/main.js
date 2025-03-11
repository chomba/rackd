import { LitElement, html, ifDefined, repeat } from '@lit';
import { sleep } from '/ui/shared/util.js';

export class RackNodes extends LitElement {
    static properties = {
        asn: { attribute: 'asn' },
        _nodes: { 
            state: true
        }
    };

    constructor() {
        super();
    }

    render() {
        return template.bind(this)();
    }

    async firstUpdated() {
        this._nodes = await this.getNodes(this.asn);
        this.requestUpdate();
    }

    connectedCallback() {
        super.connectedCallback();
    }

    async getNodes(asn) {
        await sleep(1000);
        return [{
            id: '01',
            name: 'node1',
            os: 'Debian 11',
            kernel: '5.15.0',
            arch: 'x86',
            addresses: [{
                prefix: {
                    network: 'nodes-trunk1',
                    raw: [0x2a02, 0, 0, 0],
                    length: 64
                },
                host: {
                    raw: [0, 0, 0, 0x1],
                    text: '::1'
                },
                raw: [0x2a02, 0, 0, 0, 0, 0, 0, 0x1],
                text: '2a02::1' 
            }, {
                prefix: {
                    network: 'nodes-trunk2',
                    raw: [0x2a01, 0, 0, 0],
                    length: 64
                },
                host: {
                    raw: [0, 0, 0, 0x1],
                    text: '::1'
                },
                raw: [0x2a01, 0, 0, 0, 0, 0, 0, 0x1],
                text: '2a01::1' 
            }],
            resources: {
                ram: {
                    usage: 4,
                    capacity: 12,
                    unit: 'GiB'
                },
                disk: {
                    usage: 6,
                    capacity: 12,
                    unit: 'GiB'
                },
                cpu: {
                    usage: 20,
                    capacity: 100,
                    unit: '%'
                }
            },
            status: 'online',
            rackd: {
                version: 1.1,
                role: 'leader' 
            }
        }, {
            id: '02',
            name: 'node2',
            os: 'Debian 11',
            kernel: '5.12.0',
            arch: 'x86',
            addresses: [{
                prefix: {
                    network: 'nodes-trunk1',
                    raw: [0x2a02, 0, 0, 0],
                    length: 64
                },
                host: {
                    raw: [0, 0, 0, 0x2],
                    text: '::2'
                },
                raw: [0x2a02, 0, 0, 0, 0, 0, 0, 0x2],
                text: '2a02::2' 
            }, {
                prefix: {
                    network: 'nodes-trunk2',
                    raw: [0x2a01, 0, 0, 0],
                    length: 64
                },
                host: {
                    raw: [0, 0, 0, 0x2],
                    text: '::2'
                },
                raw: [0x2a01, 0, 0, 0, 0, 0, 0, 0x2],
                text: '2a01::2' 
            }],
            resources: {
                ram: {
                    usage: 4,
                    capacity: 12,
                    unit: 'GiB'
                },
                disk: {
                    usage: 6,
                    capacity: 12,
                    unit: 'GiB'
                },
                cpu: {
                    usage: 20,
                    capacity: 100,
                    unit: '%'
                }
            },
            status: 'online',
            rackd: {
                version: 1.1,
                role: 'follower' 
            }
        }];
    }
}

customElements.define('x-rack-nodes', RackNodes);

function template() {
    return html`
<link href="/ui/rack-nodes/styles.css" rel="stylesheet" />
<table class="table">
    <thead>
        <tr>
            <th>Name</th>
            <th>IPv6 Address</th>
            <th>CPU</th>
            <th>RAM</th>
            <th>OS DISK</th>
            <th>RACKD</th>
            <th></th>
            <th>ACTIONS</th>
        </tr>
    </thead>
    ${!this._nodes ? html`
    <tbody class="damn">
        <tr>
            <td>
                <div class="node">
                    <div class="node-icon empty"></div>
                    <div class="node-data">
                        <div class="node-name empty"></div>
                        <div class="node-info empty"></div>
                    </div>
                </div>
            </td>
            <td>
                <ul class="ip-address-list">
                    <li>
                        <span class="prefix empty"></span>
                        <span class="iid empty"></span>
                    </li>
                    <li>
                        <span class="prefix empty"></span>
                        <span class="iid empty"></span>
                    </li>
                </ul>
            </td>
            <td>
                <div class="resource-usage">
                    <span class="bar empty"></span>
                    <div class="data">
                        <span></span>
                        <span></span>
                    </div>
                </div>
            </td>
            <td>
                <div class="resource-usage">
                    <span class="bar empty"></span>
                    <div class="data">
                        <span></span>
                        <span></span>
                    </div>
                </div>
            </td>
            <td>
                <div class="resource-usage">
                    <span class="bar empty"></span>
                    <div class="data">
                        <span></span>
                        <span></span>
                    </div>
                </div>
            </td>
            <td>
                <span class="rackd-version empty"></span>
            </td>
            <td></td>
            <td>
                <span class="actions empty">more_horiz</span>
            </td>
        </tr>
    </tbody>
    ` : html`
    <tbody>
        ${repeat(this._nodes, node => node.id, (node, idx) => html`
        <tr>
            <td>
                <div class="node" status=${ifDefined(node.status)}>
                    <div class="node-icon">
                        <span class="icon">computer</span>
                        ${node.rackd.role == 'leader' ? html`<span class="master">crown</span>` : undefined}
                    </div>
                    <div class="node-data">
                        <div class="node-name">${node.name}</div>
                        <span class="node-info">${node.os} (${node.arch}), kernel v${node.kernel}</span>
                    </div>
                </div>
            </td>
            <td>
                <ul class="ip-address-list">
                ${node.addresses.map(addr => html`
                    <li>
                        <span class="prefix">@${addr.prefix.network}</span>
                        <span class="iid">${addr.host.text}</span>
                    </li>`
                )}
                </ul>
            </td>
            <td>
                <div class="resource-usage cpu-usage">
                    <span class="bar" style="--prog: ${node.resources.cpu.usage / node.resources.cpu.capacity};"></span>
                    <div class="data">
                        <span>${node.resources.cpu.usage}${node.resources.cpu.unit}</span>
                        <span>${node.resources.cpu.capacity}${node.resources.cpu.unit}</span>
                    </div>
                </div>
            </td>
            <td>
                <div class="resource-usage ram-usage">
                    <span class="bar" style="--prog: ${node.resources.ram.usage / node.resources.ram.capacity};"></span>
                    <div class="data">
                        <span>${node.resources.ram.usage}${node.resources.ram.unit}</span>
                        <span>${node.resources.ram.capacity}${node.resources.ram.unit}</span>
                    </div>
                </div>
            </td>
            <td>
                <div class="resource-usage disk-usage">
                    <span class="bar" style="--prog: ${node.resources.disk.usage / node.resources.disk.capacity};"></span>
                    <div class="data">
                        <span>${node.resources.disk.usage}${node.resources.disk.unit}</span>
                        <span>${node.resources.disk.capacity}${node.resources.disk.unit}</span>
                    </div>
                </div>
            </td>
            <td>
                <span class="rackd-version">v1.1</span>
            </td>
            <td></td>
            <td>
                <span class="actions">more_horiz</span>
            </td>
        </tr>
        `)}
    </tbody>
    `}
</table>
`;
}