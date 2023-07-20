<script lang="ts">
  import DecryptForm from "$lib/components/DecryptForm.svelte";
  import { privateKeyStore } from "$lib/private_key_store";
</script>

<p>Available private keys: {Object.keys($privateKeyStore).length}</p>

<DecryptForm privateKeyStore={$privateKeyStore} />
