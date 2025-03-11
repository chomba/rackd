import { LitElement, html, repeat } from '@lit';

export class RackLans extends LitElement {
    static properties = {
        asn: { attribute: true },
        lans: { state: true }
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
        this.lans = await this.getLans(this.asn);
    }

    async getLans(asn) {

    }
}

customElements.define("x-rack-lans", RackLans);

function template() {
    return html`
<link rel="stylesheet" href="/ui/rack-lans/styles.css" />
<table class="table">
    <thead>
        <tr>
            <th>Name</th>
            <th>Trunk</th>
            <th>VLAN</th>
            <th>Subnet ID</th>
            <th>Gateway</th>
            <th>Traffic</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>
                <div class="network" node>
                    <div class="network-icon">
                        <span class="icon">lan</span>
                        <span class="kind"></span>
                    </div>
                    <div class="network-data">
                        <div class="network-name">nodes-trunk1</div>
                        <span class="network-kind">Nodes LAN</span>
                    </div>
                </div>
            </td>
            <td>
                Trunk1
            </td>
            <td>
                1919
            </td>
            <td>
                <ul class="ip-prefix">
                    <li>
                        <span>2a0f:85c1:083f:fc00::/64</span>
                    </li>
                </ul>
            </td>
            <td>-</td>
            <td>-</td>
        </tr>
        <tr>
            <td>
                <div class="network" user>
                    <div class="network-icon">
                        <span class="icon">lan</span>
                        <span class="kind"></span>
                    </div>
                    <div class="network-data">
                        <div class="network-name">users-primary</div>
                        <span class="network-kind">Users LAN</span>
                    </div>
                </div>
            </td>
            <td>
                Trunk1
            </td>
            <td>
                100
            </td>
            <td>
                <ul class="ip-prefix">
                    <li>
                        <span>2a0f:85c1:083f:fc00::/64</span>
                    </li>
                </ul>
            </td>
            <td>
                <span class="gateway-ip">FE80::FFFF</span>
            </td>
            <td>
                <ul class="traffic">
                    <li class="egress">
                        <span class="icon">north_east</span>
                        <span class="label">Egress:</span>
                        <span class="value">20Mbps</span>
                    </li>
                    <li class="ingress">
                        <span class="icon">south_west</span>
                        <span class="label">Ingress:</span>
                        <span class="value">23Mbps</span>
                    </li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>
                <div class="network" oobm>
                    <div class="network-icon">
                        <span class="icon">lan</span>
                        <span class="kind"></span>
                    </div>
                    <div class="network-data">
                        <div class="network-name">oobm-trunk1</div>
                        <span class="network-kind">OOBM LAN</span>
                    </div>
                </div>
            </td>
            <td>
                Trunk1
            </td>
            <td>
                911
            </td>
            <td>
                <ul class="ip-prefix">
                    <li>
                        <span>fd01::/64</span>
                    </li>
                </ul>
            </td>
            <td>
                <span class="gateway-ip">FE80::FFFF</span>
            </td>
            <td>
                <ul class="traffic">
                    <li class="egress">
                        <span class="icon">north_east</span>
                        <span class="label">Egress:</span>
                        <span class="value">12Kbps</span>
                    </li>
                    <li class="ingress">
                        <span class="icon">south_west</span>
                        <span class="label">Ingress:</span>
                        <span class="value">15Kbps</span>
                    </li>
                </ul>
            </td>
        </tr>
    </tbody>
</table>
`;
}