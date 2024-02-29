<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import {useRouter} from "vue-router";

const router = useRouter();

async function load() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let path: string | null = await invoke("load_project", {});
  if (path) {
    let name = path.substring(path.lastIndexOf('/') + 1);
    await router.push("/project/" + name);
  }
}
</script>

<template>
  <h1 class="text-bold">To get started, open a cargo project.</h1>
  <button @click="load">Open</button>
</template>
