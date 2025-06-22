<script setup lang="ts">
import { computed, ref, watch, type Ref, onBeforeUnmount, onMounted } from 'vue'

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

const codeType = ref('rsc-depolarize-d-3')
const decoded = ref(false)

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
  errors?: Map<number, string>
}

const code: Ref<Code> = ref({
  id: 'rsc-depolarize-d-3',
  name: 'Surface Code (Depolarize, d=3)',
  d: 3,
  data_qubit_positions: [
    [1.0, 1.0],
    [1.0, 3.0],
    [1.0, 5.0],
    [3.0, 1.0],
    [3.0, 3.0],
    [3.0, 5.0],
    [5.0, 1.0],
    [5.0, 3.0],
    [5.0, 5.0]
  ],
  data_qubit_actions: [
    { Y: [2, 1], X: [1], Z: [2] },
    { Y: [0, 3, 2], X: [3], Z: [0, 2] },
    { Y: [0, 3], Z: [0], X: [3] },
    { Y: [1, 2, 4], Z: [2], X: [1, 4] },
    { Z: [2, 5], Y: [2, 3, 5, 4], X: [3, 4] },
    { Z: [5], Y: [3, 6, 5], X: [3, 6] },
    { X: [4], Y: [4, 7], Z: [7] },
    { Y: [4, 5, 7], X: [4], Z: [5, 7] },
    { Y: [5, 6], Z: [5], X: [6] }
  ],
  stabilizer_positions: [
    [0.0, 4.0],
    [2.0, 0.0],
    [2.0, 2.0],
    [2.0, 4.0],
    [4.0, 2.0],
    [4.0, 4.0],
    [4.0, 6.0],
    [6.0, 2.0]
  ],
  stabilizer_shapes: [
    [
      [1.0, 5.0],
      [0.9103606910965665, 4.995974293995239],
      [0.8214431052013633, 4.983929588598629],
      [0.7339631544333249, 4.963962860695853],
      [0.6486251759186573, 4.936234870639737],
      [0.5661162608824419, 4.900968867902419],
      [0.48710072259409387, 4.858448793601866],
      [0.41221474770752686, 4.8090169943749475],
      [0.3420612740602874, 4.753071466003611],
      [0.27720513617260845, 4.691062648986865],
      [0.2181685175319702, 4.623489801858733],
      [0.16542674627869747, 4.5508969814521025],
      [0.119404468143262, 4.473868662472999],
      [0.08047222744854943, 4.393025031653924],
      [0.04894348370484647, 4.3090169943749475],
      [0.02507208781817638, 4.222520933956314],
      [0.0090502382320653, 4.134233265817656],
      [0.0010069334586854106, 4.044864830350515],
      [0.0010069334586852996, 3.955135169649485],
      [0.009050238232065189, 3.8657667341823445],
      [0.02507208781817638, 3.7774790660436857],
      [0.04894348370484636, 3.6909830056250525],
      [0.0804722274485492, 3.606974968346077],
      [0.119404468143262, 3.5261313375270014],
      [0.16542674627869725, 3.4491030185478975],
      [0.21816851753197009, 3.3765101981412666],
      [0.27720513617260845, 3.3089373510131352],
      [0.3420612740602873, 3.246928533996389],
      [0.41221474770752675, 3.1909830056250525],
      [0.48710072259409376, 3.141551206398134],
      [0.5661162608824417, 3.099031132097581],
      [0.6486251759186572, 3.063765129360263],
      [0.7339631544333247, 3.0360371393041468],
      [0.8214431052013632, 3.0160704114013703],
      [0.9103606910965664, 3.004025706004761],
      [0.9999999999999999, 3.0]
    ],
    [
      [1.0, 1.0],
      [1.0040257060047608, 0.9103606910965666],
      [1.0160704114013703, 0.8214431052013634],
      [1.0360371393041468, 0.7339631544333249],
      [1.0637651293602628, 0.6486251759186574],
      [1.099031132097581, 0.5661162608824419],
      [1.141551206398134, 0.48710072259409387],
      [1.1909830056250525, 0.412214747707527],
      [1.246928533996389, 0.3420612740602874],
      [1.3089373510131352, 0.27720513617260856],
      [1.3765101981412664, 0.2181685175319703],
      [1.4491030185478975, 0.16542674627869736],
      [1.5261313375270014, 0.119404468143262],
      [1.6069749683460763, 0.08047222744854943],
      [1.6909830056250525, 0.04894348370484647],
      [1.7774790660436854, 0.02507208781817638],
      [1.8657667341823445, 0.0090502382320653],
      [1.955135169649485, 0.0010069334586854106],
      [2.0448648303505146, 0.0010069334586852996],
      [2.1342332658176555, 0.009050238232065189],
      [2.2225209339563143, 0.02507208781817638],
      [2.3090169943749475, 0.04894348370484636],
      [2.3930250316539237, 0.08047222744854932],
      [2.4738686624729986, 0.11940446814326189],
      [2.5508969814521025, 0.16542674627869725],
      [2.6234898018587334, 0.21816851753197009],
      [2.6910626489868648, 0.27720513617260834],
      [2.753071466003611, 0.3420612740602873],
      [2.8090169943749475, 0.41221474770752675],
      [2.858448793601866, 0.48710072259409376],
      [2.900968867902419, 0.5661162608824417],
      [2.936234870639737, 0.6486251759186572],
      [2.9639628606958532, 0.7339631544333247],
      [2.9839295885986297, 0.8214431052013632],
      [2.995974293995239, 0.9103606910965664],
      [3.0, 0.9999999999999998]
    ],
    [
      [1.0, 1.0],
      [1.0, 3.0],
      [3.0, 3.0],
      [3.0, 1.0]
    ],
    [
      [1.0, 3.0],
      [1.0, 5.0],
      [3.0, 5.0],
      [3.0, 3.0]
    ],
    [
      [3.0, 1.0],
      [3.0, 3.0],
      [5.0, 3.0],
      [5.0, 1.0]
    ],
    [
      [3.0, 3.0],
      [3.0, 5.0],
      [5.0, 5.0],
      [5.0, 3.0]
    ],
    [
      [5.0, 5.0],
      [4.995974293995239, 5.0896393089034335],
      [4.983929588598629, 5.1785568947986365],
      [4.963962860695854, 5.266036845566675],
      [4.936234870639737, 5.351374824081343],
      [4.900968867902419, 5.433883739117558],
      [4.858448793601866, 5.512899277405906],
      [4.8090169943749475, 5.587785252292473],
      [4.753071466003611, 5.6579387259397125],
      [4.691062648986865, 5.722794863827391],
      [4.623489801858733, 5.7818314824680295],
      [4.5508969814521025, 5.834573253721302],
      [4.473868662472999, 5.880595531856738],
      [4.393025031653924, 5.919527772551451],
      [4.3090169943749475, 5.951056516295154],
      [4.222520933956314, 5.9749279121818235],
      [4.134233265817656, 5.990949761767935],
      [4.044864830350515, 5.998993066541314],
      [3.9551351696494854, 5.998993066541315],
      [3.865766734182345, 5.990949761767935],
      [3.7774790660436857, 5.9749279121818235],
      [3.690983005625053, 5.951056516295154],
      [3.606974968346077, 5.919527772551451],
      [3.5261313375270014, 5.880595531856738],
      [3.4491030185478975, 5.834573253721302],
      [3.3765101981412666, 5.78183148246803],
      [3.3089373510131352, 5.722794863827391],
      [3.2469285339963894, 5.6579387259397125],
      [3.1909830056250525, 5.587785252292473],
      [3.141551206398134, 5.512899277405906],
      [3.099031132097581, 5.433883739117558],
      [3.063765129360263, 5.351374824081343],
      [3.0360371393041468, 5.266036845566675],
      [3.0160704114013703, 5.178556894798637],
      [3.004025706004761, 5.0896393089034335],
      [3.0, 5.0]
    ],
    [
      [5.0, 1.0],
      [5.0896393089034335, 1.0040257060047608],
      [5.1785568947986365, 1.0160704114013703],
      [5.266036845566675, 1.0360371393041468],
      [5.351374824081343, 1.0637651293602628],
      [5.433883739117558, 1.099031132097581],
      [5.512899277405906, 1.141551206398134],
      [5.587785252292473, 1.1909830056250525],
      [5.6579387259397125, 1.246928533996389],
      [5.722794863827391, 1.3089373510131352],
      [5.7818314824680295, 1.3765101981412664],
      [5.834573253721302, 1.4491030185478975],
      [5.880595531856738, 1.5261313375270011],
      [5.919527772551451, 1.6069749683460763],
      [5.951056516295154, 1.6909830056250525],
      [5.9749279121818235, 1.7774790660436854],
      [5.990949761767935, 1.8657667341823443],
      [5.998993066541314, 1.955135169649485],
      [5.998993066541315, 2.0448648303505146],
      [5.990949761767935, 2.1342332658176555],
      [5.9749279121818235, 2.2225209339563143],
      [5.951056516295154, 2.3090169943749475],
      [5.919527772551451, 2.3930250316539237],
      [5.880595531856738, 2.4738686624729986],
      [5.834573253721302, 2.5508969814521025],
      [5.78183148246803, 2.6234898018587334],
      [5.722794863827391, 2.6910626489868648],
      [5.6579387259397125, 2.7530714660036106],
      [5.587785252292473, 2.8090169943749475],
      [5.512899277405906, 2.858448793601866],
      [5.433883739117558, 2.900968867902419],
      [5.351374824081343, 2.936234870639737],
      [5.266036845566675, 2.9639628606958532],
      [5.1785568947986365, 2.9839295885986297],
      [5.0896393089034335, 2.995974293995239],
      [5.0, 3.0]
    ]
  ],
  stabilizer_checks: [
    [
      [1, 'X'],
      [2, 'X']
    ],
    [
      [0, 'Z'],
      [3, 'Z']
    ],
    [
      [0, 'X'],
      [1, 'X'],
      [3, 'X'],
      [4, 'X']
    ],
    [
      [1, 'Z'],
      [2, 'Z'],
      [4, 'Z'],
      [5, 'Z']
    ],
    [
      [3, 'Z'],
      [4, 'Z'],
      [6, 'Z'],
      [7, 'Z']
    ],
    [
      [4, 'X'],
      [5, 'X'],
      [7, 'X'],
      [8, 'X']
    ],
    [
      [5, 'Z'],
      [8, 'Z']
    ],
    [
      [6, 'X'],
      [7, 'X']
    ]
  ],
  stabilizer_colors: [
    '#e8f0ff',
    '#e8f5e8',
    '#e8f0ff',
    '#e8f5e8',
    '#e8f5e8',
    '#e8f0ff',
    '#e8f5e8',
    '#e8f0ff'
  ]
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
    console.log('randomize error on data qubit ', idx)
    if (code.value.errors == null) {
      code.value.errors = new Map()
    }
    const error_type = ['I', 'X', 'Y', 'Z'][Math.floor(Math.random() * 4)]
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

onMounted(() => {
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
}

const syndrome = ref<Set<number>>(new Set())

watch(code.value, () => {
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
        <p v-if="!decoded">Click "Decode" to start decoding</p>
        <p v-if="decoded">Rigorously proven (<a href="" target="_blank">decoding process</a>)</p>

        <div class="controller-panel">
          <div>
            <button class="button reset-button" @click="reset">Reset</button>
          </div>
          <!-- Code type selector -->
          <div>
            <select id="code-type" class="select" v-model="codeType">
              <option value="rsc-depolarize-d-3">Surface Code (Depolarize, d=3)</option>
              <option value="rsc-depolarize-d-5">Surface Code (Depolarize, d=5)</option>
              <option value="rsc-bit-flip-d-3">Surface Code (Bit-Flip, d=3)</option>
              <option value="rsc-bit-flip-d-5">Surface Code (Bit-Flip, d=5)</option>
              <option value="rsc-only-y-d-3">Surface Code (Only Y, d=3)</option>
              <option value="rsc-only-y-d-5">Surface Code (Only Y, d=5)</option>
              <option value="color-bit-flip-d-3">Color Code (Bit-Flip, d=3)</option>
              <option value="color-bit-flip-d-5">Color Code (Bit-Flip, d=5)</option>
            </select>
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
    1.8 * var(--hs)
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
