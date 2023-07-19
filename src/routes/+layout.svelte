<script lang="ts">
  import "@fontsource/fira-sans";
  import "./global.scss";
</script>

<div class="title">
  <a href="/">fbrage</a>
</div>
<div class="content">
  <slot />
</div>

<style lang="scss">
  .title {
    display: flex;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
  }
  .content {
    padding: 10px;
  }
</style>
