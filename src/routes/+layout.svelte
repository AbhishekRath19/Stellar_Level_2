<script lang="ts">
  import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte';
  import ErrorNotification from '$lib/components/ErrorNotification.svelte';
  
  let { children } = $props();
</script>

<Navbar />
<ErrorNotification />

<main>
  {@render children()}
</main>

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
</style>
