<script lang="ts">
    import { PUBLIC_CONTRACT_ID } from '$lib/constants';
  import { wallet } from '$lib/wallet.svelte';
  import TransactionStatus from '$lib/components/TransactionStatus.svelte';
  import { 
    Zap, 
    ShieldAlert, 
    CheckCircle2, 
    ArrowRight,
    ExternalLink,
    Copy,
    Fingerprint,
    Hash,
    Heart,
    TrendingUp,
    Users,
    Activity,
    Coins,
    RefreshCw,
    Info,
    Lock
  } from 'lucide-svelte';
  import { onMount, onDestroy } from 'svelte';
  import * as StellarSdk from '@stellar/stellar-sdk';

  const contractId = PUBLIC_CONTRACT_ID;
  
  // Crowdfund State
  let totalAmount = $state(0);
  let goalAmount = $state(0);
  let donationAmount = $state(10);
  let donors = $state<any[]>([]);

  let progressPercentage = $derived(goalAmount > 0 ? Math.min((totalAmount / goalAmount) * 100, 100) : 0);

  async function handleDonate() {
    try {
      const args = [
        new StellarSdk.Address(wallet.address!).toScVal(),
        StellarSdk.nativeToScVal(donationAmount, { type: 'u32' })
      ];
      await wallet.callContract(contractId, 'donate', args);
      wallet.startEventListener(contractId);
      await fetchData();
    } catch (e) {
      console.error('Donation Error:', e);
    }
  }

  async function fetchData() {
    if (!wallet.address) return;
    try {
        const server = new StellarSdk.rpc.Server('https://soroban-testnet.stellar.org');
        const contract = new StellarSdk.Contract(contractId);
        
        // Fetch Total
        const totalSim = await server.simulateTransaction(
            new StellarSdk.TransactionBuilder(new StellarSdk.Account(wallet.address, "0"), {
                fee: '100',
                networkPassphrase: StellarSdk.Networks.TESTNET
            }).addOperation(contract.call('get_total')).setTimeout(30).build()
        );
        if (StellarSdk.rpc.Api.isSimulationSuccess(totalSim) && totalSim.result) {
            totalAmount = StellarSdk.scValToNative(totalSim.result.retval);
        }

        // Fetch Goal
        const goalSim = await server.simulateTransaction(
            new StellarSdk.TransactionBuilder(new StellarSdk.Account(wallet.address, "0"), {
                fee: '100',
                networkPassphrase: StellarSdk.Networks.TESTNET
            }).addOperation(contract.call('get_goal')).setTimeout(30).build()
        );
        if (StellarSdk.rpc.Api.isSimulationSuccess(goalSim) && goalSim.result) {
            goalAmount = StellarSdk.scValToNative(goalSim.result.retval);
        }

        // Fix Event Parsing
        donors = wallet.events
            .filter(e => e.contractId === contractId)
            .map(e => {
                try {
                    const topics = e.topic.map((t: any) => StellarSdk.scValToNative(t));
                    // Match ["fund", "donate"]
                    if (topics[0] === 'fund' && topics[1] === 'donate') {
                        const val = StellarSdk.scValToNative(e.value);
                        return {
                            address: val[0], // donor
                            amount: val[1]   // amount
                        };
                    }
                } catch (err) {
                    return null;
                }
                return null;
            })
            .filter(d => d !== null);
            
    } catch (e) {
        console.error('Fetch Data Error:', e);
    }
  }

  onMount(() => {
    window.addEventListener('stellar-event', fetchData);
    wallet.startEventListener(contractId);
    if (wallet.address) fetchData();
  });

  onDestroy(() => {
    window.removeEventListener('stellar-event', fetchData);
    wallet.stopEventListener();
  });
</script>

