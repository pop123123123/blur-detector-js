<template>
  <div id="app">
    <BlurDetector :threshold="130" @result="blur = $event"/>
    <div v-if="blur != undefined" :style="`color: ${blur ? 'red': 'green'}`">
      Cette image {{blur ? "est" : "n'est pas"}} floue.
    </div>
  </div>
</template>

<script>
import BlurDetector from './components/BlurDetector.vue';

export default {
  name: 'App',
  data: () => ({
    blur: undefined,
  }),
  components: {
    BlurDetector,
  },
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
