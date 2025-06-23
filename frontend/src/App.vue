<script setup lang="ts">
import {
  computed,
  ref,
  watch,
  type Ref,
  type ComputedRef,
  type Reactive,
  onBeforeUnmount,
  onMounted,
  reactive,
  watchEffect
} from 'vue'

const height = ref(0) // the height of the portrait region
const aspectRatio = 1.618
const width = computed(() => {
  return height.value / aspectRatio
})

function updateHeight() {
  if (window.innerHeight > window.innerWidth * aspectRatio) {
    height.value = window.innerWidth * aspectRatio * 0.98
  } else {
    height.value = window.innerHeight * 0.98
  }
}
window.addEventListener('resize', updateHeight)
updateHeight()

interface Code {
  id: string
  name: string
  d: number
  data_qubit_positions: [number, number][]
  data_qubit_actions: { [error: string]: number[] }[] // { error: syndrome }
  stabilizer_positions: [number, number][]
  stabilizer_shapes: [number, number][][]
  stabilizer_checks: [number, string][][] // (data_qubit_index, check_type)
  stabilizer_colors: string[]
  logical_observables: [number, string][][] // (data_qubit_index, check_type)
  errors?: Map<number, string>
  decoded?: Decoded
}

const defaultCode: Reactive<Code> = reactive({
  id: '',
  name: '',
  d: 3,
  data_qubit_positions: [],
  data_qubit_actions: [],
  stabilizer_positions: [],
  stabilizer_shapes: [],
  stabilizer_checks: [],
  stabilizer_colors: [],
  logical_observables: []
})
const codes: Ref<Reactive<Code>[]> = ref([])
const codeNameIds: Ref<string[]> = ref([])
const codeIndex = ref<number>(-1)
const code: ComputedRef<Reactive<Code>> = computed(() => {
  if (codeIndex.value == -1) {
    return defaultCode
  }
  return codes.value[codeIndex.value]
})

const trans_margin = 1
const trans = computed(() => {
  // get the max and min of the positions
  const maxX = Math.max(
    ...code.value.data_qubit_positions.map(([x, _]) => x),
    ...code.value.stabilizer_positions.map(([x, _]) => x)
  )
  const maxY = Math.max(
    ...code.value.data_qubit_positions.map(([_, y]) => y),
    ...code.value.stabilizer_positions.map(([_, y]) => y)
  )
  const minX = Math.min(
    ...code.value.data_qubit_positions.map(([x, _]) => x),
    ...code.value.stabilizer_positions.map(([x, _]) => x)
  )
  const minY = Math.min(
    ...code.value.data_qubit_positions.map(([_, y]) => y),
    ...code.value.stabilizer_positions.map(([_, y]) => y)
  )
  // calculate the scale
  const scale = Math.min(1 / (maxX - minX + 2 * trans_margin), 1 / (maxY - minY + 2 * trans_margin))
  const centerX = (maxX + minX) / 2
  const centerY = (maxY + minY) / 2
  return [scale, centerX, centerY]
})
const scale = computed(() => {
  return trans.value[0]
})

function transform(pos: [number, number]) {
  const [scale, centerX, centerY] = trans.value
  const [x, y] = pos
  return [(0.5 + scale * (x - centerX)) * width.value, (0.5 + scale * (y - centerY)) * width.value]
}

const data_qubit_radius = computed(() => {
  return 0.4 * width.value * scale.value
})

function updateStyle() {
  document.documentElement.style.setProperty('--window-height', `${window.innerHeight}px`)
  document.documentElement.style.setProperty('--window-width', `${window.innerWidth}px`)
  document.documentElement.style.setProperty('--height', `${height.value}px`)
  document.documentElement.style.setProperty('--width', `${width.value}px`)
  document.documentElement.style.setProperty('--hs', `${height.value * 0.01}px`)
  document.documentElement.style.setProperty('--ws', `${width.value * 0.01}px`)
  document.documentElement.style.setProperty('--radius', `${data_qubit_radius.value}px`)
}
watch([height, code], updateStyle)
updateStyle()

const downDataQubitIdx = ref<number | null>(null)

