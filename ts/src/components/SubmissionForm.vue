<template>
  <!-- <div class="ml-8"> -->
    <!-- <form @submit.prevent="onSubmit">
        <textarea class="bottom" v-model="msg_data"></textarea>
        <br>
        <button type="submit">submit message</button>
    </form> -->
    <div class="ml-5" >
  	<form class="m-6 flex space-x-1" @submit.prevent="onSubmit">
    	<textarea class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" v-model="msg_data" placeholder="new message"  />
		  <button class="px-8 rounded-r-lg bg-yellow-400 text-gray-800 font-bold p-4 uppercase border-yellow-500 border-t border-b border-r">Submit</button>
	  </form>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import PostItService from "@/service/PostItService";

@Options({
  data() {
    return {
      msg_data: "",
    };
  },
  methods: {
    onSubmit() {
      let submission =
        PostItService.postMessage(this.msg_data)
          .then((response: any) => {
            this.msgs = response.data;
          })
          .catch((error: Error) => {
            console.log(error);
          });
      this.$emit("on-msg-create", submission);
      this.msg_data = "";
    },
  },
})
export default class SubmissionForm extends Vue {}
</script>
<style scoped></style>
