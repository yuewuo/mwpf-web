<script setup lang="ts"></script>

<template>
  <div class="app-container">
    <div class="portrait-region">
      <div class="content-placeholder">
        <h1>Portrait Content</h1>
        <p>This is your content area with 1.618 aspect ratio</p>
        <div class="code"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Reset and base styles */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

/* Disable scrolling on body and html */
:global(html),
:global(body) {
  overflow: hidden;
  height: 100vh;
  width: 100vw;
}

.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f5f5f5;
  padding: 1rem;
}

.portrait-region {
  /* Golden ratio: height = width * 1.618 */
  aspect-ratio: 1 / 1.618;
  max-width: 100%;
  max-height: 100%;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.content-placeholder {
  position: relative;
  text-align: center;
  padding: 2rem;
  color: #333;
}

.content-placeholder h1 {
  font-size: 4vh; /* Relative to viewport height, which scales with the portrait region */
  margin-bottom: 1rem;
  font-weight: 600;
}

.content-placeholder p {
  font-size: 4vh; /* Relative to viewport height, which scales with the portrait region */
  line-height: 1.6;
  color: #666;
}

.code {
  aspect-ratio: 1;
  height: 50vh;
  background-color: red;
  position: fixed;
}
</style>
