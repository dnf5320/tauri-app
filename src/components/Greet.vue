<script setup>
import { confirm } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

const greetMsg = ref('')
const name = ref('')

async function greet() {
  const yes = await confirm('王老师是不是很帅?', '')
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = yes ? await invoke('greet', { name: name.value }) : '你居然说王老师不帅'
}
</script>

<template>
  <div class="card">
    <input id="greet-input"
           v-model="name"
           placeholder="Enter a name..." />
    <button type="button"
            @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
