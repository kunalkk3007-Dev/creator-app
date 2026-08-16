interface WelcomeProps {
  onChooseFolder: () => void;
  busy: boolean;
}

export function Welcome({ onChooseFolder, busy }: WelcomeProps) {
  return (
    <main className="screen screen-center">
      <div className="welcome-card">
        <span className="pin">📌</span>
        <h1>Pinfolder</h1>
        <p>
          Every board you make lives as plain files in a folder you choose on
          this computer — no account, no cloud, nothing syncs unless you set
          that up yourself.
        </p>
        <button
          type="button"
          className="btn btn-primary"
          onClick={onChooseFolder}
          disabled={busy}
        >
          {busy ? "Waiting for folder…" : "Choose a folder to get started"}
        </button>
      </div>
    </main>
  );
}
