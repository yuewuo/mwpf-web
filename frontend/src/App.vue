<script setup lang="ts">
import { computed, ref, watch } from 'vue'

const height = ref(0) // the height of the portrait region
const aspectRatio = 1.618
const width = computed(() => {
  return height.value / aspectRatio
})

function updateHeight () {
  if (window.innerHeight > window.innerWidth * aspectRatio) {
    height.value = window.innerWidth * aspectRatio * 0.98
  } else {
    height.value = window.innerHeight * 0.98
  }
}
window.addEventListener('resize', updateHeight)
updateHeight()

function updateStyle () {
  document.documentElement.style.setProperty('--window-height', `${window.innerHeight}px`)
  document.documentElement.style.setProperty('--window-width', `${window.innerWidth}px`)
  document.documentElement.style.setProperty('--height', `${height.value}px`)
  document.documentElement.style.setProperty('--width', `${width.value}px`)
  document.documentElement.style.setProperty('--hs', `${height.value * 0.01}px`)
  document.documentElement.style.setProperty('--ws', `${width.value * 0.01}px`)
}
watch(height, updateStyle)
updateStyle()

const codeDistance = ref(3)
const codeType = ref('rsc-depolarize-d-3')
const decoded = ref(false)

watch(codeDistance, newVal => {
  console.log('codeDistance changed to', newVal)
})
</script>

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
        <p v-if="!decoded">Click "Decode" to start decoding</p>
        <p v-if="decoded">Rigorously proven (<a href="" target="_blank">decoding process</a>)</p>

        <div class="controller-panel">
          <!-- Code type selector -->
          <div>
            <select id="code-type" class="select" v-model="codeType">
              <option value="rsc-bit-flip-d-3">Surface Code (Bit-Flip, d=3)</option>
              <option value="rsc-bit-flip-d-5">Surface Code (Bit-Flip, d=5)</option>
              <option value="rsc-depolarize-d-3">Surface Code (Depolarize, d=3)</option>
              <option value="rsc-depolarize-d-5">Surface Code (Depolarize, d=5)</option>
              <option value="rsc-y-only-d-3">Surface Code (Y-only, d=3)</option>
              <option value="rsc-y-only-d-5">Surface Code (Y-only, d=5)</option>
              <option value="color-bit-flip-d-3">Color Code (Bit-Flip, d=3)</option>
              <option value="color-bit-flip-d-5">Color Code (Bit-Flip, d=5)</option>
            </select>
          </div>

          <!-- Action button -->
          <div>
            <button class="button reset-button" :disabled="!decoded">Reset</button>
          </div>
          <div>
            <button class="button decode-button">Decode</button>
          </div>
        </div>
        <div style="height: calc(10 * var(--hs))"></div>
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
  height: var(--window-height);
  width: var(--window-width);
}

.app-container {
  width: var(--window-width);
  height: var(--window-height);
  display: flex;
  align-items: top;
  padding-top: var(--hs);
  justify-content: center;
  background-color: #f5f5f5;
}

.portrait-region {
  width: var(--width);
  height: var(--height);
  background-color: white;
  border-radius: var(--hs);
  box-shadow: 0 calc(0.8 * var(--hs)) calc(4 * var(--hs)) rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  position: relative;
}

.content-placeholder {
  position: relative;
  text-align: center;
  padding: calc(4 * var(--hs));
  color: #333;
}

.content-placeholder h1 {
  font-size: calc(
    4 * var(--hs)
  ); /* Relative to viewport height, which scales with the portrait region */
  margin-bottom: calc(2 * var(--hs));
  font-weight: 600;
}

.content-placeholder p {
  font-size: calc(
    1.5 * var(--hs)
  ); /* Relative to viewport height, which scales with the portrait region */
  line-height: calc(1.6 * var(--hs));
  color: #666;
}

.code {
  aspect-ratio: 1;
  width: 100%;
  background-color: red;
  margin: calc(2 * var(--hs)) auto;
}

/* GitHub icon styles */
.github-link {
  position: absolute;
  bottom: calc(2 * var(--hs));
  left: calc(2 * var(--hs));
  color: #333;
  transition: color 0.3s ease;
  z-index: 10;
}

