<script lang="ts">
  import CreateRecipientForm from "$lib/components/CreateRecipientForm.svelte";
  import ImportPublicKeys from "$lib/components/ImportPublicKeys.svelte";
  import RecipientCard from "$lib/components/RecipientCard.svelte";
  import { sortedKeys } from "$lib/public_key_store";
</script>

<h1>Existing public keys</h1>

{#each $sortedKeys as recipientName}
  <RecipientCard {recipientName} />
{/each}

<h1>Recipients</h1>

<CreateRecipientForm />

<ImportPublicKeys />
