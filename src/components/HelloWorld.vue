<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
defineProps<{ msg: string }>()
const count = ref(0)
const form = ref({
  x: 0,
  y: 0,
  count: 1,
})
function move(e: Event) {
  invoke('move_mouse', { posStr: JSON.stringify(form.value) })
  console.log('----------', form.value.x)
}
</script>

<template>
  <!-- <h1>{{ msg }}</h1> -->
  <div class="content">
    <div style="flex:1">
      <span>X:</span>
      <input v-model.number="form.x" type="text" placeholder="X">
      <span>Y:</span>
      <input v-model.number="form.y" type="text" placeholder="Y">
      <span>次数</span>
      <input v-model.number="form.count" type="text" placeholder="次数">
    </div>
    <div style="flex:0 0 100px">
      <span @click="move" class="btn">执行</span>
    </div>
  </div>
</template>

<style scoped>
.content {
  display: flex;
  justify-content: flex-start;
  align-items: start;
}

.content span {
  margin-left: 6px;
}

.btn {
  color: #ffffff;
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid #686968;
  padding: 4px;
  background-color: #070707;
}
</style>
