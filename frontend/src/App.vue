<script setup lang="ts"></script>

<template>
  <div class="app-container">
    <div class="portrait-region">
      <div class="content-placeholder">
        <h1>MWPF Decoder</h1>
        <p>
          Minimum-Weight Parity Factor decoder is a generalization of Minimum-Weight Perfect
          Matching (MWPM) decoder for general qLDPC codes
        </p>
        <div class="code">
          <div style="width: 50%; height: 50%; background-color: blue"></div>
        </div>
        <p>Rigorously proven</p>
      </div>

      <!-- GitHub icon in bottom-left corner -->
      <a
        href="https://github.com/yuewuo/mwpf"
        target="_blank"
        rel="noopener noreferrer"
        class="github-link"
      >
        <svg class="github-icon" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
          />
        </svg>
      </a>

      <!-- Python/PyPI icon -->
      <a
        href="https://pypi.org/project/mwpf/"
        target="_blank"
        rel="noopener noreferrer"
        class="python-link"
      >
        <img src="./assets/pypi.png" alt="PyPI" class="python-icon" />
      </a>
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
  padding: 2vh;
}

.portrait-region {
  /* Golden ratio: height = width * 1.618 */
  aspect-ratio: 1 / 1.618;
  max-width: 100%;
  max-height: 100%;
  background-color: white;
  border-radius: 1vh;
  box-shadow: 0 0.8vh 4vh rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  position: relative;
}

.content-placeholder {
  position: relative;
  text-align: center;
  padding: 4vh;
  color: #333;
}

.content-placeholder h1 {
  font-size: 4vh; /* Relative to viewport height, which scales with the portrait region */
  margin-bottom: 2vh;
  font-weight: 600;
}

.content-placeholder p {
  font-size: 1.5vh; /* Relative to viewport height, which scales with the portrait region */
  line-height: 1.6;
  color: #666;
}

.code {
  aspect-ratio: 1;
  width: 100%;
  background-color: red;
  margin: 2vh auto;
}

/* GitHub icon styles */
.github-link {
  position: absolute;
  bottom: 2vh;
  left: 2vh;
  color: #333;
  transition: color 0.3s ease;
  z-index: 10;
}

.github-link:hover {
  color: #0366d6;
}

.github-icon {
  width: 4vh;
  height: 4vh;
}

/* Python/PyPI icon styles */
.python-link {
  position: absolute;
  bottom: 2vh;
  left: 7vh;
  color: #333;
  transition: color 0.3s ease;
  z-index: 10;
}

.python-icon {
  width: 4vh;
  height: 4vh;
}
</style>
