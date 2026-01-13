<script>
  import { onMount } from 'svelte';

  let canvas;
  let ctx;
  let particles = [];
  let animationId;

  class Particle {
    constructor(x, y) {
      this.x = x;
      this.y = y;
      this.size = Math.random() * 3 + 2;
      this.speedX = Math.random() * 3 - 1.5;
      this.speedY = Math.random() * -3 - 1;
      this.color = this.getRandomColor();
      this.life = 100;
    }

    getRandomColor() {
      const colors = ['#ff6b6b', '#ffa500', '#ffff00', '#ff4500'];
      return colors[Math.floor(Math.random() * colors.length)];
    }

    update() {
      this.x += this.speedX;
      this.y += this.speedY;
      this.speedY += 0.1; // Gravity
      this.life -= 2;
    }

    draw(ctx) {
      ctx.fillStyle = this.color;
      ctx.globalAlpha = this.life / 100;
      ctx.beginPath();
      ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
      ctx.fill();
      ctx.globalAlpha = 1;
    }

    isDead() {
      return this.life <= 0;
    }
  }

  onMount(() => {
    ctx = canvas.getContext('2d');
    canvas.width = 200;
    canvas.height = 200;

    function animate() {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Create new particles at the center
      if (Math.random() > 0.7) {
        particles.push(new Particle(canvas.width / 2, canvas.height / 2));
      }

      // Update and draw particles
      particles = particles.filter(particle => {
        particle.update();
        particle.draw(ctx);
        return !particle.isDead();
      });

      animationId = requestAnimationFrame(animate);
    }

    animate();

    return () => {
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
    };
  });
</script>

<canvas bind:this={canvas} class="particle-canvas"></canvas>

<style>
  .particle-canvas {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    pointer-events: none;
    z-index: -1;
  }
</style>
