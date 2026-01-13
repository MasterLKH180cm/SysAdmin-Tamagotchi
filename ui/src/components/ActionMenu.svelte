<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  function handleCleanup() {
    dispatch('cleanup');
  }

  function handleClose() {
    dispatch('close');
  }
</script>

<div class="menu-overlay" on:click={handleClose}></div>
<div class="menu clickable">
  <div class="menu-title">Actions</div>
  <button class="menu-button cleanup" on:click={handleCleanup}>
    🧹 Clean Temp Files
  </button>
  <button class="menu-button close" on:click={handleClose}>
    ❌ Close
  </button>
</div>

<style>
  .menu-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: transparent;
    z-index: 999;
  }

  .menu {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: rgba(255, 255, 255, 0.95);
    border-radius: 12px;
    padding: 16px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
    min-width: 200px;
    z-index: 1000;
    backdrop-filter: blur(10px);
  }

  .menu-title {
    font-weight: 600;
    font-size: 16px;
    margin-bottom: 12px;
    text-align: center;
    color: #333;
  }

  .menu-button {
    width: 100%;
    padding: 10px 16px;
    margin: 6px 0;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .menu-button.cleanup {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .menu-button.cleanup:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }

  .menu-button.close {
    background: #f0f0f0;
    color: #666;
  }

  .menu-button.close:hover {
    background: #e0e0e0;
  }

  .menu-button:active {
    transform: translateY(0);
  }
</style>
