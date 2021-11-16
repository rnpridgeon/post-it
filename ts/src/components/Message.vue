<template>
  <div class="content-center grid grid-flow-row auto-rows-max p-2 h-18">
    <div class ="bg-white rounded-l-lg shadow-lg ">
      <p class="ml-8"> {{ msg }} </p>
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    msg: String,
  },
})
export default class Message extends Vue {
  msg!: string;
}

</script>

<style scoped>
</style>