function mouseDown(idx: number) {
  downDataQubitIdx.value = idx
  downErrorAction.value = null
}

function randomizeError(idx: number) {
  if (downDataQubitIdx.value == idx) {
    // console.log('randomize error on data qubit ', idx)
    code.value.decoded = undefined
    if (code.value.errors == null) {
      code.value.errors = new Map()
    }
    const error_types = Object.keys(code.value.data_qubit_actions[idx])
    error_types.push('I')
    const error_type = error_types[Math.floor(Math.random() * error_types.length)]
    if (error_type === 'I') {
      code.value.errors.delete(idx)
    } else {
      code.value.errors.set(idx, error_type)
    }
  }
  downDataQubitIdx.value = null
}

const dataQubits = ref<HTMLDivElement[]>([])

function globalMouseUp(event: MouseEvent | TouchEvent) {
  if (downDataQubitIdx.value !== null && downErrorAction.value !== null) {
    code.value.decoded = undefined
    if (code.value.errors == null) {
      code.value.errors = new Map()
    }
    if (downErrorAction.value === 'I') {
      code.value.errors.delete(downDataQubitIdx.value)
    } else {
      code.value.errors.set(downDataQubitIdx.value, downErrorAction.value)
    }
  }
  // check if it's randomize event
  else if (downDataQubitIdx.value !== null) {
    let target = event.target as Element
    if (event instanceof TouchEvent) {
      target = document.elementFromPoint(
        event.changedTouches[0].clientX,
        event.changedTouches[0].clientY
      ) as Element
    }
    if (target.closest('.data-qubit') == dataQubits.value[downDataQubitIdx.value]) {
      randomizeError(downDataQubitIdx.value)
    }
  }
  downDataQubitIdx.value = null
}
function globalTouchMove(event: TouchEvent) {
  // handling touch screen error selection
  let target = document.elementFromPoint(
    event.changedTouches[0].clientX,
    event.changedTouches[0].clientY
  ) as HTMLElement
  if (target.closest('.error-button')) {
    let error = 'I'
    if (target.classList.contains('error-i')) {
      error = 'I'
    } else if (target.classList.contains('error-x')) {
      error = 'X'
    } else if (target.classList.contains('error-y')) {
      error = 'Y'
    } else if (target.classList.contains('error-z')) {
      error = 'Z'
    }
    mouseEnterErrorAction(error)
    target.focus()
  } else {
    mouseLeaveErrorAction()
    ;(document.activeElement as HTMLElement)?.blur()
  }
}

const ERROR_ACTION_DIS_RATIO = 3

const downErrorAction = ref<string | null>(null)

function mouseEnterErrorAction(errorType: string) {
  downErrorAction.value = errorType
}

function mouseLeaveErrorAction() {
  downErrorAction.value = null
}

function stabilizerCheckPosition(stabilizer_idx: number, data_idx: number) {
  const [x1, y1] = transform(code.value.data_qubit_positions[data_idx])
  const [x2, y2] = transform(code.value.stabilizer_positions[stabilizer_idx])
  const diff = [x2 - x1, y2 - y1]
  const length = Math.sqrt(diff[0] ** 2 + diff[1] ** 2)
  const direction = [diff[0] / length, diff[1] / length]
  const check_length = 1.5 * data_qubit_radius.value
  return [x1 + direction[0] * check_length, y1 + direction[1] * check_length]
}

async function getCodes(): Promise<Code[]> {
  try {
    const response = await fetch('/api/codes')
    const codes: Code[] = await response.json()
    return codes
  } catch (error) {
    decoderServerDown.value = true
  }
  return []
}

onMounted(async () => {
  let loadingCodes = await getCodes()
  for (const code of loadingCodes) {
    codes.value.push(reactive(code))
  }
  if (loadingCodes.length > 0) {
    codeIndex.value = 0
  }
  document.addEventListener('mouseup', globalMouseUp)
  document.addEventListener('touchend', globalMouseUp)
  document.addEventListener('touchmove', globalTouchMove)
})
onBeforeUnmount(() => {
  document.removeEventListener('mouseup', globalMouseUp)
  document.removeEventListener('touchend', globalMouseUp)
  document.removeEventListener('touchmove', globalTouchMove)
})

