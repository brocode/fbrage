<script lang="ts">
  import "@fontsource/fira-sans";
  import "./global.scss";
</script>

<div class="title">
  <a href="/">fbrage</a>
</div>
<main>
  <slot />
</main>

<style lang="scss">
  .title {
    display: flex;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
  }
</style>
