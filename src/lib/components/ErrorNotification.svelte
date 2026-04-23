<script lang="ts">
  import { wallet } from '$lib/wallet.svelte';
  import { 
    AlertCircle, 
    X, 
    RefreshCcw, 
    Download,
    ShieldAlert,
    Coins
  } from 'lucide-svelte';

  let { onRetry } = $props<{ onRetry?: () => void }>();

  interface ErrorConfig {
    title: string;
    message: string;
    icon: any;
    color: string;
    actionLabel: string;
    actionUrl?: string;
  }

  const errorConfigs: Record<string, ErrorConfig> = {
    WALLET_NOT_INSTALLED: {
      title: 'Wallet Not Found',
      message: 'You need a Stellar wallet extension (like Freighter) to interact with this app.',
      icon: Download,
      color: '#fbbf24',
      actionLabel: 'Install Freighter',
      actionUrl: 'https://www.freighter.app/'
    },
    USER_REJECTED: {
      title: 'Transaction Rejected',
      message: 'You cancelled the transaction request in your wallet.',
      icon: ShieldAlert,
      color: '#f87171',
      actionLabel: 'Try Again'
    },
    INSUFFICIENT_BALANCE: {
      title: 'Insufficient Balance',
      message: 'You do not have enough XLM in your testnet account to cover the transaction fees.',
      icon: Coins,
      color: '#f87171',
      actionLabel: 'Fund Account',
      actionUrl: 'https://laboratory.stellar.org/#account-creator?network=testnet'
    },
    CONTRACT_NOT_FOUND: {
      title: 'Contract Not Found',
      message: 'The smart contract could not be found on the network. Please verify the ID.',
      icon: AlertCircle,
      color: '#f87171',
      actionLabel: 'Verify ID'
    },
    UNKNOWN_ERROR: {
      title: 'Something went wrong',
      message: 'An unexpected error occurred. Please check your network and try again.',
      icon: AlertCircle,
      color: '#94a3b8',
      actionLabel: 'Retry'
    }
  };

  const currentError = $derived(errorConfigs[wallet.error as keyof typeof errorConfigs] || errorConfigs.UNKNOWN_ERROR);

  function handleAction() {
    if (currentError.actionUrl) {
      window.open(currentError.actionUrl, '_blank');
    } else if (onRetry) {
      wallet.error = null;
      onRetry();
    } else {
      wallet.error = null;
    }
  }
</script>

{#if wallet.error}
  <div class="notification glass-card animate-fade-in" style="border-left-color: {currentError.color}">
    <div class="content">
      <div class="icon-wrapper" style="background: {currentError.color}20">
        <svelte:component this={currentError.icon} size={20} color={currentError.color} />
      </div>
      <div class="text">
        <h5>{currentError.title}</h5>
        <p>{currentError.message}</p>
      </div>
    </div>

    <div class="actions">
      <button class="btn-primary sm" onclick={handleAction}>
        {#if !currentError.actionUrl}
          <RefreshCcw size={14} />
        {/if}
        {currentError.actionLabel}
      </button>
      <button class="close-btn" onclick={() => wallet.error = null}>
        <X size={18} />
      </button>
    </div>
  </div>
{/if}

<style>
  .notification {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    width: 400px;
    padding: 1.25rem;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1.5rem;
    z-index: 1000;
    border-left: 4px solid transparent;
    background: rgba(15, 23, 42, 0.95);
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
  }

  .content {
    display: flex;
    gap: 1rem;
  }

  .icon-wrapper {
    padding: 0.75rem;
    border-radius: 0.75rem;
    display: flex;
    height: fit-content;
  }

  .text h5 {
    margin: 0 0 0.25rem 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .text p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--text-dim);
    line-height: 1.4;
  }

  .actions {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 1rem;
  }

  .btn-primary.sm {
    padding: 0.5rem 1rem;
    font-size: 0.8125rem;
    white-space: nowrap;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    padding: 0.25rem;
    transition: color 0.2s;
  }

  .close-btn:hover {
    color: white;
  }

  @media (max-width: 480px) {
    .notification {
      left: 1rem;
      right: 1rem;
      bottom: 1rem;
      width: auto;
    }
  }
</style>