function reset() {
  code.value.errors = undefined
  code.value.decoded = undefined
}

interface Decoded {
  correction: [number, string][]
  lower: number
  upper: number
  json?: object
  html?: string
}

const decoding = ref(false)
const decoderServerDown = ref(false)

const syndrome = ref<Set<number>>(new Set())

async function decode(with_html: boolean = false): Promise<Decoded | undefined> {
  if (syndrome.value.size === 0) {
    const decoded: Decoded = {
      correction: [],
      lower: 0,
      upper: 0
    }
    code.value.decoded = decoded
    return decoded
  }

  decoding.value = true
  code.value.decoded = undefined

  try {
    // Convert syndrome Set to comma-separated string
    const syndromeString = Array.from(syndrome.value).join(',')

    // Send GET request to backend
    let url = `/api/decode?code_id=${code.value.id}&syndrome=${syndromeString}`
    if (with_html) {
      url += '&with_html=true'
    }
    const response = await fetch(url)

    if (!response.ok) {
      throw new Error(await response.text())
    }

    const data = await response.json()
    code.value.decoded = data
    return data
  } catch (error) {
    console.error(error)
    alert('Decoding error: ' + (error as Error).message)
  } finally {
    decoding.value = false
  }
}

const correction_map: ComputedRef<Map<number, string>> = computed(() => {
  const map = new Map<number, string>()
  if (code.value.decoded != null) {
    for (const [data_idx, error_type] of code.value.decoded.correction) {
      map.set(data_idx, error_type)
    }
  }
  return map
})

watchEffect(() => {
  const errors = code.value.errors
  const new_syndrome: Set<number> = new Set()
  if (errors != null) {
    for (const [data_idx, error_type] of errors) {
      const error_syndrome = code.value.data_qubit_actions[data_idx][error_type]
      for (const stabilizer_idx of error_syndrome) {
        if (!new_syndrome.has(stabilizer_idx)) {
          new_syndrome.add(stabilizer_idx)
        } else {
          new_syndrome.delete(stabilizer_idx)
        }
      }
    }
  }
  syndrome.value = new_syndrome
})

function pauli_product(pauli1: string, pauli2: string): string {
  if (pauli1 === 'I') return pauli2
  if (pauli2 === 'I') return pauli1
  if (pauli1 === pauli2) return 'I'
  if (pauli1 === 'X' && pauli2 === 'Y') return 'Z'
  if (pauli1 === 'Y' && pauli2 === 'X') return 'Z'
  if (pauli1 === 'Y' && pauli2 === 'Z') return 'X'
  if (pauli1 === 'Z' && pauli2 === 'Y') return 'X'
  if (pauli1 === 'Z' && pauli2 === 'X') return 'Y'
  if (pauli1 === 'X' && pauli2 === 'Z') return 'Y'
  console.error(pauli1, pauli2)
  return 'I' // Should never reach here
}

function pauli_is_anti_commuting(pauli1: string, pauli2: string): boolean {
  if (pauli1 === 'I') return false
  if (pauli2 === 'I') return false
  if (pauli1 === pauli2) return false
  return true
}

const is_logical_error = computed(() => {
  if (code.value.decoded == null) {
    return false
  }
  // compute residual error
  const residual_error = new Map<number, string>()
  for (const [data_idx, error_type] of code.value.errors ?? []) {
    residual_error.set(data_idx, error_type)
  }
  for (const [data_idx, error_type] of code.value.decoded.correction) {
    if (residual_error.has(data_idx)) {
      residual_error.set(data_idx, pauli_product(residual_error.get(data_idx)!, error_type))
    } else {
      residual_error.set(data_idx, error_type)
    }
  }
  // check if any logical observable anti-commutes with the residual error
  for (const observable of code.value.logical_observables) {
    let parity = false
    for (const [data_idx, obs_type] of observable) {
      const error_type = residual_error.get(data_idx) ?? 'I'
      if (pauli_is_anti_commuting(obs_type, error_type)) {
        parity = !parity
      }
    }
    if (parity) {
      return true
    }
  }
  return false
})

