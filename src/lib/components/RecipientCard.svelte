<script lang="ts">
  import { publicKeyStore } from "$lib/public_key_store";
  export let recipientName: string;
</script>

<article>
  <header>{recipientName}</header>
  {#each $publicKeyStore[recipientName] as key}
    <pre>{key}</pre>
  {/each}
</article>
