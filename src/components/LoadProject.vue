<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"
import { useRouter } from "vue-router"

const router = useRouter()

async function load() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let path: string | null = await invoke("load_project", {})
  if (path) {
    let name = path.substring(path.lastIndexOf("/") + 1)
    await router.push("/project/" + name)
  }
}
</script>

<template>
  <div class="flex justify-center items-center h-full">
    <div class="flex flex-col">
      <h1 class="text-bold">To get started, open a cargo project.</h1>
      <div class="w-full flex justify-center">
        <button
          class="shrink py-1 px-2 border border-neutral-600 rounded-lg"
          @click="load"
        >
          Open
        </button>
      </div>
    </div>
  </div>
</template>
