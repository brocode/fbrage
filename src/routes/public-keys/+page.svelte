<script lang="ts">
  import ImportPublicKeys from "$lib/components/ImportPublicKeys.svelte";
  import { publicKeyStore } from "$lib/public_key_store";
</script>

<h1>Coole public keys</h1>

<p>Currently only the import of symbiolab age public keys is supported. Manual key management pending.</p>
<p>
  <strong>Beware: An import replaces all keys with the imported ones.</strong>
</p>

<ImportPublicKeys />

{#each Object.keys($publicKeyStore) as recipientName}
  <article>
    <header>{recipientName}</header>
    {#each $publicKeyStore[recipientName] as key}
      <pre>{key}</pre>
    {/each}
  </article>
{/each}