const showDecodingProcess = ref(false)
function viewDecodingProcess() {
  showDecodingProcess.value = true
}

async function decodeShowHTML() {
  showDecodingProcess.value = false
  const decoded = await decode(true)
  if (decoded?.html != null) {
    // I tried to open a new tab but it doesn't work somehow
    const blob = new Blob([decoded.html], { type: 'text/html' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = 'decoding-process.html'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
  }
}
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
          <!-- Stabilizer shapes -->
          <svg class="stabilizer-shape non-selectable">
            <polygon
              v-for="(shape, idx) in code.stabilizer_shapes"
              :key="idx"
              :points="shape.map(([x, y]) => transform([y, x])).join(' ')"
              :fill="code.stabilizer_colors[idx]"
              class="non-selectable"
            />
          </svg>
          <!-- Stabilizer checks-->
          <div
            v-for="(check, stabilizer_idx) in code.stabilizer_checks"
            :key="stabilizer_idx"
            style="position: absolute"
          >
            <div
              v-for="([data_idx, check_type], check_idx) in check"
              :key="check_idx"
              class="stabilizer-check non-selectable"
              :style="{
                top: stabilizerCheckPosition(stabilizer_idx, data_idx)[0] + 'px',
                left: stabilizerCheckPosition(stabilizer_idx, data_idx)[1] + 'px'
              }"
            >
              {{ check_type }}
            </div>
          </div>
          <!-- Stabilizers -->
          <div
            v-for="(pos, idx) in code.stabilizer_positions"
            :key="idx"
            class="qubit stabilizer-qubit non-selectable"
            :style="{
              borderRadius: data_qubit_radius + 'px',
              width: 2 * data_qubit_radius + 'px',
              height: 2 * data_qubit_radius + 'px',
              border:
                data_qubit_radius * 0.2 + 'px solid ' + (syndrome.has(idx) ? 'pink' : 'lightgray'),
              top: transform(pos)[0] + 'px',
              left: transform(pos)[1] + 'px',
              backgroundColor: syndrome.has(idx) ? 'red' : 'white'
            }"
          ></div>
          <!-- Data qubits -->
          <div
            v-for="(pos, idx) in code.data_qubit_positions"
            :key="idx"
            class="qubit data-qubit"
            :style="{
              borderRadius: data_qubit_radius + 'px',
              width: 2 * data_qubit_radius + 'px',
              height: 2 * data_qubit_radius + 'px',
              border: data_qubit_radius * 0.2 + 'px solid black',
              top: transform(pos)[0] + 'px',
              left: transform(pos)[1] + 'px'
            }"
            @touchstart.prevent="mouseDown(idx)"
            @mousedown.prevent="mouseDown(idx)"
            ref="dataQubits"
          >
            <span v-if="downDataQubitIdx !== idx" class="non-selectable">{{
              code.errors?.get(idx) ?? ''
            }}</span>
            <svg
              v-else
              fill="#000000"
              width="800px"
              height="800px"
              viewBox="0 0 256 256"
              xmlns="http://www.w3.org/2000/svg"
              class="non-selectable"
            >
              <g fill-rule="evenodd">
                <path
                  d="M47.895 88.097c.001-4.416 3.064-9.837 6.854-12.117l66.257-39.858c3.785-2.277 9.915-2.277 13.707.008l66.28 39.934c3.786 2.28 6.853 7.703 6.852 12.138l-.028 79.603c-.001 4.423-3.069 9.865-6.848 12.154l-66.4 40.205c-3.781 2.29-9.903 2.289-13.69-.01l-66.167-40.185c-3.78-2.295-6.842-7.733-6.84-12.151l.023-79.72zm13.936-6.474l65.834 36.759 62.766-36.278-62.872-36.918L61.83 81.623zM57.585 93.52c0 1.628-1.065 71.86-1.065 71.86-.034 2.206 1.467 4.917 3.367 6.064l61.612 37.182.567-77.413s-64.48-39.322-64.48-37.693zm76.107 114.938l60.912-38.66c2.332-1.48 4.223-4.915 4.223-7.679V93.125l-65.135 37.513v77.82z"
                />
                <path
                  d="M77.76 132.287c-4.782 2.762-11.122.735-14.16-4.526-3.037-5.261-1.622-11.765 3.16-14.526 4.783-2.762 11.123-.735 14.16 4.526 3.038 5.261 1.623 11.765-3.16 14.526zm32 21c-4.782 2.762-11.122.735-14.16-4.526-3.037-5.261-1.622-11.765 3.16-14.526 4.783-2.762 11.123-.735 14.16 4.526 3.038 5.261 1.623 11.765-3.16 14.526zm-32 16c-4.782 2.762-11.122.735-14.16-4.526-3.037-5.261-1.622-11.765 3.16-14.526 4.783-2.762 11.123-.735 14.16 4.526 3.038 5.261 1.623 11.765-3.16 14.526zm32 21c-4.782 2.762-11.122.735-14.16-4.526-3.037-5.261-1.622-11.765 3.16-14.526 4.783-2.762 11.123-.735 14.16 4.526 3.038 5.261 1.623 11.765-3.16 14.526zm78.238-78.052c-4.783-2.762-11.122-.735-14.16 4.526-3.037 5.261-1.623 11.765 3.16 14.526 4.783 2.762 11.123.735 14.16-4.526 3.038-5.261 1.623-11.765-3.16-14.526zm-16.238 29c-4.782-2.762-11.122-.735-14.16 4.526-3.037 5.261-1.622 11.765 3.16 14.526 4.783 2.762 11.123.735 14.16-4.526 3.038-5.261 1.623-11.765-3.16-14.526zm-17 28c-4.782-2.762-11.122-.735-14.16 4.526-3.037 5.261-1.622 11.765 3.16 14.526 4.783 2.762 11.123.735 14.16-4.526 3.038-5.261 1.623-11.765-3.16-14.526zM128.5 69c-6.351 0-11.5 4.925-11.5 11s5.149 11 11.5 11S140 86.075 140 80s-5.149-11-11.5-11z"
                />
              </g>
            </svg>
          </div>
          <!-- Correction -->
          <div v-for="(pos, idx) in code.data_qubit_positions" :key="idx">
            <div
              v-if="code.decoded && correction_map.get(idx) != null"
              class="qubit correction-border"
              :style="{
                borderRadius: data_qubit_radius + 'px',
                width: 1.7 * data_qubit_radius + 'px',
                height: 1.7 * data_qubit_radius + 'px',
                top: transform(pos)[0] - data_qubit_radius * 1 + 'px',
                left: transform(pos)[1] + data_qubit_radius * 1 + 'px',
                color: 'darkgreen'
              }"
            ></div>
            <div
              v-if="code.decoded && correction_map.get(idx) != null"
              class="qubit correction-text non-selectable"
              :style="{
                top: transform(pos)[0] - data_qubit_radius * 1 + 'px',
                left: transform(pos)[1] + data_qubit_radius * 1 + 'px',
                color: 'darkgreen'
              }"
            >
              {{ correction_map.get(idx) }}
            </div>
          </div>
          <!-- Error actions -->
          <div v-if="downDataQubitIdx !== null" class="error-actions">
            <!-- I error button (top) -->
            <button
              class="error-button error-i"
              :style="{
                top:
                  transform(code.data_qubit_positions[downDataQubitIdx])[0] -
                  data_qubit_radius * ERROR_ACTION_DIS_RATIO +
                  'px',
                left: transform(code.data_qubit_positions[downDataQubitIdx])[1] + 'px',
                width: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                height: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                fontSize: ERROR_ACTION_DIS_RATIO * 0.7 * data_qubit_radius + 'px'
              }"
              @mouseenter="mouseEnterErrorAction('I')"
              @mouseleave="mouseLeaveErrorAction"
            >
              <span class="non-selectable">I</span>
            </button>

            <!-- X error button (right) -->
            <button
              class="error-button error-x"
              :style="{
                top: transform(code.data_qubit_positions[downDataQubitIdx])[0] + 'px',
                left:
                  transform(code.data_qubit_positions[downDataQubitIdx])[1] +
                  data_qubit_radius * ERROR_ACTION_DIS_RATIO +
                  'px',
                width: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                height: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                fontSize: ERROR_ACTION_DIS_RATIO * 0.7 * data_qubit_radius + 'px'
              }"
              @mouseenter="mouseEnterErrorAction('X')"
              @touchmove="mouseEnterErrorAction('X')"
              @mouseleave="mouseLeaveErrorAction"
              v-if="'X' in code.data_qubit_actions[downDataQubitIdx]"
            >
              <span class="non-selectable">X</span>
            </button>

            <!-- Y error button (bottom) -->
            <button
              class="error-button error-y"
              :style="{
                top:
                  transform(code.data_qubit_positions[downDataQubitIdx])[0] +
                  data_qubit_radius * ERROR_ACTION_DIS_RATIO +
                  'px',
                left: transform(code.data_qubit_positions[downDataQubitIdx])[1] + 'px',
                width: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                height: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                fontSize: ERROR_ACTION_DIS_RATIO * 0.7 * data_qubit_radius + 'px'
              }"
              @mouseenter="mouseEnterErrorAction('Y')"
              @mouseleave="mouseLeaveErrorAction"
              v-if="'Y' in code.data_qubit_actions[downDataQubitIdx]"
            >
              <span class="non-selectable">Y</span>
            </button>

            <!-- Z error button (left) -->
            <button
              class="error-button error-z"
              :style="{
                top: transform(code.data_qubit_positions[downDataQubitIdx])[0] + 'px',
                left:
                  transform(code.data_qubit_positions[downDataQubitIdx])[1] -
                  data_qubit_radius * ERROR_ACTION_DIS_RATIO +
                  'px',
                width: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                height: ERROR_ACTION_DIS_RATIO * data_qubit_radius + 'px',
                fontSize: ERROR_ACTION_DIS_RATIO * 0.7 * data_qubit_radius + 'px'
              }"
              @mouseenter="mouseEnterErrorAction('Z')"
              @mouseleave="mouseLeaveErrorAction"
              v-if="'Z' in code.data_qubit_actions[downDataQubitIdx]"
            >
              <span class="non-selectable">Z</span>
            </button>
          </div>
        </div>
        <p v-if="!code.decoded && !decoding">
          Press data qubits to add errors, then click "Decode" to find a correction
        </p>
        <p v-if="decoding">Decoding...</p>
        <p v-if="code.decoded">
          <span v-if="code.decoded.upper == 0">Empty correction</span>
          <span v-else-if="code.decoded.lower >= code.decoded.upper - 1"
            >Found optimal Most-Likely Error (MLE)</span
          >
          <span v-else
            >Rigorously proven: {{ code.decoded.lower }} ≤ weight(MWPF) ≤ {{ code.decoded.upper }}
          </span>
          <span v-if="code.decoded.upper != 0">
            (<a href="" @click.prevent="viewDecodingProcess">view decoding process</a>)</span
          >
        </p>

        <div class="controller-panel">
          <div>
            <button class="button reset-button" @click="reset" :disabled="decoding">Reset</button>
          </div>
          <!-- Code type selector -->
          <div>
            <select id="code-type" class="select" v-model="codeIndex" :disabled="decoding">
              <option v-for="(code, codeIndex) in codes" :key="code.id" :value="codeIndex">
                {{ code.name }}
              </option>
            </select>
          </div>
          <div>
            <button
              class="button decode-button"
              @click="() => decode()"
              :disabled="decoding || code.decoded != null"
            >
              {{ decoding ? 'Decoding...' : 'Decode' }}
            </button>
          </div>
        </div>

        <div
          style="
            height: calc(10 * var(--hs));
            margin-top: calc(2 * var(--hs));
            font-size: calc(3 * var(--hs));
          "
        >
          <span
            v-if="code.decoded != null && is_logical_error"
            style="
              background-color: rgba(255, 255, 155, 0.8);
              font-weight: bold;
              padding: calc(1 * var(--hs)) calc(2 * var(--hs));
              border-radius: calc(2 * var(--hs));
              color: red;
            "
            >Logical Error!</span
          >
          <span
            v-if="code.decoded != null && !is_logical_error"
            style="
              background-color: rgba(255, 255, 155, 0.8);
              font-weight: bold;
              padding: calc(1 * var(--hs)) calc(2 * var(--hs));
              border-radius: calc(2 * var(--hs));
              color: green;
            "
            >Correction Succeed!</span
          >
        </div>
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

      <!-- Decoding process overlay -->
      <div
        v-if="showDecodingProcess"
        style="
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background-color: rgba(0, 0, 0, 0.3);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 100;
        "
      >
        <div
          style="
            background-color: white;
            padding: calc(3 * var(--hs));
            border-radius: calc(2 * var(--hs));
            text-align: center;
            max-width: 80%;
          "
        >
          <p style="font-size: calc(2.5 * var(--hs)); margin-bottom: calc(2 * var(--hs))">
            For best experience, please use a desktop browser and open the downloaded HTML
          </p>
          <img
            style="width: 100%; height: auto; margin-bottom: calc(2 * var(--hs))"
            src="/public/decoding-process.jpg"
            alt="Decoding Process"
          />
          <button
            style="
              padding: calc(2 * var(--hs)) calc(4 * var(--hs));
              font-size: calc(2 * var(--hs));
              background-color: grey;
              color: white;
              border: none;
              border-radius: calc(1.5 * var(--hs));
              cursor: pointer;
            "
            @click="showDecodingProcess = false"
          >
            Close
          </button>
          <button
            style="
              margin-left: calc(2 * var(--hs));
              padding: calc(2 * var(--hs)) calc(4 * var(--hs));
              font-size: calc(2 * var(--hs));
              background-color: #007bff;
              color: white;
              border: none;
              border-radius: calc(1.5 * var(--hs));
              cursor: pointer;
            "
            @click="decodeShowHTML()"
          >
            Decode (step-by-step)
          </button>
        </div>
      </div>

      <div
        v-if="decoderServerDown"
        style="
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background-color: rgba(0, 0, 0, 0.3);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 100;
        "
      >
        <div
          style="
            background-color: white;
            padding: calc(3 * var(--hs));
            border-radius: calc(2 * var(--hs));
            text-align: center;
            max-width: 80%;
          "
        >
          <p style="font-size: calc(2.5 * var(--hs)); margin-bottom: calc(0.5 * var(--hs))">
            Decoder server is down. Please try again later or contact me at
            <a href="mailto:yue.wu@yale.edu">yue.wu@yale.edu</a>.
          </p>
        </div>
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
  border-radius: calc(2 * var(--hs));
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
    2 * var(--hs)
  ); /* Relative to viewport height, which scales with the portrait region */
  line-height: calc(2 * var(--hs));
  color: #666;
}

