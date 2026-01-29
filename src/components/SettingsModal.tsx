import { useState } from 'react';

interface Config {
    port: number;
    pin: string;
    key_bindings: Record<string, string>;
}

interface Props {
    isOpen: boolean;
    onClose: () => void;
    config: Config | null;
    onSave: (config: Config) => void;
}

const INSTRUMENTS = {
    'Guitar/Bass': ['green', 'red', 'yellow', 'blue', 'orange', 'strum_up', 'strum_down', 'whammy'],
    'Game': ['starpower', 'start', 'select', 'up', 'down', 'left', 'right'],
    'Drums': ['drum_red', 'drum_yellow', 'drum_blue', 'drum_orange', 'drum_green', 'drum_kick']
};

export default function SettingsModal({ isOpen, onClose, config, onSave }: Props) {
    const [localConfig, setLocalConfig] = useState<Config | null>(config);

    if (!isOpen || !localConfig) return null;

    const handleKeyChange = (key: string, value: string) => {
        setLocalConfig({
            ...localConfig,
            key_bindings: {
                ...localConfig.key_bindings,
                [key]: value
            }
        });
    };

    return (
        <div className="modal-overlay">
            <div className="modal-content">
                <div className="modal-header">
                    <h2>Settings</h2>
                    <button className="header-btn" onClick={onClose}>✕</button>
                </div>

                <div className="modal-body">
                    <div className="setting-group">
                        <div className="group-title">Connection</div>
                        <div className="input-row">
                            <span className="input-label">Server Port</span>
                            <input
                                className="input-field"
                                type="number"
                                value={localConfig.port}
                                onChange={(e) => setLocalConfig({ ...localConfig, port: parseInt(e.target.value) })}
                            />
                        </div>
                        <div className="input-row">
                            <span className="input-label">Security PIN</span>
                            <input
                                className="input-field"
                                maxLength={4}
                                value={localConfig.pin}
                                onChange={(e) => setLocalConfig({ ...localConfig, pin: e.target.value })}
                            />
                        </div>
                    </div>

                    {Object.entries(INSTRUMENTS).map(([group, keys]) => (
                        <div className="setting-group" key={group}>
                            <div className="group-title">{group}</div>
                            {keys.map(key => (
                                <div className="input-row" key={key}>
                                    <span className="input-label">
                                        {key.replace('drum_', '').replace('_', ' ').toUpperCase()}
                                    </span>
                                    <input
                                        className="input-field"
                                        value={localConfig.key_bindings[key] || ''}
                                        onKeyDown={(e) => {
                                            e.preventDefault();
                                            handleKeyChange(key, e.key);
                                        }}
                                        readOnly
                                        placeholder="Press Key"
                                    />
                                </div>
                            ))}
                        </div>
                    ))}
                </div>

                <div className="modal-footer">
                    <button className="control-btn btn-settings" style={{ width: 'auto' }} onClick={onClose}>
                        Cancel
                    </button>
                    <button
                        className="control-btn btn-start"
                        style={{ width: 'auto' }}
                        onClick={() => onSave(localConfig)}
                    >
                        Save Changes
                    </button>
                </div>
            </div>
        </div>
    );
}
