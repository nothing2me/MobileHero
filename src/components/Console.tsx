import { useEffect, useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

export default function Console() {
    const [logs, setLogs] = useState<{ time: string; msg: string; type: string }[]>([]);
    const bottomRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const unlisten = listen<string>('log', (event) => {
            const msg = event.payload;
            const time = new Date().toLocaleTimeString('en-US', { hour12: false });

            let type = 'norm';
            if (msg.includes('[ERROR]') || msg.includes('[X]')) type = 'log-e';
            else if (msg.includes('[WARN]')) type = 'log-w';
            else if (msg.includes('[+]') || msg.includes('[OK]')) type = 'log-i';

            setLogs((prev) => [...prev.slice(-100), { time, msg, type }]);
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [logs]);

    return (
        <div className="card console-card">
            <div className="card-title">System Log</div>
            <div className="console-output">
                {logs.map((log, i) => (
                    <div key={i} className={`log-line ${log.type}`}>
                        <span className="log-time">[{log.time}]</span>
                        {log.msg}
                    </div>
                ))}
                <div ref={bottomRef} />
            </div>
        </div>
    );
}
