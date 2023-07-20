<script lang="ts">
  import EncryptForm from "$lib/components/EncryptForm.svelte";
  import { publicKeyStore } from "$lib/public_key_store";
</script>

<EncryptForm publicKeyStore={$publicKeyStore} />
