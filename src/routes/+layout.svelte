<script lang="ts">
  import "@fontsource/fira-sans";
  import "@fontsource/fira-code";
  import "$lib/scss/global.scss";
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="title">
  <a href="/">fbrage</a>
</div>
<main>
  {@render children?.()}
</main>

<style lang="scss">
  .title {
    display: flex;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
  }
</style>
