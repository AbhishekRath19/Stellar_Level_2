<script lang="ts">
  import { wallet } from '$lib/wallet.svelte';
  import { LogIn, LogOut, Wallet, Shield } from 'lucide-svelte';

  function truncate(str: string | null) {
    if (!str) return '';
    return str.slice(0, 6) + '...' + str.slice(-4);
  }
</script>

<nav class="navbar">
  <div class="container">
    <div class="logo">
      <div class="logo-icon">
        <Shield size={24} color="var(--accent-primary)" />
      </div>
      <span class="logo-text">Stellar <span class="gradient-text">Yellow Belt</span></span>
    </div>

    <div class="actions">
      {#if wallet.address}
        <div class="wallet-badge glass-card">
          <Wallet size={16} />
          <span>{truncate(wallet.address)}</span>
        </div>
        <button class="btn-secondary" onclick={() => wallet.disconnect()}>
          <LogOut size={18} />
          <span>Disconnect</span>
        </button>
      {:else}
        <button class="btn-primary" onclick={() => wallet.connect()} disabled={wallet.isConnecting}>
          {#if wallet.isConnecting}
            <div class="spinner"></div>
            <span>Connecting...</span>
          {:else}
            <LogIn size={18} />
            <span>Connect Wallet</span>
          {/if}
        </button>
      {/if}
    </div>
  </div>
</nav>

<style>
  .navbar {
    padding: 1.5rem 2rem;
    position: sticky;
    top: 0;
    z-index: 100;
    background: rgba(15, 23, 42, 0.8);
    backdrop-filter: blur(8px);
    border-bottom: 1px solid var(--border-glass);
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.25rem;
    font-weight: 700;
  }

  .logo-icon {
    background: rgba(252, 211, 77, 0.1);
    padding: 0.5rem;
    border-radius: 0.75rem;
    display: flex;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .wallet-badge {
    padding: 0.5rem 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-family: monospace;
    font-size: 0.875rem;
    background: rgba(255, 255, 255, 0.05);
  }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(0,0,0,0.3);
    border-top: 2px solid #000;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>