<div class="dashboard">
  <header class="hero">
    <h1 class="gradient-text">Stellar Fundraiser</h1>
    <p class="subtitle">Support innovative projects on the Stellar network. Every donation counts.</p>
  </header>

  <div class="grid stats-grid">
    <div class="stat-card glass-card">
      <div class="stat-icon"><TrendingUp size={20} color="var(--accent-primary)" /></div>
      <div class="stat-info">
        <span class="stat-label">Total Raised</span>
        <span class="stat-value">{totalAmount} XLM</span>
      </div>
    </div>
    <div class="stat-card glass-card">
      <div class="stat-icon"><Users size={20} color="var(--accent-primary)" /></div>
      <div class="stat-info">
        <span class="stat-label">Donors</span>
        <span class="stat-value">{donors.length}</span>
      </div>
    </div>
    <div class="stat-card glass-card">
      <div class="stat-icon"><Activity size={20} color="var(--accent-primary)" /></div>
      <div class="stat-info">
        <span class="stat-label">Progress</span>
        <span class="stat-value">{Math.round(progressPercentage)}%</span>
      </div>
    </div>
  </div>

  <section class="main-content">
    <div class="card glass-card campaign-section">
      {#if !wallet.address}
        <div class="connect-prompt">
          <Fingerprint size={48} color="var(--accent-primary)" />
          <h2>Connect Your Wallet</h2>
          <p>You must be connected to donate to this campaign.</p>
          <button class="btn-primary" onclick={() => wallet.connect()}>
            Connect Wallet <ArrowRight size={18} />
          </button>
        </div>
      {:else}
        <div class="campaign-content">
          <div class="campaign-header">
            <h3>Support the Yellow Belt Project</h3>
            <p>We are raising funds to build the best educational platform for Stellar developers.</p>
          </div>

          <div class="progress-container">
            <div class="progress-labels">
              <span>{totalAmount} XLM raised</span>
              <span>Goal: {goalAmount} XLM</span>
            </div>
            <div class="progress-bar-lg">
              <div class="progress-fill-lg" style="width: {progressPercentage}%"></div>
            </div>
          </div>

          <div class="input-group-row">
            <div class="input-wrap">
              <label for="contractId">Active Contract ID</label>
              <div class="contract-badge mono">
                <Lock size={14} />
                <span>{contractId.slice(0, 8)}...{contractId.slice(-8)}</span>
              </div>
            </div>
            <button class="btn-secondary sm" onclick={fetchData}>
              <RefreshCw size={14} />
              Sync
            </button>
          </div>

          <div class="donation-form glass-card">
            <div class="form-title">
              <Heart size={18} color="#f87171" />
              <h4>Make a Donation</h4>
            </div>
            <div class="amount-selector">
              {#each [10, 50, 100, 500] as amount}
                <button 
                    class="amount-btn" 
                    class:active={donationAmount === amount}
                    onclick={() => donationAmount = amount}
                >
                  {amount}
                </button>
              {/each}
              <input type="number" bind:value={donationAmount} class="custom-amount" placeholder="Other" />
            </div>
            <button class="btn-primary w-full" onclick={handleDonate} disabled={wallet.isConnecting}>
              {#if wallet.isConnecting}
                <div class="spinner"></div>
                <span>Processing...</span>
              {:else}
                <Coins size={18} />
                <span>Donate {donationAmount} XLM</span>
              {/if}
            </button>
          </div>

          <TransactionStatus />
        </div>
      {/if}
    </div>

    <div class="info-grid">
      <div class="card glass-card donors-card">
        <div class="card-header">
          <Users size={20} color="var(--accent-primary)" />
          <h3>Recent Donors</h3>
        </div>
        <div class="donor-list">
          {#if donors.length === 0}
            <p class="empty-msg">No donations yet. Be the first!</p>
          {:else}
            {#each donors as donor}
              <div class="donor-item glass-card">
                <div class="donor-info">
                  <span class="donor-addr mono">{donor.address?.slice(0, 6)}...{donor.address?.slice(-4)}</span>
                  <span class="donor-amount">+{donor.amount} XLM</span>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <div class="card glass-card warning">
        <div class="card-header">
          <ShieldAlert size={20} color="#fbbf24" />
          <h3>Safety First</h3>
        </div>
        <p>This is a <strong>Testnet</strong> demo. No real XLM is being transferred. Use the Stellar Lab to get test tokens.</p>
      </div>
    </div>
  </section>
</div>

<style>
  .dashboard { display: flex; flex-direction: column; gap: 3rem; animation: fadeIn 0.6s ease-out; }
  .hero { text-align: center; max-width: 800px; margin: 0 auto; }
  h1 { font-size: 3.5rem; font-weight: 800; margin-bottom: 1rem; }
  .subtitle { font-size: 1.25rem; color: var(--text-dim); }
  
  .grid { display: grid; gap: 1.5rem; }
  .stats-grid { grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); }
  .stat-card { padding: 1.5rem; display: flex; align-items: center; gap: 1rem; }
  .stat-icon { background: rgba(252, 211, 77, 0.1); padding: 0.75rem; border-radius: 1rem; display: flex; }
  .stat-info { display: flex; flex-direction: column; }
  .stat-label { font-size: 0.875rem; color: var(--text-dim); text-transform: uppercase; }
  .stat-value { font-size: 1.125rem; font-weight: 600; }

  .main-content { display: grid; grid-template-columns: 1.5fr 1fr; gap: 2rem; }
  @media (max-width: 1000px) { .main-content { grid-template-columns: 1fr; } }

  .card { padding: 2.5rem; }
  .campaign-section { background: radial-gradient(circle at top right, rgba(252, 211, 77, 0.05), transparent), var(--bg-card); }
  
  .campaign-header h3 { font-size: 2rem; margin-bottom: 0.5rem; }
  .campaign-header p { color: var(--text-dim); margin-bottom: 2rem; }

  .progress-container { margin-bottom: 2.5rem; }
  .progress-labels { display: flex; justify-content: space-between; margin-bottom: 0.75rem; font-weight: 600; font-size: 0.875rem; }
  .progress-bar-lg { height: 12px; background: rgba(255, 255, 255, 0.05); border-radius: 6px; overflow: hidden; border: 1px solid var(--border-glass); }
  .progress-fill-lg { height: 100%; background: var(--accent-primary); border-radius: 6px; transition: width 1s ease; box-shadow: 0 0 15px var(--glow); }

  .input-group-row { display: flex; gap: 0.75rem; align-items: flex-end; margin-bottom: 1.5rem; }
  .input-wrap { flex: 1; display: flex; flex-direction: column; gap: 0.5rem; }
  .input-wrap label { font-size: 0.875rem; color: var(--text-dim); }
  
  .contract-badge {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: rgba(252, 211, 77, 0.1);
    border: 1px solid rgba(252, 211, 77, 0.2);
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    color: var(--accent-primary);
    font-size: 0.875rem;
    width: 100%;
  }

  .donation-form { padding: 2rem; border-color: rgba(248, 113, 113, 0.2); }
  .form-title { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1.5rem; }
  .form-title h4 { font-size: 1.25rem; margin: 0; }

  .amount-selector { display: grid; grid-template-columns: repeat(5, 1fr); gap: 0.5rem; margin-bottom: 1.5rem; }
  .amount-btn { background: rgba(255, 255, 255, 0.05); border: 1px solid var(--border-glass); padding: 0.75rem 0.25rem; border-radius: 0.5rem; color: white; cursor: pointer; transition: all 0.2s; font-weight: 600; }
  .amount-btn.active { background: var(--accent-primary); color: black; border-color: var(--accent-primary); }
  .custom-amount { background: rgba(255, 255, 255, 0.05); border: 1px solid var(--border-glass); padding: 0.75rem; border-radius: 0.5rem; color: white; width: 100%; text-align: center; }

  .donor-list { display: flex; flex-direction: column; gap: 0.75rem; margin-top: 1.5rem; }
  .donor-item { padding: 1rem; border-radius: 0.75rem; }
  .donor-info { display: flex; justify-content: space-between; align-items: center; }
  .donor-addr { font-size: 0.875rem; color: var(--text-dim); }
  .donor-amount { font-weight: 700; color: #10b981; }

  .w-full { width: 100%; justify-content: center; }
  .empty-msg { text-align: center; color: var(--text-dim); font-size: 0.875rem; padding: 2rem 0; }
  .mono { font-family: monospace; }

  .connect-prompt { text-align: center; padding: 3rem 0; display: flex; flex-direction: column; align-items: center; gap: 1rem; }
  .connect-prompt h2 { font-size: 1.5rem; margin: 0; }
  .connect-prompt p { color: var(--text-dim); }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(0,0,0,0.3);
    border-top: 2px solid #000;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .card-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1.5rem; }
  .card-header h3 { font-size: 1.25rem; margin: 0; }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
