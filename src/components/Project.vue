<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import {useRoute} from "vue-router";
import { SimpleReport } from "../types";
import FileDisplay from "./FileDisplay.vue";

const route = useRoute();

const report = ref(null as SimpleReport | null);

async function run() {
  let result: SimpleReport | null = await invoke("run_tarpaulin", {});
  if (result) {
    report.value = result;
  }
}

</script>

<template>
  <h1 class="text-bold">{{ route.params.name }}</h1>
  <button @click="run">Run</button>
  <p v-if="report">Coverage: {{ (report.coverage * 100).toFixed(2) }}%</p>
  <FileDisplay v-if="report" v-for="file in report.files" :report="file" :key="file.path" />
</template>
