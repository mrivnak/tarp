<script setup lang="ts">
import { invoke } from "@tauri-apps/api"
import { ref } from "vue"
import { useRoute } from "vue-router"
// @ts-ignore
import { PulseLoader } from "vue3-spinner"
import { SimpleReport } from "../types"
import FileDisplay from "./FileDisplay.vue"

const route = useRoute()

const report = ref(null as SimpleReport | null)
const loading = ref(false)

async function run() {
  loading.value = true
  let result: SimpleReport | null = await invoke("run_tarpaulin", {})
  if (result) {
    report.value = result
  }
  loading.value = false
}
</script>

<template>
  <div
    class="fixed h-14 flex items-center w-full px-4 border-b border-neutral-600 bg-neutral-800 bg-opacity-95"
  >
    <h1 class="grow text-bold text-2xl">{{ route.params.name }}</h1>
    <p class="px-2" v-if="report">
      Coverage: {{ (report.coverage * 100).toFixed(2) }}%
    </p>
    <button
      class="bg-blue-900 p-1 px-4 w-20 rounded border border-blue-600"
      @click="run"
    >
      <PulseLoader v-if="loading" :loading="true" color="#e5e5e5" size="5px" />
      <span v-else>Run</span>
    </button>
  </div>
  <div class="pt-16 m-2 mt-0">
    <FileDisplay
      v-if="report"
      v-for="file in report.files.filter((f) => f.coverable > 0)"
      :report="file"
      :key="file.path"
    />
  </div>
</template>
