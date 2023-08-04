<script lang="ts">
  import CreateRecipientForm from "$lib/components/CreateRecipientForm.svelte";
  import ImportPublicKeys from "$lib/components/ImportPublicKeys.svelte";
  import { publicKeyStore } from "$lib/public_key_store";
</script>

<h1>Existing public keys</h1>

{#each Object.keys($publicKeyStore) as recipientName}
  <article>
    <header>{recipientName}</header>
    {#each $publicKeyStore[recipientName] as key}
      <pre>{key}</pre>
    {/each}
  </article>
{/each}

<h1>Recipients</h1>

<CreateRecipientForm />

<ImportPublicKeys />