.github-link:hover {
  color: #0366d6;
}

.github-icon {
  width: calc(4 * var(--hs));
  height: calc(4 * var(--hs));
}

/* Python/PyPI icon styles */
.python-link {
  position: absolute;
  bottom: calc(2 * var(--hs));
  left: calc(7 * var(--hs));
  color: #333;
  transition: color 0.3s ease;
  z-index: 10;
}

.python-icon {
  width: calc(4 * var(--hs));
  height: calc(4 * var(--hs));
}

.controller-panel {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: calc(2 * var(--hs));
  margin-top: calc(3 * var(--hs));
}

.select {
  padding: calc(1 * var(--hs)) calc(1 * var(--hs));
  height: calc(4 * var(--hs));
  font-size: calc(1.5 * var(--hs));
  border: 1px solid #ccc;
  border-radius: calc(0.5 * var(--hs));
  background-color: white;
  color: #333;
  cursor: pointer;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
  min-width: calc(4 * var(--hs));
}

.select:hover {
  border-color: #666;
}

.select:focus {
  outline: none;
  border-color: #3776ab;
  box-shadow: 0 0 0 calc(0.2 * var(--hs)) rgba(55, 118, 171, 0.2);
}

.button {
  height: calc(4 * var(--hs));
  padding: calc(1 * var(--hs)) calc(1 * var(--hs));
  font-size: calc(1.5 * var(--hs));
  border-radius: calc(0.5 * var(--hs));
  color: black;
  cursor: pointer;
  transition: border-color 0.3s ease, box-shadow 0.3s ease, background-color 0.3s ease;
  min-width: calc(3 * var(--hs));
  /* font-weight: 600; */
  text-shadow: 0 5px 5px rgb(255, 255, 255);
}

.decode-button {
  border: 1px solid #e67e22;
  background: linear-gradient(
    45deg,
    #ffa726 25%,
    transparent 25%,
    transparent 50%,
    #ffa726 50%,
    #ffa726 75%,
    transparent 75%,
    transparent
  );
  background-size: calc(1 * var(--hs)) calc(1 * var(--hs));
}

.decode-button:hover {
  border-color: #d35400;
  background: linear-gradient(
    45deg,
    #ff9800 25%,
    transparent 25%,
    transparent 50%,
    #ff9800 50%,
    #ff9800 75%,
    transparent 75%,
    transparent
  );
  background-size: calc(1 * var(--hs)) calc(1 * var(--hs));
  transform: translateY(calc(-0.1 * var(--hs)));
  box-shadow: 0 calc(0.2 * var(--hs)) calc(0.4 * var(--hs)) rgba(230, 126, 34, 0.3);
}

.decode-button:focus {
  outline: none;
  border-color: #d35400;
  box-shadow: 0 0 0 calc(0.2 * var(--hs)) rgba(230, 126, 34, 0.4);
}

.decode-button:active {
  background: linear-gradient(
    45deg,
    #ff8f00 25%,
    transparent 25%,
    transparent 50%,
    #ff8f00 50%,
    #ff8f00 75%,
    transparent 75%,
    transparent
  );
  background-size: calc(1 * var(--hs)) calc(1 * var(--hs));
  transform: translateY(0);
}

.reset-button {
  border: 1px solid #d5d5d5;
  background-color: white;
}

.reset-button:hover {
  border-color: #c0c0c0;
  background-color: #f8f9fa;
  transform: translateY(calc(-0.1 * var(--hs)));
  box-shadow: 0 calc(0.2 * var(--hs)) calc(0.4 * var(--hs)) rgba(149, 165, 166, 0.3);
}

.reset-button:focus {
  outline: none;
  border-color: #c0c0c0;
  box-shadow: 0 0 0 calc(0.2 * var(--hs)) rgba(149, 165, 166, 0.4);
}

.reset-button:active {
  background-color: #e9ecef;
  transform: translateY(0);
}

.reset-button:disabled {
  background-color: #f5f5f5;
  border-color: #e0e0e0;
  color: #999;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.reset-button:disabled:hover {
  background-color: #f5f5f5;
  border-color: #e0e0e0;
  transform: none;
  box-shadow: none;
}
</style>