.code {
  aspect-ratio: 1;
  width: var(--width);
  /* background-color: red; */
  /* margin: calc(2 * var(--hs)) auto; */
  position: relative;
}

.qubit {
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: calc(1.3 * var(--radius));
  color: red;
  font-weight: bold;
}

.stabilizer-qubit {
  transform: translate(-50%, -50%);
}

.correction-border {
  animation: rotate-border 10s linear infinite;
  --border-width: calc(0.2 * var(--radius));
  background-color: rgba(255, 255, 155, 0.8);
}

.correction-text {
  transform: translate(-50%, -50%);
}

@keyframes rotate-border {
  0% {
    border: var(--border-width) dotted green;
    border-radius: 50%;
    transform: translate(-50%, -50%) rotate(0deg);
  }
  25% {
    border: var(--border-width) dotted #00ff00;
    border-radius: 50%;
    transform: translate(-50%, -50%) rotate(90deg);
  }
  50% {
    border: var(--border-width) dotted #00cc00;
    border-radius: 50%;
    transform: translate(-50%, -50%) rotate(180deg);
  }
  75% {
    border: var(--border-width) dotted #009900;
    border-radius: 50%;
    transform: translate(-50%, -50%) rotate(270deg);
  }
  100% {
    border: var(--border-width) dotted green;
    border-radius: 50%;
    transform: translate(-50%, -50%) rotate(360deg);
  }
}

