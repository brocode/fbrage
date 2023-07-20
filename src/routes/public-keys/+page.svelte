<script lang="ts">
  import ImportPublicKeys from "$lib/components/ImportPublicKeys.svelte";
  import { publicKeyStore } from "$lib/public_key_store";
</script>

<h1>Coole public keys</h1>
<ImportPublicKeys />
<pre>
    {JSON.stringify($publicKeyStore, null, 4)}
</pre>
