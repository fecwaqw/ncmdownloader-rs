<template>
  <div class="pa-6">
    <h1>设置</h1>
    <v-card class="pa-4" elevation="2">
      <v-row>
        <v-col cols="12" md="6">
          <v-switch v-model="dark" label="深色模式" />
        </v-col>
      </v-row>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
const dark = ref(false);
watch(dark, (v) => {
  // 简单示例：切换 html 属性用于演示
  document.documentElement.setAttribute("data-theme", v ? "dark" : "light");
});
</script>

<style scoped></style>
