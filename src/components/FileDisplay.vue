<script setup lang="ts">
import { ref } from "vue"
import { SimpleFileReport } from "../types"

const expanded = ref(false)

let props = defineProps<{
  report: SimpleFileReport
}>()
</script>

<template>
  <div class="border border-neutral-600 p-2 cursor-pointer flex" @click="expanded = !expanded">
    <h1 class="text-bold">
      {{ props.report.path }}
    </h1>
    <p class="mx-2 text-neutral-400">
      {{ report.covered }}/{{ report.coverable }} ({{ ((report.covered / report.coverable) * 100).toFixed(2) }}%)
    </p>
  </div>
  <div v-if="expanded">
    <table class="font-mono text-sm">
      <tr v-for="line in props.report.lines" :key="line.content">
        <td class="px-2 text-neutral-500">{{ line.number }}</td>
        <td
          class="whitespace-pre bg-opacity-50"
          :class="{
            'bg-red-900': line.coverage === 'uncovered',
            'bg-green-900': line.coverage === 'covered',
          }"
        >
          {{ line.content }}
        </td>
      </tr>
    </table>
  </div>
</template>