/* Data qubit specific styles */
.data-qubit {
  cursor: pointer;
  transition: all 0.3s ease;
  animation: breathing 3s ease-in-out infinite;
  background-color: white;
  box-shadow: 0 0 calc(0.5 * var(--radius)) rgba(255, 0, 0, 0.3);
}

.data-qubit:hover {
  transform: translate(-50%, -50%) scale(1.2) !important;
  box-shadow: 0 0 calc(1 * var(--radius)) rgba(255, 0, 0, 0.6);
  animation: breathing-hover 1.5s ease-in-out infinite;
  z-index: 10;
}

.data-qubit:active {
  transform: translate(-50%, -50%) scale(0.95) !important;
  box-shadow: 0 0 calc(0.3 * var(--radius)) rgba(255, 0, 0, 0.8);
}

@keyframes breathing {
  0%,
  100% {
    box-shadow: 0 0 calc(0.5 * var(--radius)) rgba(255, 0, 0, 0.3);
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    box-shadow: 0 0 calc(0.8 * var(--radius)) rgba(255, 0, 0, 0.5);
    transform: translate(-50%, -50%) scale(1.05);
  }
}

@keyframes breathing-hover {
  0%,
  100% {
    box-shadow: 0 0 calc(1 * var(--radius)) rgba(255, 0, 0, 0.6);
    transform: translate(-50%, -50%) scale(1.2);
  }
  50% {
    box-shadow: 0 0 calc(1.5 * var(--radius)) rgba(255, 0, 0, 0.8);
    transform: translate(-50%, -50%) scale(1.25);
  }
}

