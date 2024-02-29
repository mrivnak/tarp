<script setup lang="ts">
import { ref } from "vue";
import { SimpleFileReport } from "../types";

const expanded = ref(false);

let props = defineProps<{
    report: SimpleFileReport;
}>();
</script>

<template>
    <h1 class="text-bold">{{ props.report.path }}</h1>
    <button @click="expanded = !expanded">Toggle</button>
    <div v-if="expanded">
        <p v-for="line in props.report.lines" :key="line.content">
          {{ line.number }}  
          <span class="whitespace-pre"
                :class="{
                    'bg-red-300': line.coverage === 'uncovered',
                    'bg-green-300': line.coverage === 'covered',
                }"
                >{{ line.content }}</span
            >
        </p>
    </div>
</template>
