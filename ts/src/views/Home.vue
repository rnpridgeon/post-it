<template>
  <div class="grid grid-rows-3 grid-flow-col">
    <div class="row-span-3 bg-white"></div>
    <MessageBoard />
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import MessageBoard from "@/components/MessageBoard.vue"; // @ is an alias to /src

@Options({
  components: {
    MessageBoard,
  },
})
export default class Home extends Vue {}
</script>
