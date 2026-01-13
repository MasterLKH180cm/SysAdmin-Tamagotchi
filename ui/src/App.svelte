<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import Pet from './components/Pet.svelte';
  import ActionMenu from './components/ActionMenu.svelte';
  import StatsTooltip from './components/StatsTooltip.svelte';

  let petState = 'Happy';
  let petEmoji = '😊';
  let petDescription = 'Pet is happy - system healthy!';
  let metrics = null;
  let showMenu = false;
  let showTooltip = false;

  let unlistenMetrics;

  onMount(async () => {
    // Initial fetch of metrics
    await fetchMetrics();

    // Listen for background polling updates
    unlistenMetrics = await listen('metrics-update', (event) => {
      const data = event.payload;
      petState = data.pet_state;
      petEmoji = data.pet_emoji;
      petDescription = data.pet_description;
      metrics = data.metrics;
    });
  });

  onDestroy(() => {
    if (unlistenMetrics) {
      unlistenMetrics();
    }
  });

  async function fetchMetrics() {
    try {
      const response = await invoke('get_metrics');
      petState = response.pet_state;
      petEmoji = response.pet_emoji;
      petDescription = response.pet_description;
      metrics = response.metrics;
    } catch (error) {
      console.error('Failed to fetch metrics:', error);
    }
  }

  function toggleMenu() {
    showMenu = !showMenu;
    if (showMenu) {
      showTooltip = false; // Close tooltip when opening menu
    }
  }

  function handlePetHover(event) {
    showTooltip = event.detail.hovered;
    if (showTooltip) {
      showMenu = false; // Close menu when showing tooltip
    }
  }

  async function handleCleanup() {
    try {
      const response = await invoke('cleanup_temp');
      alert(response.message);
      // Refresh metrics after cleanup
      await fetchMetrics();
      showMenu = false;
    } catch (error) {
      console.error('Cleanup failed:', error);
      alert('Cleanup failed: ' + error);
    }
  }

  function handleCloseMenu() {
    showMenu = false;
  }
</script>

<div class="app-container">
  <Pet
    emoji={petEmoji}
    state={petState}
    on:hover={handlePetHover}
    on:click={toggleMenu}
  />

  {#if showTooltip && metrics}
    <StatsTooltip {metrics} />
  {/if}

  {#if showMenu}
    <ActionMenu
      on:cleanup={handleCleanup}
      on:close={handleCloseMenu}
    />
  {/if}
</div>

<style>
  .app-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background: transparent;
  }

  /* Allow click-through on transparent areas */
  :global(body) {
    -webkit-app-region: drag;
  }

  :global(button), :global(.clickable) {
    -webkit-app-region: no-drag;
  }
</style>
