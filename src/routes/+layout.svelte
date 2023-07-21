<script lang="ts">
  import "@fontsource/fira-sans";
  import "@fontsource/fira-code";
  import "./global.scss";
</script>

<div class="title">
  <a href="/">fbrage</a>
</div>
<main>
  <slot />
</main>

<footer>
  <img src="/logo/logo_500x500.webp" height="200" width="200" alt="" />
</footer>

<style lang="scss">
  .title {
    display: flex;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
  }
  footer {
    display: flex;
    justify-content: center;
    padding: 0;
  }
</style>
