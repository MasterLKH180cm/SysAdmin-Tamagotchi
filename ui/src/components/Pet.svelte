<script>
  import { createEventDispatcher } from 'svelte';
  import ParticleEffect from './ParticleEffect.svelte';

  export let emoji = '😊';
  export let state = 'Happy';

  const dispatch = createEventDispatcher();
  let isHovered = false;

  function handleMouseEnter() {
    isHovered = true;
    dispatch('hover', { hovered: true });
  }

  function handleMouseLeave() {
    isHovered = false;
    dispatch('hover', { hovered: false });
  }

  function handleClick() {
    dispatch('click');
  }

  // Determine animation class based on state
  $: animationClass = getAnimationClass(state);

  function getAnimationClass(state) {
    switch (state) {
      case 'Happy':
        return 'bounce';
      case 'Okay':
        return 'sway';
      case 'Stressed':
        return 'breathe';
      case 'Critical':
        return 'shake';
      default:
        return 'bounce';
    }
  }
</script>

<div
  class="pet-container clickable"
  on:mouseenter={handleMouseEnter}
  on:mouseleave={handleMouseLeave}
  on:click={handleClick}
  role="button"
  tabindex="0"
>
  <div class="pet-emoji {animationClass}" class:hovered={isHovered}>
    {emoji}
  </div>

  {#if state === 'Critical'}
    <ParticleEffect />
  {/if}
</div>

<style>
  .pet-container {
    position: relative;
    cursor: pointer;
    user-select: none;
  }

  .pet-emoji {
    font-size: 120px;
    line-height: 1;
    display: block;
    transition: transform 0.2s ease;
  }

  .pet-emoji.hovered {
    transform: scale(1.1);
  }

  /* Animation: Bounce (Happy state) */
  @keyframes bounce {
    0%, 100% {
      transform: translateY(0) scale(1);
    }
    50% {
      transform: translateY(-10px) scale(1.05);
    }
  }

  .bounce {
    animation: bounce 2s ease-in-out infinite;
  }

  /* Animation: Sway (Okay state) */
  @keyframes sway {
    0%, 100% {
      transform: rotate(-5deg);
    }
    50% {
      transform: rotate(5deg);
    }
  }

  .sway {
    animation: sway 3s ease-in-out infinite;
  }

  /* Animation: Breathe (Stressed state) */
  @keyframes breathe {
    0%, 100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.1);
      opacity: 0.8;
    }
  }

  .breathe {
    animation: breathe 1.5s ease-in-out infinite;
  }

  /* Animation: Shake (Critical state) */
  @keyframes shake {
    0%, 100% {
      transform: translateX(0) rotate(0deg);
    }
    10%, 30%, 50%, 70%, 90% {
      transform: translateX(-5px) rotate(-5deg);
    }
    20%, 40%, 60%, 80% {
      transform: translateX(5px) rotate(5deg);
    }
  }

  .shake {
    animation: shake 0.5s ease-in-out infinite;
  }
</style>
