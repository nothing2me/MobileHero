interface Props {
    serverRunning: boolean;
    clientCount: number;
    ip: string;
    port: number;
    pin: string;
    qrCode: string;
    onStart: () => void;
    onStop: () => void;
    onSettings: () => void;
}

export default function Sidebar({
    serverRunning, clientCount, ip, port, pin, qrCode,
    onStart, onStop, onSettings
}: Props) {
    return (
        <div className="sidebar-col">
            {/* Status Card */}
            <div className="card">
                <div className="card-title">Server Status</div>
                <div className="status-row">
                    <div className={`status-dot ${serverRunning ? 'active' : ''}`} />
                    <div className={`status-text ${serverRunning ? 'active' : ''}`}>
                        {serverRunning ? 'RUNNING' : 'STOPPED'}
                    </div>
                </div>

                {serverRunning && (
                    <div className="info-grid">
                        <span className="info-label">Local IP</span>
                        <span className="info-value">{ip}</span>

                        <span className="info-label">Port</span>
                        <span className="info-value">{port}</span>

                        <span className="info-label">PIN</span>
                        <span className="info-value">{pin}</span>

                        <span className="info-label">Clients</span>
                        <span className="info-value accent-text">{clientCount}</span>
                    </div>
                )}
            </div>

            {/* QR Code */}
            {serverRunning && qrCode && (
                <div className="card" style={{ padding: 0 }}>
                    <div className="qr-container">
                        <img src={qrCode} alt="QR Code" className="qr-image" />
                        <div className="qr-hint">Scan to Connect</div>
                    </div>
                </div>
            )}

            {/* Controls */}
            <div className="card">
                <div className="card-title">Controls</div>
                <div className="sidebar-col" style={{ gap: '10px' }}>
                    {!serverRunning ? (
                        <button className="control-btn btn-start" onClick={onStart}>
                            ▶ START SERVER
                        </button>
                    ) : (
                        <button className="control-btn btn-stop" onClick={onStop}>
                            ⏹ STOP SERVER
                        </button>
                    )}

                    <button className="control-btn btn-settings" onClick={onSettings}>
                        ⚙ SETTINGS
                    </button>
                </div>
            </div>
        </div>
    );
}
