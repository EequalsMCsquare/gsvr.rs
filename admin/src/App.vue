<script setup lang="ts">
import HomeVue from "./views/Home.vue";
import {NMessageProvider, NConfigProvider, darkTheme } from 'naive-ui'
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <HomeVue />
    </n-message-provider>
  </n-config-provider>
</template>