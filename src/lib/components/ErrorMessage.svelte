<script lang="ts">
  export let error: string;
</script>

<article class="error-message">
  {error}
</article>

<style lang="scss">
  .error-message {
    border: var(--border-width) solid var(--form-element-invalid-border-color);
  }
</style>
