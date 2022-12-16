<script setup lang="ts">
import {NMessageProvider, NConfigProvider, darkTheme } from 'naive-ui'
import HomeVue from "./views/Home.vue";
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <HomeVue style="position: absolute; width: 100vw; height: 100vh;"/>
    </n-message-provider>
  </n-config-provider>
</template>