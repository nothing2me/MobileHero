import guitarIcon from '../assets/guitar-icon.png';

export default function Header() {
    return (
        <header className="header">
            <div className="brand">
                <img src={guitarIcon} alt="Logo" className="brand-icon-img" />
                <div>
                    <div className="brand-title">
                        MOBILE<span className="accent-text">HERO</span>
                    </div>
                    <div className="brand-subtitle">Launcher</div>
                </div>
            </div>
        </header>
    );
}
