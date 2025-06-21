<script setup lang="ts">
import { ref, watch } from 'vue'

const codeDistance = ref(3)
const codeType = ref('rsc-bit-flip')
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
          <!-- Code distance selector -->
          <div>
            <select id="code-distance" class="select" v-model="codeDistance">
              <option :value="3">
                <math><mi>d</mi><mo>=</mo><mn>3</mn></math>
              </option>
              <option :value="5">
                <math><mi>d</mi><mo>=</mo><mn>5</mn></math>
              </option>
            </select>
          </div>
          <!-- Code type selector -->
          <div>
            <select id="code-type" class="select" v-model="codeType">
              <option value="rsc-bit-flip">Surface Code (Bit-Flip)</option>
              <option value="rsc-depolarize">Surface Code (Depolarize)</option>
              <option value="rsc-y-only">Surface Code (Y-only)</option>
              <option value="color-bit-flip">Color Code (Bit-Flip)</option>
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
        <div style="height: 10vh"></div>
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

.controller-panel {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 2vh;
  margin-top: 3vh;
}

.select {
  padding: 1vh 1vh;
  font-size: 1.5vh;
  border: 1px solid #ccc;
  border-radius: 0.5vh;
  background-color: white;
  color: #333;
  cursor: pointer;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
  min-width: 4vh;
}

.select:hover {
  border-color: #666;
}

.select:focus {
  outline: none;
  border-color: #3776ab;
  box-shadow: 0 0 0 0.2vh rgba(55, 118, 171, 0.2);
}

.button {
  padding: 1vh 1vh;
  font-size: 1.5vh;
  border-radius: 0.5vh;
  color: black;
  cursor: pointer;
  transition: border-color 0.3s ease, box-shadow 0.3s ease, background-color 0.3s ease;
  min-width: 3vh;
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
  background-size: 1vh 1vh;
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
  background-size: 1vh 1vh;
  transform: translateY(-0.1vh);
  box-shadow: 0 0.2vh 0.4vh rgba(230, 126, 34, 0.3);
}

.decode-button:focus {
  outline: none;
  border-color: #d35400;
  box-shadow: 0 0 0 0.2vh rgba(230, 126, 34, 0.4);
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
  background-size: 1vh 1vh;
  transform: translateY(0);
}

.reset-button {
  border: 1px solid #d5d5d5;
  background-color: white;
}

.reset-button:hover {
  border-color: #c0c0c0;
  background-color: #f8f9fa;
  transform: translateY(-0.1vh);
  box-shadow: 0 0.2vh 0.4vh rgba(149, 165, 166, 0.3);
}

.reset-button:focus {
  outline: none;
  border-color: #c0c0c0;
  box-shadow: 0 0 0 0.2vh rgba(149, 165, 166, 0.4);
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
