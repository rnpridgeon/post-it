<!-- https://v3.vuejs.org/api/sfc-spec.html#intro -->
<template>
  <div class="bg-gray-200">
    <h1 class="5xl">Message Board</h1>
    <!-- https://v3.vuejs.org/api/directives.html#v-for -->
    <div class="col-span-2 h-24 content-end">
      <SubmissionForm @on-msg-create="fetchMessages" />
    </div>
    <div class="row-span-2 col-span2">
      <Message v-for="(msg, index) in latestMsgs" :key="index" :msg="msg" />
    </div>
  </div>
</template>

<!-- https://v3.vuejs.org/api/sfc-spec.html#pre-processors -->
<script lang="ts">
import { Options, Vue } from "vue-class-component";

import Message from "@/components/Message.vue";
import SubmissionForm from "@/components/SubmissionForm.vue";

import PostItService from "@/service/PostItService";

@Options({
  components: {
    Message,
    SubmissionForm,
  },
  // https://v3.vuejs.org/api/options-data.html#data-2
  data() {
    return {
      msgs: null,
    };
  },
  computed: {
    latestMsgs: function () {
      return this.msgs.reverse();
    },
  },
  // https://v3.vuejs.org/api/options-lifecycle-hooks.html#created
  created() {
    this.fetchMessages();
  },
  // https://v3.vuejs.org/api/options-data.html#methods
  methods: {
    fetchMessages() {
      PostItService.listMessage()
        .then((response: any) => {
          this.msgs = response.data;
        })
        .catch((error: Error) => {
          console.log(error);
        });
    },
  },
})
export default class MessageBoard extends Vue {
  msg!: string;
}
</script>

<!-- https://v3.vuejs.org/api/sfc-spec.html#style -->
<style scoped></style>
