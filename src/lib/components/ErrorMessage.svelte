<script lang="ts">
  interface Props {
    error: string;
  }

  let { error }: Props = $props();
</script>

<article class="error-message">
  {error}
</article>

<style lang="scss">
  .error-message {
    border: var(--border-width) solid var(--form-element-invalid-border-color);
  }
</style>
