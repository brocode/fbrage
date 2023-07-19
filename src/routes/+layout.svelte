<script lang="ts">
  import "@fontsource/fira-sans";
</script>

<ul>
  <li>
    <a href="/private-key">Private key management</a>
  </li>
  <li>
    <a href="/test">Test</a>
  </li>
</ul>

<div>
  <slot />
</div>

<style>
  :global(body) {
    font-family: "Fira Sans", sans-serif;
  }
</style>
