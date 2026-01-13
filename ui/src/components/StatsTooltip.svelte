<script>
  export let metrics;

  function formatPercent(value) {
    return value ? value.toFixed(1) : '0.0';
  }

  function formatMB(value) {
    if (!value) return '0';
    if (value >= 1024) {
      return `${(value / 1024).toFixed(2)} GB`;
    }
    return `${value} MB`;
  }

  $: ramPercent = formatPercent(metrics?.ram_percent);
  $: cpuPercent = formatPercent(metrics?.cpu_percent);
  $: diskJunkPercent = formatPercent(metrics?.disk_junk_percent);
  $: diskJunkMB = formatMB(metrics?.disk_junk_mb);
</script>

<div class="tooltip">
  <div class="tooltip-title">System Status</div>
  <div class="tooltip-stats">
    <div class="stat-row">
      <span class="stat-label">RAM:</span>
      <span class="stat-value">{ramPercent}%</span>
    </div>
    <div class="stat-row">
      <span class="stat-label">CPU:</span>
      <span class="stat-value">{cpuPercent}%</span>
    </div>
    <div class="stat-row">
      <span class="stat-label">Junk:</span>
      <span class="stat-value">{diskJunkMB} ({diskJunkPercent}%)</span>
    </div>
  </div>
</div>

<style>
  .tooltip {
    position: absolute;
    top: -10px;
    left: 50%;
    transform: translate(-50%, -100%);
    background: rgba(0, 0, 0, 0.9);
    color: white;
    padding: 12px 16px;
    border-radius: 8px;
    font-size: 13px;
    white-space: nowrap;
    pointer-events: none;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    z-index: 1000;
  }

  .tooltip-title {
    font-weight: 600;
    margin-bottom: 8px;
    font-size: 14px;
    text-align: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.2);
    padding-bottom: 6px;
  }

  .tooltip-stats {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .stat-row {
    display: flex;
    justify-content: space-between;
    gap: 16px;
  }

  .stat-label {
    color: rgba(255, 255, 255, 0.7);
  }

  .stat-value {
    font-weight: 500;
    font-family: 'Courier New', monospace;
  }
</style>
