<script lang="ts">
  export let message: string;
</script>

<article>
  <h2>Plaintext</h2>
  <div class="message">{message}</div>
</article>

<style lang="scss">
  .message {
    white-space: pre-wrap;
  }
</style>
