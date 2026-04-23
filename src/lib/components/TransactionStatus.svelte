<script lang="ts">
  import { wallet } from '$lib/wallet.svelte';
  import { 
    Loader2, 
    CheckCircle2, 
    XCircle, 
    ExternalLink,
    Clock
  } from 'lucide-svelte';

  function truncate(str: string) {
    return str.slice(0, 10) + '...' + str.slice(-10);
  }
</script>

{#if wallet.txStatus !== 'IDLE'}
  <div class="status-card glass-card animate-fade-in" class:success={wallet.txStatus === 'SUCCESS'} class:error={wallet.txStatus === 'ERROR'}>
    <div class="status-header">
      {#if wallet.txStatus === 'PENDING'}
        <div class="icon-spin">
          <Loader2 size={24} color="var(--accent-primary)" />
        </div>
        <div class="text">
          <h4>Transaction Pending</h4>
          <p>Processing on Stellar Network...</p>
        </div>
      {:else if wallet.txStatus === 'SUCCESS'}
        <CheckCircle2 size={24} color="#10b981" />
        <div class="text">
          <h4>Transaction Successful</h4>
          <p>The contract state has been updated.</p>
        </div>
      {:else if wallet.txStatus === 'ERROR'}
        <XCircle size={24} color="#f87171" />
        <div class="text">
          <h4>Transaction Failed</h4>
          <p>{wallet.txError || wallet.error || 'Something went wrong during submission.'}</p>
        </div>
      {/if}
    </div>

    {#if wallet.txHash}
      <div class="status-body">
        <div class="tx-hash">
          <Clock size={14} />
          <span>Hash: <span class="mono">{truncate(wallet.txHash)}</span></span>
        </div>
        <a 
          href="https://stellar.expert/explorer/testnet/tx/{wallet.txHash}" 
          target="_blank" 
          class="btn-secondary sm"
        >
          View on Explorer <ExternalLink size={14} />
        </a>
      </div>
    {/if}
  </div>
{/if}

<style>
  .status-card {
    margin-top: 1.5rem;
    padding: 1.5rem;
    border-left: 4px solid var(--accent-primary);
    background: rgba(15, 23, 42, 0.4);
  }

  .status-card.success {
    border-left-color: #10b981;
  }

  .status-card.error {
    border-left-color: #f87171;
  }

  .status-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .status-header h4 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
  }

  .status-header p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--text-dim);
  }

  .status-body {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 1rem;
    border-top: 1px solid var(--border-glass);
    gap: 1rem;
  }

  .tx-hash {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-dim);
  }

  .btn-secondary.sm {
    padding: 0.4rem 0.8rem;
    font-size: 0.75rem;
    border-radius: 0.5rem;
  }

  .icon-spin {
    animation: spin 2s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