.stabilizer-shape {
  position: absolute;
  width: var(--width);
  height: var(--width);
  top: 0;
  left: 0;
}

.stabilizer-check {
  position: absolute;
  transform: translate(-50%, -50%);
  font-size: calc(1 * var(--radius));
  color: lightgrey;
}

.non-selectable {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  pointer-events: none;
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
  height: calc(5 * var(--hs));
  font-size: calc(2 * var(--hs));
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
  height: calc(5 * var(--hs));
  padding: calc(1 * var(--hs)) calc(1 * var(--hs));
  font-size: calc(2 * var(--hs));
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
  width: calc(12 * var(--hs));
}

.decode-button[disabled='disabled'],
.decode-button:disabled {
  background-color: #f5f5f5;
  border-color: #e0e0e0;
  color: #999;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
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

.error-actions {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 20;
}

.error-button {
  position: absolute;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  font-weight: bold;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  pointer-events: auto;
  box-shadow: 0 calc(0.2 * var(--radius)) calc(0.4 * var(--radius)) rgba(0, 0, 0, 0.3);
  transform: translate(-50%, -50%);
}

.error-button:hover {
  transform: translate(-50%, -50%) scale(1.2);
  box-shadow: 0 calc(0.3 * var(--radius)) calc(0.6 * var(--radius)) rgba(0, 0, 0, 0.4);
}

.error-button:active {
  transform: translate(-50%, -50%) scale(0.95);
}

/* I error button (Identity - no error) */
.error-i {
  background: linear-gradient(135deg, #4caf50, #45a049);
}

.error-i:hover {
  background: linear-gradient(135deg, #45a049, #3d8b40);
}

/* X error button (Bit flip) */
.error-x {
  background: linear-gradient(135deg, #f44336, #d32f2f);
}

.error-x:hover {
  background: linear-gradient(135deg, #d32f2f, #b71c1c);
}

/* Y error button (Bit and phase flip) */
.error-y {
  background: linear-gradient(135deg, #ff9800, #f57c00);
}

.error-y:hover {
  background: linear-gradient(135deg, #f57c00, #e65100);
}

/* Z error button (Phase flip) */
.error-z {
  background: linear-gradient(135deg, #2196f3, #1976d2);
}

.error-z:hover {
  background: linear-gradient(135deg, #1976d2, #1565c0);
}
</style>
